use std::{collections::HashMap, error::Error, fmt, fmt::Formatter};

use mime::Mime;
use reqwest::{
    Body,
    Error as ReqwestError,
    multipart::{Form as MultipartForm, Part},
};
use serde::{Serialize, ser};
use serde_json::Error as JsonError;
use tokio::io::AsyncRead;
use tokio_util::codec::{BytesCodec, FramedRead};

pub(crate) enum FormValue {
    Bytes(Vec<u8>),
    Json(Vec<u8>),
    Text(String),
    File {
        name: Option<String>,
        mime_type: Option<Mime>,
        reader: FramedRead<Box<dyn AsyncRead + Send + Sync + Unpin>, BytesCodec>,
    },
}

impl FormValue {
    fn float<T>(value: T) -> Self
    where
        T: zmij::Float,
    {
        Self::Text(zmij::Buffer::new().format(value).to_owned())
    }

    fn integer<T>(value: T) -> Self
    where
        T: itoa::Integer,
    {
        Self::Text(itoa::Buffer::new().format(value).to_owned())
    }
}

impl fmt::Debug for FormValue {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(value) => out.debug_tuple("FormValue::Bytes").field(value).finish(),
            Self::Json(value) => out
                .debug_tuple("FormValue::Json")
                .field(
                    &serde_json::from_slice::<serde_json::Value>(value).unwrap_or_else(|err| {
                        panic!("can not format JSON value: `{}`: {err}", String::from_utf8_lossy(value))
                    }),
                )
                .finish(),
            Self::Text(value) => out.debug_tuple("FormValue::Text").field(value).finish(),
            Self::File { name, mime_type, .. } => out
                .debug_struct("FormValue::File")
                .field("name", name)
                .field("mime_type", mime_type)
                .finish(),
        }
    }
}

impl PartialEq for FormValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bytes(a), Self::Bytes(b)) => a.eq(b),
            (Self::Json(a), Self::Json(b)) => a.eq(b),
            (Self::Text(a), Self::Text(b)) => a.eq(b),
            (
                Self::File {
                    name: a_name,
                    mime_type: a_mime_type,
                    ..
                },
                Self::File {
                    name: b_name,
                    mime_type: b_mime_type,
                    ..
                },
            ) => a_name.eq(b_name) && a_mime_type.eq(b_mime_type),
            _ => false,
        }
    }
}

impl TryFrom<FormValue> for Part {
    type Error = FormError;

    fn try_from(value: FormValue) -> Result<Self, Self::Error> {
        Ok(match value {
            FormValue::Bytes(buf) => Part::bytes(buf),
            FormValue::Json(buf) => Part::bytes(buf),
            FormValue::Text(text) => Part::text(text),
            FormValue::File {
                reader,
                name,
                mime_type,
            } => {
                let body = Body::wrap_stream(reader);
                let mut part = Part::stream(body);
                if let Some(name) = name {
                    part = part.file_name(name);
                }
                if let Some(mime_type) = mime_type {
                    part = part.mime_str(mime_type.as_ref()).map_err(FormError::Mime)?;
                }
                part
            }
        })
    }
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Form {
    fields: HashMap<String, FormValue>,
}

impl Form {
    pub(crate) fn insert_field<N, V>(&mut self, name: N, value: V)
    where
        N: Into<String>,
        V: Into<FormValue>,
    {
        self.fields.insert(name.into(), value.into());
    }

    pub(crate) fn len(&self) -> usize {
        self.fields.len()
    }

    #[cfg(test)]
    pub(crate) fn into_fields(self) -> Vec<(String, FormValue)> {
        let mut result: Vec<(String, FormValue)> = self.fields.into_iter().collect();
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result
    }
}

impl<I, K> From<I> for Form
where
    I: IntoIterator<Item = (K, FormValue)>,
    K: Into<String>,
{
    fn from(fields: I) -> Form {
        let mut form = Form::default();
        for (name, value) in fields {
            form.insert_field(name, value);
        }
        form
    }
}

impl TryFrom<Form> for MultipartForm {
    type Error = FormError;

    fn try_from(value: Form) -> Result<Self, Self::Error> {
        let mut result = MultipartForm::new();
        for (field_name, field_value) in value.fields {
            let field_value = field_value.try_into()?;
            result = result.part(field_name, field_value);
        }
        Ok(result)
    }
}

pub(crate) trait WriteForm {
    type Output: Serialize;

    fn write(self, form: &mut Form) -> Self::Output;
}

/// An error occurred when building multipart form.
#[derive(Debug)]
pub enum FormError {
    /// Failed to serialize a JSON value.
    Json(JsonError),
    /// Failed to set MIME type.
    Mime(ReqwestError),
    /// Can not serialize a struct into form.
    Serialize(FormSerializeError),
}

impl From<FormSerializeError> for FormError {
    fn from(err: FormSerializeError) -> Self {
        Self::Serialize(err)
    }
}

impl From<JsonError> for FormError {
    fn from(err: JsonError) -> Self {
        Self::Json(err)
    }
}

impl Error for FormError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Json(err) => err,
            Self::Mime(err) => err,
            Self::Serialize(err) => err,
        })
    }
}

impl fmt::Display for FormError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Json(err) => write!(out, "can not serialize JSON: {err}"),
            Self::Mime(err) => write!(out, "can not set MIME type: {err}"),
            Self::Serialize(err) => write!(out, "can not serialize struct: {err}"),
        }
    }
}

/// An error occurred when build a form from a struct.
#[derive(Debug)]
pub enum FormSerializeError {
    /// A custom serialization error.
    Custom(String),
    /// Got an unexpected key type.
    InvalidKey,
    /// Can not serialize a JSON form value.
    Json(JsonError),
    /// Can not serialize a primitive as a top-level value.
    TopLevel,
}

impl From<JsonError> for FormSerializeError {
    fn from(err: JsonError) -> Self {
        Self::Json(err)
    }
}

impl Error for FormSerializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Custom(_) => None,
            Self::InvalidKey => None,
            Self::Json(err) => Some(err),
            Self::TopLevel => None,
        }
    }
}

impl fmt::Display for FormSerializeError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Custom(msg) => write!(out, "{}", msg),
            Self::InvalidKey => write!(out, "can not serialize a form key"),
            Self::Json(err) => write!(out, "{}", err),
            Self::TopLevel => write!(out, "can not serialize a primitive as a top-level value"),
        }
    }
}

impl ser::Error for FormSerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl<'a> ser::Serializer for &'a mut Form {
    type Ok = &'a mut Form;
    type Error = FormSerializeError;

    type SerializeMap = &'a mut Form;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = &'a mut Form;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        v.serialize(self)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(FormSerializeError::TopLevel)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.insert_field(name, FormValue::Text(String::from(variant)));
        Ok(self)
    }
}

impl<'a> ser::SerializeMap for &'a mut Form {
    type Ok = &'a mut Form;
    type Error = FormSerializeError;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let key = serde_json::to_value(key)?;
        if let serde_json::Value::String(key) = key {
            let value = serde_json::to_vec(value)?;
            self.insert_field(key, FormValue::Json(value));
            Ok(())
        } else {
            Err(FormSerializeError::InvalidKey)
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

impl<'a> ser::SerializeStruct for &'a mut Form {
    type Ok = &'a mut Form;
    type Error = FormSerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(FormValueSerializer)?;
        self.insert_field(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

struct FormValueSerializer;

impl ser::Serializer for FormValueSerializer {
    type Ok = FormValue;
    type Error = FormSerializeError;

    type SerializeMap = FormValueJsonSerializer;
    type SerializeSeq = FormValueJsonSerializer;
    type SerializeStruct = FormValueJsonSerializer;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::Text(if v { "true" } else { "false" }.to_owned()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::Bytes(v.to_vec()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut s = String::new();
        s.push(v);
        Ok(FormValue::Text(s))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::float(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::integer(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::integer(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::integer(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::integer(v))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(FormValueJsonSerializer::default())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(FormValueJsonSerializer::default())
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        v.serialize(self)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::Text(String::from(v)))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(FormValueJsonSerializer::default())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::integer(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::integer(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::integer(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::integer(v))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(FormValue::Text(String::from(variant)))
    }
}

#[derive(Default)]
struct FormValueJsonSerializer {
    buf: Vec<u8>,
    is_initialized: bool,
}

impl FormValueJsonSerializer {
    fn write_key<T>(&mut self, key: &T) -> Result<(), FormSerializeError>
    where
        T: ?Sized + Serialize,
    {
        if self.is_initialized {
            self.buf.extend(b",");
        } else {
            self.buf.extend(b"{");
            self.is_initialized = true;
        }
        self.buf.extend(serde_json::to_vec(key)?);
        self.buf.extend(b":");
        Ok(())
    }

    fn write_value<T>(&mut self, value: &T) -> Result<(), FormSerializeError>
    where
        T: ?Sized + Serialize,
    {
        self.buf.extend(serde_json::to_vec(value)?);
        Ok(())
    }

    fn write_item<T>(&mut self, value: &T) -> Result<(), FormSerializeError>
    where
        T: ?Sized + Serialize,
    {
        if self.is_initialized {
            self.buf.extend(b",");
        } else {
            self.buf.extend(b"[");
            self.is_initialized = true;
        }
        self.buf.extend(serde_json::to_vec(value)?);
        Ok(())
    }

    fn finish_map(mut self) -> FormValue {
        if !self.is_initialized {
            self.buf.extend(b"{")
        }
        self.buf.extend(b"}");
        FormValue::Json(self.buf)
    }

    fn finish_seq(mut self) -> FormValue {
        if !self.is_initialized {
            self.buf.extend(b"[")
        }
        self.buf.extend(b"]");
        FormValue::Json(self.buf)
    }
}

impl ser::SerializeMap for FormValueJsonSerializer {
    type Ok = FormValue;
    type Error = FormSerializeError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.write_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.write_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.finish_map())
    }
}

impl ser::SerializeStruct for FormValueJsonSerializer {
    type Ok = FormValue;
    type Error = FormSerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.write_key(key)?;
        self.write_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.finish_map())
    }
}

impl ser::SerializeSeq for FormValueJsonSerializer {
    type Ok = FormValue;
    type Error = FormSerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.write_item(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.finish_seq())
    }
}

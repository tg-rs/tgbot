use std::{collections::HashMap, error::Error, fmt, fmt::Formatter};

use mime::Mime;
use reqwest::{
    multipart::{Form as MultipartForm, Part},
    Body,
    Error as ReqwestError,
};
use tokio::io::AsyncRead;
use tokio_util::codec::{BytesCodec, FramedRead};

pub(crate) enum FormValue {
    Text(String),
    File {
        name: Option<String>,
        mime_type: Option<Mime>,
        reader: FramedRead<Box<dyn AsyncRead + Send + Sync + Unpin>, BytesCodec>,
    },
}

impl fmt::Debug for FormValue {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
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

impl<T> From<T> for FormValue
where
    T: ToString,
{
    fn from(value: T) -> Self {
        FormValue::Text(value.to_string())
    }
}

impl TryFrom<FormValue> for Part {
    type Error = FormError;

    fn try_from(value: FormValue) -> Result<Self, Self::Error> {
        Ok(match value {
            FormValue::Text(text) => Part::text(text),
            FormValue::File {
                reader,
                name,
                mime_type,
            } => {
                let body = Body::wrap_stream(reader);
                let part = Part::stream(body);
                match (name, mime_type) {
                    (Some(name), mime_type) => match mime_type {
                        Some(mime_type) => part
                            .file_name(name)
                            .mime_str(mime_type.as_ref())
                            .map_err(FormError::Mime)?,
                        None => part.file_name(name),
                    },
                    _ => part,
                }
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

    pub(crate) fn remove_field<N>(&mut self, name: N)
    where
        N: Into<String>,
    {
        self.fields.remove(&name.into());
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

/// An error occurred when building multipart form
#[derive(Debug)]
pub enum FormError {
    /// Failed to set MIME type
    Mime(ReqwestError),
}

impl Error for FormError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            FormError::Mime(err) => err,
        })
    }
}

impl fmt::Display for FormError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormError::Mime(err) => write!(out, "can not set MIME type: {}", err),
        }
    }
}

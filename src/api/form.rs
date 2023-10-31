use std::{collections::HashMap, error::Error, fmt};

use reqwest::{
    multipart::{Form as MultipartForm, Part},
    Body,
    Error as ReqwestError,
};

use crate::types::{InputFile, InputFileKind, InputFileReader};

#[derive(Debug, PartialEq)]
pub(crate) enum FormValue {
    Text(String),
    File(InputFile),
}

impl<T> From<T> for FormValue
where
    T: ToString,
{
    fn from(value: T) -> Self {
        FormValue::Text(value.to_string())
    }
}

impl From<InputFile> for FormValue {
    fn from(value: InputFile) -> Self {
        FormValue::File(value)
    }
}

impl TryFrom<FormValue> for Part {
    type Error = FormError;

    fn try_from(value: FormValue) -> Result<Self, Self::Error> {
        Ok(match value {
            FormValue::Text(text) => Part::text(text),
            FormValue::File(file) => match file.kind {
                InputFileKind::Reader(InputFileReader {
                    reader,
                    info: file_info,
                }) => {
                    let body = Body::wrap_stream(reader);
                    let part = Part::stream(body);
                    match file_info {
                        Some(info) => {
                            let file_name = String::from(info.name());
                            let mime_type = info.mime_type();
                            match mime_type {
                                Some(mime_type) => part
                                    .file_name(file_name)
                                    .mime_str(mime_type.as_ref())
                                    .map_err(FormError::Mime)?,
                                None => part.file_name(file_name),
                            }
                        }
                        None => part,
                    }
                }
                InputFileKind::Id(file_id) => Part::text(file_id),
                InputFileKind::Url(url) => Part::text(url),
            },
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

use crate::types::{InputFile, InputFileInfo, InputFileKind, InputFileReader};
use reqwest::{
    multipart::{Form as MultipartForm, Part},
    Body, Error as ReqwestError,
};
use std::{collections::HashMap, error::Error as StdError, fmt, io::Error as IoError};

#[derive(Debug)]
pub(crate) enum FormValue {
    Text(String),
    File(InputFile),
}

impl FormValue {
    #[cfg(test)]
    pub(crate) fn get_text(&self) -> Option<&str> {
        match self {
            FormValue::Text(ref text) => Some(text),
            FormValue::File(_) => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn get_file(&self) -> Option<&InputFile> {
        match self {
            FormValue::Text(_) => None,
            FormValue::File(ref file) => Some(file),
        }
    }

    async fn into_part(self) -> Result<Part, FormError> {
        Ok(match self {
            FormValue::Text(text) => Part::text(text),
            FormValue::File(file) => match file.kind {
                InputFileKind::Reader(InputFileReader {
                    reader,
                    info: file_info,
                }) => {
                    let body = Body::wrap_stream(reader);
                    let part = Part::stream(body);
                    match file_info {
                        Some(InputFileInfo {
                            name: file_name,
                            mime_type: Some(mime_type),
                        }) => part
                            .file_name(file_name)
                            .mime_str(mime_type.as_ref())
                            .map_err(FormError::Mime)?,
                        Some(InputFileInfo {
                            name: file_name,
                            mime_type: None,
                        }) => part.file_name(file_name),
                        None => part,
                    }
                }
                InputFileKind::Id(file_id) => Part::text(file_id),
                InputFileKind::Url(url) => Part::text(url),
            },
        })
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

impl From<InputFile> for FormValue {
    fn from(value: InputFile) -> Self {
        FormValue::File(value)
    }
}

#[derive(Debug)]
pub(crate) struct Form {
    pub(crate) fields: HashMap<String, FormValue>,
}

impl Form {
    pub(crate) fn new() -> Self {
        Self { fields: HashMap::new() }
    }

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

    pub(crate) async fn into_multipart(self) -> Result<MultipartForm, FormError> {
        let mut result = MultipartForm::new();
        for (field_name, field_value) in self.fields {
            let field_value = field_value.into_part().await?;
            result = result.part(field_name, field_value);
        }
        Ok(result)
    }
}

/// An error occurred when building multipart form
#[derive(Debug)]
pub enum FormError {
    /// Failed to read file
    Io(IoError),
    /// Failed to set MIME type
    Mime(ReqwestError),
}

impl From<IoError> for FormError {
    fn from(err: IoError) -> Self {
        FormError::Io(err)
    }
}

impl StdError for FormError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            FormError::Io(err) => err,
            FormError::Mime(err) => err,
        })
    }
}

impl fmt::Display for FormError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormError::Io(err) => write!(out, "can not read file: {}", err),
            FormError::Mime(err) => write!(out, "can not set MIME type: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn form_value() {
        let val = FormValue::from(1);
        assert_eq!(val.get_text().unwrap(), "1");
        assert!(val.get_file().is_none());

        let val = FormValue::from(InputFile::file_id("file-id"));
        assert!(val.get_text().is_none());
        assert!(val.get_file().is_some());
    }

    #[tokio::test]
    async fn form() {
        let mut form = Form::new();
        form.insert_field("id", 1);
        form.insert_field("file-id", InputFile::file_id("file-id"));
        form.insert_field("file-url", InputFile::url("url"));
        form.insert_field("file-reader", InputFile::from(Cursor::new(b"test")));
        form.into_multipart().await.unwrap();
    }
}

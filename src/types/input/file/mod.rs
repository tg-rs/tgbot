use std::{fmt, path::Path};

use mime::{APPLICATION_OCTET_STREAM, Mime};
use tokio::{
    fs,
    io::{AsyncRead, Result as IoResult},
};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::api::FormValue;

#[cfg(test)]
mod tests;

/// Represents a file reader for uploading files.
pub struct InputFileReader {
    file_name: Option<String>,
    mime_type: Option<Mime>,
    reader: FramedRead<Box<dyn AsyncRead + Send + Sync + Unpin>, BytesCodec>,
}

impl InputFileReader {
    /// Creates a new `InputFileReader`.
    ///
    /// # Arguments
    ///
    /// * `value` - The actual reader.
    pub fn new<T>(reader: T) -> Self
    where
        T: AsyncRead + Send + Sync + Unpin + 'static,
    {
        InputFileReader {
            reader: FramedRead::new(Box::new(reader), BytesCodec::new()),
            file_name: None,
            mime_type: None,
        }
    }

    /// Sets a new name of the file
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the file.
    pub fn with_file_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.file_name = Some(value.into());
        self
    }

    /// Returns the name of the file.
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    /// Sets a new MIME type of the file.
    ///
    /// # Arguments
    ///
    /// * `value` - The MIME type of the file.
    pub fn with_mime_type(mut self, value: Mime) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Returns the MIME type of the file.
    pub fn mime_type(&self) -> Option<&Mime> {
        self.mime_type.as_ref()
    }
}

impl<T> From<T> for InputFileReader
where
    T: AsyncRead + Send + Sync + Unpin + 'static,
{
    fn from(reader: T) -> Self {
        InputFileReader::new(reader)
    }
}

impl PartialEq for InputFileReader {
    fn eq(&self, other: &Self) -> bool {
        self.file_name.eq(&other.file_name) && self.mime_type.eq(&other.mime_type)
    }
}

impl fmt::Debug for InputFileReader {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.debug_struct("InputFileReader")
            .field("file_name", &self.file_name)
            .field("mime_type", &self.mime_type)
            .finish()
    }
}

/// Represents a file to upload.
#[derive(Debug, PartialEq)]
pub enum InputFile {
    /// A `file_id` that exists on the Telegram servers.
    Id(String),
    /// An HTTP URL to get a file from the Internet.
    Url(String),
    /// A file to upload using `multipart/form-data`.
    Reader(InputFileReader),
}

impl InputFile {
    /// Creates an `InputFile` from a file ID.
    ///
    /// # Arguments
    ///
    /// * `value` - File ID.
    pub fn file_id<T>(file_id: T) -> Self
    where
        T: Into<String>,
    {
        Self::Id(file_id.into())
    }

    /// Creates an `InputFile` for an HTTP URL.
    ///
    /// # Arguments
    ///
    /// * `value` - HTTP URL to get a file from the Internet.
    pub fn url<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self::Url(url.into())
    }

    /// Creates an `InputFile` from a file path.
    ///
    /// # Arguments
    ///
    /// * `value` - Path to file on a filesystem.
    pub async fn path(path: impl AsRef<Path>) -> IoResult<Self> {
        let path = path.as_ref();
        let file = fs::File::open(path).await?;
        let mut reader = InputFileReader::new(file);
        if let Some(file_name) = path.file_name().and_then(|x| x.to_str()) {
            let mime_type = path
                .extension()
                .and_then(|x| x.to_str())
                .and_then(|x| mime_guess::from_ext(x).first())
                .unwrap_or(APPLICATION_OCTET_STREAM);
            reader = reader.with_file_name(file_name).with_mime_type(mime_type);
        }
        Ok(reader.into())
    }
}

impl<T> From<T> for InputFile
where
    T: Into<InputFileReader>,
{
    fn from(reader: T) -> Self {
        InputFile::Reader(reader.into())
    }
}

impl From<InputFile> for FormValue {
    fn from(value: InputFile) -> Self {
        match value {
            InputFile::Id(value) | InputFile::Url(value) => FormValue::Text(value),
            InputFile::Reader(InputFileReader {
                file_name: name,
                mime_type,
                reader,
            }) => FormValue::File {
                name,
                mime_type,
                reader,
            },
        }
    }
}

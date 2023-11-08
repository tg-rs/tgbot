use std::{fmt, path::Path};

use mime::{Mime, APPLICATION_OCTET_STREAM};
use serde::{Deserialize, Serialize};
use tokio::{
    fs,
    io::{AsyncRead, Result as IoResult},
};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    api::{FormValue, Method, Payload},
    types::Integer,
};

pub use self::{animation::*, audio::*, document::*, photo::*, video::*, video_note::*, voice::*};

#[cfg(test)]
mod tests;

mod animation;
mod audio;
mod document;
mod photo;
mod video;
mod video_note;
mod voice;

/// Represents a file ready to be downloaded
///
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling `GetFile`.
/// Maximum file size to download is 20 MB.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct File {
    /// Identifier
    ///
    /// Can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    /// File path
    ///
    /// Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

impl File {
    /// Creates a new File
    ///
    /// # Arguments
    ///
    /// * file_id - Identifier
    /// * file_unique_id - Unique identifier
    pub fn new<A, B>(file_id: A, file_unique_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            file_size: None,
            file_path: None,
        }
    }

    /// Sets a new file size
    ///
    /// # Arguments
    ///
    /// * value - File size in bytes
    pub fn with_file_size(mut self, value: Integer) -> Self {
        self.file_size = Some(value);
        self
    }

    /// Sets a new file path
    ///
    /// # Arguments
    ///
    /// * value - File path
    pub fn with_file_path<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.file_path = Some(value.into());
        self
    }
}

/// Returns a basic information about a file and prepares it for downloading
///
/// For the moment, bots can download files of up to 20MB in size.
///
/// The file can then be downloaded via the link
/// `https://api.telegram.org/file/bot<token>/<file_path>`,
/// where `<file_path>` is taken from the response.
///
/// It is guaranteed that the link will be valid for at least 1 hour.
///
/// When the link expires, a new one can be requested by calling `GetFile` again.
///
/// Note: This function may not preserve the original file name and MIME type.
/// You should save the file's MIME type and name (if available) when the `File` object is received.
#[derive(Clone, Debug, Serialize)]
pub struct GetFile {
    file_id: String,
}

impl GetFile {
    /// Creates a new GetFile
    ///
    /// # Arguments
    ///
    /// * file_id - File identifier to get info about
    pub fn new<T>(file_id: T) -> Self
    where
        T: Into<String>,
    {
        GetFile {
            file_id: file_id.into(),
        }
    }
}

impl Method for GetFile {
    type Response = File;

    fn into_payload(self) -> Payload {
        Payload::json("getFile", self)
    }
}

/// A file reader for uploading files
pub struct InputFileReader {
    file_name: Option<String>,
    mime_type: Option<Mime>,
    reader: FramedRead<Box<dyn AsyncRead + Send + Sync + Unpin>, BytesCodec>,
}

impl InputFileReader {
    /// Creates a new file reader
    ///
    /// # Arguments
    ///
    /// * value - Actual reader
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

    /// Sets a file name
    ///
    /// # Arguments
    ///
    /// * value - File name
    pub fn with_file_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.file_name = Some(value.into());
        self
    }

    /// Returns a file name
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    /// Sets a MIME type of the file
    ///
    /// # Arguments
    ///
    /// * value - The mime type to set
    pub fn with_mime_type(mut self, value: Mime) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Returns a MIME type
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

/// File to upload
#[derive(Debug, PartialEq)]
pub enum InputFile {
    /// A `file_id` that exists on the Telegram servers
    Id(String),
    /// An HTTP URL to get a file from the Internet
    Url(String),
    /// A file to upload using `multipart/form-data`
    Reader(InputFileReader),
}

impl InputFile {
    /// Creates an InputFile
    ///
    /// # Arguments
    ///
    /// * value - File ID
    pub fn file_id<T>(file_id: T) -> Self
    where
        T: Into<String>,
    {
        Self::Id(file_id.into())
    }

    /// Creates an InputFile
    ///
    /// # Arguments
    ///
    /// * value - HTTP URL to get a file from the Internet
    pub fn url<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self::Url(url.into())
    }

    /// Creates an InputFile
    ///
    /// # Arguments
    ///
    /// * value - Path to file in FS
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

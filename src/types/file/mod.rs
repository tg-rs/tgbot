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

/// File ready to be downloaded
///
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`
/// It is guaranteed that the link will be valid for at least 1 hour
/// When the link expires, a new one can be requested by calling getFile
/// Maximum file size to download is 20 MB
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    /// File path
    /// Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

/// Get basic info about a file and prepare it for downloading
///
/// For the moment, bots can download files of up to 20MB in size
///
/// The file can then be downloaded via the link
/// `https://api.telegram.org/file/bot<token>/<file_path>`,
/// where `<file_path>` is taken from the response
///
/// It is guaranteed that the link will be valid for at least 1 hour
///
/// When the link expires, a new one can be requested by calling getFile again
///
/// Note: This function may not preserve the original file name and MIME type
/// You should save the file's MIME type and name (if available) when the File object is received
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
    pub fn new<S: Into<String>>(file_id: S) -> Self {
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

/// File reader to upload
pub struct InputFileReader {
    file_name: Option<String>,
    mime_type: Option<Mime>,
    reader: FramedRead<Box<dyn AsyncRead + Send + Sync + Unpin>, BytesCodec>,
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

impl InputFileReader {
    /// Creates a new file reader
    pub fn new<R>(reader: R) -> Self
    where
        R: AsyncRead + Send + Sync + Unpin + 'static,
    {
        InputFileReader {
            reader: FramedRead::new(Box::new(reader), BytesCodec::new()),
            file_name: None,
            mime_type: None,
        }
    }

    /// Sets a name of the file
    ///
    /// # Arguments
    ///
    /// * value - The file name to set
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

    /// Sets a mime type of the file
    ///
    /// # Arguments
    ///
    /// * value - The mime type to set
    pub fn with_mime_type(mut self, value: Mime) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Returns a mime type
    pub fn mime_type(&self) -> Option<&Mime> {
        self.mime_type.as_ref()
    }
}

impl<R> From<R> for InputFileReader
where
    R: AsyncRead + Send + Sync + Unpin + 'static,
{
    fn from(reader: R) -> Self {
        InputFileReader::new(reader)
    }
}

/// File to upload
#[derive(Debug, PartialEq)]
pub struct InputFile {
    pub(crate) kind: InputFileKind,
}

impl InputFile {
    /// Send a file_id that exists on the Telegram servers
    pub fn file_id<S: Into<String>>(file_id: S) -> Self {
        Self {
            kind: InputFileKind::Id(file_id.into()),
        }
    }

    /// Send an HTTP URL to get a file from the Internet
    ///
    /// Telegram will download a file from that URL
    pub fn url<S: Into<String>>(url: S) -> Self {
        Self {
            kind: InputFileKind::Url(url.into()),
        }
    }

    /// Path to file in FS (will be uploaded using multipart/form-data)
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
        Ok(Self {
            kind: InputFileKind::Reader(reader),
        })
    }

    /// A reader (file will be uploaded using multipart/form-data)
    pub fn reader<R: Into<InputFileReader>>(reader: R) -> Self {
        Self {
            kind: InputFileKind::Reader(reader.into()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum InputFileKind {
    Id(String),
    Url(String),
    Reader(InputFileReader),
}

impl From<InputFileReader> for InputFile {
    fn from(reader: InputFileReader) -> Self {
        Self::reader(reader)
    }
}

impl<R> From<R> for InputFile
where
    R: AsyncRead + Send + Sync + Unpin + 'static,
{
    fn from(reader: R) -> Self {
        InputFile::reader(InputFileReader::new(reader))
    }
}

impl From<InputFile> for FormValue {
    fn from(value: InputFile) -> Self {
        match value.kind {
            InputFileKind::Id(value) | InputFileKind::Url(value) => FormValue::Text(value),
            InputFileKind::Reader(InputFileReader {
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

use std::{fmt, path::Path};

use mime::{Mime, APPLICATION_OCTET_STREAM};
use serde::{Deserialize, Serialize};
use tokio::{
    fs,
    io::{AsyncRead, Result as IoResult},
};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{method::Method, request::Request, types::Integer};

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

    fn into_request(self) -> Request {
        Request::json("getFile", self)
    }
}

/// Information about a file for reader
#[derive(Clone, Debug, PartialEq)]
pub struct InputFileInfo {
    name: String,
    mime_type: Option<Mime>,
}

impl InputFileInfo {
    /// Creates a new info object with given file name
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            mime_type: None,
        }
    }

    /// Sets mime type of a file
    pub fn with_mime_type(mut self, mime_type: Mime) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    /// Returns a file name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a mime type of a file
    pub fn mime_type(&self) -> Option<&Mime> {
        self.mime_type.as_ref()
    }
}

impl From<&str> for InputFileInfo {
    fn from(name: &str) -> Self {
        InputFileInfo::new(name)
    }
}

impl From<(&str, Mime)> for InputFileInfo {
    fn from((name, mime_type): (&str, Mime)) -> Self {
        InputFileInfo::new(name).with_mime_type(mime_type)
    }
}

impl From<String> for InputFileInfo {
    fn from(name: String) -> Self {
        InputFileInfo::new(name)
    }
}

impl From<(String, Mime)> for InputFileInfo {
    fn from((name, mime_type): (String, Mime)) -> Self {
        InputFileInfo::new(name).with_mime_type(mime_type)
    }
}

/// File reader to upload
pub struct InputFileReader {
    pub(crate) info: Option<InputFileInfo>,
    pub(crate) reader: FramedRead<Box<dyn AsyncRead + Send + Sync + Unpin>, BytesCodec>,
}

impl PartialEq for InputFileReader {
    fn eq(&self, other: &Self) -> bool {
        self.info.eq(&other.info)
    }
}

impl fmt::Debug for InputFileReader {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "InputFileReader(reader: ..., info: {:?})", self.info)
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
            info: None,
        }
    }

    /// Sets a file info
    pub fn info<I: Into<InputFileInfo>>(mut self, info: I) -> Self {
        self.info = Some(info.into());
        self
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
            reader = reader.info(InputFileInfo::new(file_name).with_mime_type(mime_type));
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

use crate::types::primitive::Integer;
use mime::{Mime, APPLICATION_OCTET_STREAM};
use serde::Deserialize;
use std::{fmt, path::Path};
use tokio::{
    fs,
    io::{AsyncRead, Result as IoResult},
};
use tokio_util::codec::{BytesCodec, FramedRead};

/// File ready to be downloaded
///
/// The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>
/// It is guaranteed that the link will be valid for at least 1 hour
/// When the link expires, a new one can be requested by calling getFile
/// Maximum file size to download is 20 MB
#[derive(Clone, Debug, Deserialize)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size, if known
    pub file_size: Option<Integer>,
    /// File path
    /// Use https://api.telegram.org/file/bot<token>/<file_path> to get the file
    pub file_path: Option<String>,
}

/// Information about a file for reader
#[derive(Clone, Debug)]
pub struct InputFileInfo {
    pub(crate) name: String,
    pub(crate) mime_type: Option<Mime>,
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
    pub fn mime_type(mut self, mime_type: Mime) -> Self {
        self.mime_type = Some(mime_type);
        self
    }
}

impl From<&str> for InputFileInfo {
    fn from(name: &str) -> Self {
        InputFileInfo::new(name)
    }
}

impl From<(&str, Mime)> for InputFileInfo {
    fn from((name, mime_type): (&str, Mime)) -> Self {
        InputFileInfo::new(name).mime_type(mime_type)
    }
}

impl From<String> for InputFileInfo {
    fn from(name: String) -> Self {
        InputFileInfo::new(name)
    }
}

impl From<(String, Mime)> for InputFileInfo {
    fn from((name, mime_type): (String, Mime)) -> Self {
        InputFileInfo::new(name).mime_type(mime_type)
    }
}

/// File reader to upload
pub struct InputFileReader {
    pub(crate) info: Option<InputFileInfo>,
    pub(crate) reader: FramedRead<Box<dyn AsyncRead + Send + Sync + Unpin>, BytesCodec>,
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
#[derive(Debug)]
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
            reader = reader.info(InputFileInfo::new(file_name).mime_type(mime_type));
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

pub(crate) enum InputFileKind {
    Id(String),
    Url(String),
    Reader(InputFileReader),
}

impl fmt::Debug for InputFileKind {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputFileKind::Id(ref s) => write!(out, "InputFileKind::Id({:?})", s),
            InputFileKind::Url(ref s) => write!(out, "InputFileKind::Url({:?})", s),
            InputFileKind::Reader(ref r) => write!(out, "InputFileKind::Reader({:?})", r),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn deserialize_file_full() {
        let data: File = serde_json::from_value(serde_json::json!({
            "file_id": "id",
            "file_unique_id": "unique-id",
            "file_size": 123,
            "file_path": "path"
        }))
        .unwrap();
        assert_eq!(data.file_id, "id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert_eq!(data.file_size.unwrap(), 123);
        assert_eq!(data.file_path.unwrap(), "path");
    }

    #[test]
    fn deserialize_file_partial() {
        let data: File = serde_json::from_value(serde_json::json!({
            "file_id": "id",
            "file_unique_id": "unique-id"
        }))
        .unwrap();
        assert_eq!(data.file_id, "id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert!(data.file_size.is_none());
        assert!(data.file_path.is_none());
    }

    #[tokio::test]
    async fn input_file() {
        let id = InputFile::file_id("file-id");
        assert_eq!(format!("{:?}", id.kind), r#"InputFileKind::Id("file-id")"#);
        let url = InputFile::url("http://example.com/archive.zip");
        assert_eq!(
            format!("{:?}", url.kind),
            r#"InputFileKind::Url("http://example.com/archive.zip")"#
        );
        // NOTE: you must be sure that file exists in current working directory (usually it exists)
        // otherwise test will fail
        let path = InputFile::path("LICENSE").await.unwrap();
        assert!(format!("{:?}", path.kind).starts_with("InputFileKind::Reader("),);

        let reader = InputFileReader::from(Cursor::new(b"data")).info(("name", mime::TEXT_PLAIN));
        let reader = InputFile::from(reader);
        assert!(format!("{:?}", reader.kind).starts_with("InputFileKind::Reader("));

        let reader = InputFile::from(Cursor::new(b"data"));
        assert!(format!("{:?}", reader.kind).starts_with("InputFileKind::Reader("));
    }

    #[test]
    fn input_file_info() {
        let info = InputFileInfo::from("name");
        assert_eq!(info.name, "name");
        assert!(info.mime_type.is_none());

        let info = InputFileInfo::from(("name", mime::TEXT_PLAIN));
        assert_eq!(info.name, "name");
        assert_eq!(info.mime_type.unwrap(), mime::TEXT_PLAIN);

        let info = InputFileInfo::from(String::from("name"));
        assert_eq!(info.name, "name");
        assert!(info.mime_type.is_none());

        let info = InputFileInfo::from((String::from("name"), mime::TEXT_PLAIN));
        assert_eq!(info.name, "name");
        assert_eq!(info.mime_type.unwrap(), mime::TEXT_PLAIN);
    }
}

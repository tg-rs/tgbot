use serde::{Deserialize, Serialize};

pub use self::{animation::*, audio::*, document::*, photo::*, video::*, video_note::*, voice::*};
use crate::{
    api::{Method, Payload, PayloadError},
    types::Integer,
};

mod animation;
mod audio;
mod document;
mod photo;
mod video;
mod video_note;
mod voice;

/// Represents a file ready to be downloaded.
///
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling [`GetFile`].
/// Maximum file size to download is 20 MB.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct File {
    /// Identifier of the file.
    ///
    /// Can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier of the file.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size in bytes.
    pub file_size: Option<Integer>,
    /// File path.
    ///
    /// Use [`crate::api::Client::download_file`] to get the file.
    pub file_path: Option<String>,
}

/// Returns basic information about a file and prepares it for downloading.
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
/// # Notes
///
/// This function may not preserve the original file name and MIME type.
/// You should save the file's MIME type and name (if available) when the `File` object is received.
#[derive(Clone, Debug, Serialize)]
pub struct GetFile {
    file_id: String,
}

impl GetFile {
    /// Creates a new `GetFile`.
    ///
    /// # Arguments
    ///
    /// * `file_id` - File identifier to get info about.
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

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("getFile", self)
    }
}

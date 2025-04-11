use serde::{Deserialize, Serialize};

pub use self::{animation::*, audio::*, document::*, photo::*, video::*, video_note::*, voice::*};
use crate::{
    api::{Method, Payload},
    types::Integer,
};

#[cfg(test)]
mod tests;

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

impl File {
    /// Creates a new `File`.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Identifier of the file.
    /// * `file_unique_id` - Unique identifier of the file.
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

    /// Sets a new size of the file.
    ///
    /// # Arguments
    ///
    /// * `value` - The size of the file in bytes.
    pub fn with_file_size(mut self, value: Integer) -> Self {
        self.file_size = Some(value);
        self
    }

    /// Sets a new path of the file.
    ///
    /// # Arguments
    ///
    /// * `value` - The path of the file.
    pub fn with_file_path<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.file_path = Some(value.into());
        self
    }
}

/// Returns a basic information about a file and prepares it for downloading.
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

    fn into_payload(self) -> Payload {
        Payload::json("getFile", self)
    }
}

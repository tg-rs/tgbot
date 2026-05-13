use serde::{Deserialize, Serialize};

use crate::types::{Integer, PhotoSize};

/// Represents a live photo.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct LivePhoto {
    /// Duration of the video in seconds as defined by the sender.
    duration: Integer,
    /// Identifier for the video file which can be used to download or reuse the file.
    file_id: String,
    /// Unique identifier for the video file which is supposed to be the same over time and for different bots.
    ///
    /// Can't be used to download or reuse the file.
    file_unique_id: String,
    /// Video height as defined by the sender.
    height: Integer,
    /// Video width as defined by the sender.
    width: Integer,
    /// File size in bytes.
    file_size: Option<Integer>,
    /// MIME type of the file as defined by the sender.
    mime_type: Option<String>,
    /// Available sizes of the corresponding static photo.
    photo: Option<Vec<PhotoSize>>,
}

impl LivePhoto {
    /// Creates a new `LivePhoto`.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the video in seconds.
    /// * `file_id` - Identifier for the video file which can be used to download or reuse the file.
    /// * `file_unique_id` - Unique identifier for the video file which is supposed to be the same over time for different bots.
    /// * `height` - Video height.
    /// * `width` -> Video width.
    pub fn new<A, B>(duration: Integer, file_id: A, file_unique_id: B, height: Integer, width: Integer) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            duration,
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            height,
            width,
            file_size: None,
            mime_type: None,
            photo: None,
        }
    }

    /// Sets a new file size.
    ///
    /// # Arguments
    ///
    /// * `value` - File size in bytes.
    pub fn with_file_size(mut self, value: Integer) -> Self {
        self.file_size = Some(value);
        self
    }

    /// Sets a new MIME type.
    ///
    /// # Arguments
    ///
    /// * `value` - MIME type of the file as defined by the sender.
    pub fn with_mime_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.mime_type = Some(value.into());
        self
    }

    /// Sets a new photo.
    ///
    /// # Arguments
    ///
    /// * `value` - Available sizes of the corresponding static photo.
    pub fn with_photo<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PhotoSize>,
    {
        self.photo = Some(value.into_iter().collect());
        self
    }
}

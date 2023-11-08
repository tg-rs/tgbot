use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{ChatId, InputFile, Integer, Message, PhotoSize, ReplyMarkup, ReplyMarkupError},
};

#[cfg(test)]
mod tests;

/// Represents a video message
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct VideoNote {
    /// Duration in seconds
    pub duration: Integer,
    /// Identifier
    ///
    /// Can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Width and height (diameter)
    pub length: Integer,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    /// Thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

impl VideoNote {
    /// Creates a new VideoNote
    ///
    /// # Arguments
    ///
    /// * duration - Duration in seconds
    /// * file_id - Identifier
    /// * file_unique_id - Unique identifier
    /// * length - Width and height (diameter)
    pub fn new<A, B>(duration: Integer, file_id: A, file_unique_id: B, length: Integer) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            duration,
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            length,
            file_size: None,
            thumbnail: None,
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

    /// Sets a new thumbnail
    ///
    /// # Arguments
    ///
    /// * value - Thumbnail
    pub fn with_thumbnail(mut self, value: PhotoSize) -> Self {
        self.thumbnail = Some(value);
        self
    }
}

/// Send a video message
///
/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long
#[derive(Debug)]
pub struct SendVideoNote {
    form: Form,
}

impl SendVideoNote {
    /// Creates a new SendVideoNote
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier of the target chat
    /// * video_note - Video note to send
    pub fn new<A, B>(chat_id: A, video_note: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<InputFile>,
    {
        SendVideoNote {
            form: Form::from([
                ("chat_id", chat_id.into().into()),
                ("video_note", video_note.into().into()),
            ]),
        }
    }

    /// Sets a new value for the `allow_sending_without_reply` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the message should be sent even
    ///           if the specified replied-to message is not found
    pub fn with_allow_sending_without_reply(mut self, value: bool) -> Self {
        self.form.insert_field("allow_sending_without_reply", value.to_string());
        self
    }

    /// Sets a new value for the `disable_notification` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether to send the message silently;
    ///           a user will receive a notification without sound
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Sets a new duration
    ///
    /// # Arguments
    ///
    /// * value - Duration in seconds
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.form.insert_field("duration", value);
        self
    }

    /// Sets a new length
    ///
    /// # Arguments
    ///
    /// * value - Video width and height, i.e. diameter of the video message
    pub fn with_length(mut self, value: Integer) -> Self {
        self.form.insert_field("length", value);
        self
    }

    /// Sets a new message thread ID
    ///
    /// # Arguments
    ///
    /// * value - Unique identifier of the target message thread (topic) of the forum;
    ///           for forum supergroups only
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }

    /// Sets a new value for the `protect_content` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether to protect the contents of the sent message from forwarding and saving
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value.to_string());
        self
    }

    /// Sets a new reply markup
    ///
    /// # Arguments
    ///
    /// * value - Markup
    pub fn with_reply_markup<R: Into<ReplyMarkup>>(mut self, value: R) -> Result<Self, ReplyMarkupError> {
        let value = value.into();
        self.form.insert_field("reply_markup", value.serialize()?);
        Ok(self)
    }

    /// Sets a new message ID for a reply
    ///
    /// # Arguments
    ///
    /// * value - ID of the original message
    pub fn with_reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.insert_field("reply_to_message_id", value);
        self
    }

    /// Sets a new thumbnail
    ///
    /// # Arguments
    ///
    /// * value - Thumbnail
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 320.
    /// Ignored if the file is not uploaded using `multipart/form-data`.
    /// Thumbnails can’t be reused and can be only uploaded as a new file.
    pub fn with_thumbnail<V>(mut self, value: V) -> Result<Self, SendVideoNoteError>
    where
        V: Into<InputFile>,
    {
        let value = value.into();
        if matches!(value, InputFile::Id(_)) {
            return Err(SendVideoNoteError::InvalidThumbnail);
        }
        self.form.insert_field("thumbnail", value);
        Ok(self)
    }
}

impl Method for SendVideoNote {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendVideoNote", self.form)
    }
}

/// An error when sending a video note
#[derive(Debug)]
pub enum SendVideoNoteError {
    /// Thumbnails can not be reused
    InvalidThumbnail,
}

impl fmt::Display for SendVideoNoteError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidThumbnail => write!(out, "thumbnails can’t be reused and can be only uploaded as a new file"),
        }
    }
}

impl Error for SendVideoNoteError {}

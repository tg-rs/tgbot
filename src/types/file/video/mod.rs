use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{
        ChatId,
        InputFile,
        Integer,
        Message,
        ParseMode,
        PhotoSize,
        ReplyMarkup,
        ReplyMarkupError,
        ReplyParameters,
        ReplyParametersError,
        TextEntities,
        TextEntity,
        TextEntityError,
    },
};

#[cfg(test)]
mod tests;

/// Represents a video file.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Video {
    /// Duration in seconds as defined by sender.
    pub duration: Integer,
    /// Identifier of the file.
    ///
    /// Can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier of the file.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Height as defined by sender.
    pub height: Integer,
    /// Width as defined by sender.
    pub width: Integer,
    /// Available sizes of the cover of the video in the message.
    pub cover: Option<Vec<PhotoSize>>,
    /// Original filename as defined by sender.
    pub file_name: Option<String>,
    /// File size in bytes.
    pub file_size: Option<Integer>,
    /// MIME type as defined by sender.
    pub mime_type: Option<String>,
    /// Timestamp in seconds from which the video will play in the message.
    pub start_timestamp: Option<Integer>,
    /// Thumbnail.
    pub thumbnail: Option<PhotoSize>,
}

impl Video {
    /// Creates a new `Video`.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the video in seconds.
    /// * `file_id` - Identifier of the file.
    /// * `file_unique_id` - Unique identifier of the file.
    /// * `height` - Height of the video.
    /// * `width` - Width of the video.
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
            cover: None,
            file_name: None,
            file_size: None,
            mime_type: None,
            start_timestamp: None,
            thumbnail: None,
        }
    }

    /// Sets a new cover.
    ///
    /// # Arguments
    ///
    /// * `value` - Available sizes of the cover of the video in the message.
    pub fn with_cover<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PhotoSize>,
    {
        self.cover = Some(value.into_iter().collect());
        self
    }

    /// Sets a new name of the file.
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

    /// Sets a new size of the file.
    ///
    /// # Arguments
    ///
    /// * `value` - The size of the file in bytes.
    pub fn with_file_size(mut self, value: Integer) -> Self {
        self.file_size = Some(value);
        self
    }

    /// Sets a new MIME type.
    ///
    /// # Arguments
    ///
    /// * `value` - MIME type.
    pub fn with_mime_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.mime_type = Some(value.into());
        self
    }

    /// Sets a new start timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - Timestamp in seconds from which the video will play in the message.
    pub fn with_start_timestamp(mut self, value: Integer) -> Self {
        self.start_timestamp = Some(value);
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail.
    pub fn with_thumbnail(mut self, value: PhotoSize) -> Self {
        self.thumbnail = Some(value);
        self
    }
}

/// Sends a video file.
///
/// Telegram clients support mp4 videos (other formats may be sent as Document).
/// Bots can currently send video files of up to 50 MB in size,
/// this limit may be changed in the future.
#[derive(Debug)]
pub struct SendVideo {
    form: Form,
}

impl SendVideo {
    /// Creates a new `SendVideo`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `video` - Video to send.
    pub fn new<A, B>(chat_id: A, video: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<InputFile>,
    {
        Self {
            form: Form::from([("chat_id", chat_id.into().into()), ("video", video.into().into())]),
        }
    }

    /// Sets a new value for the `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///   for a fee of 0.1 Telegram Stars per message.
    ///   The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.form.insert_field("allow_paid_broadcast", value);
        self
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("business_connection_id", value.into());
        self
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    ///
    /// May also be used when resending documents by `file_id`.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("caption", value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value: TextEntities = value.into_iter().collect();
        self.form.insert_field("caption_entities", value.serialize()?);
        self.form.remove_field("parse_mode");
        Ok(self)
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.form.insert_field("parse_mode", value);
        self.form.remove_field("caption_entities");
        self
    }

    /// Sets a new cover.
    ///
    /// # Arguments
    ///
    /// * `value` - Cover for the video in the message.
    pub fn with_cover<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        let value = value.into();
        self.form.insert_field("cover", value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration in seconds.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.form.insert_field("duration", value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to cover with a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.form.insert_field("has_spoiler", value);
        self
    }

    /// Sets a new height.
    ///
    /// # Arguments
    ///
    /// * `value` - Height.
    pub fn with_height(mut self, value: Integer) -> Self {
        self.form.insert_field("height", value);
        self
    }

    /// Sets a new message effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect to be added to the message; for private chats only.
    pub fn with_message_effect_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("message_effect_id", value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value.to_string());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Result<Self, ReplyMarkupError>
    where
        T: Into<ReplyMarkup>,
    {
        let value = value.into();
        self.form.insert_field("reply_markup", value.serialize()?);
        Ok(self)
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Result<Self, ReplyParametersError> {
        self.form.insert_field("reply_parameters", value.serialize()?);
        Ok(self)
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.form.insert_field("show_caption_above_media", value);
        self
    }

    /// Sets a new start timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - Start timestamp for the video in the message.
    pub fn with_start_timestamp(mut self, value: Integer) -> Self {
        self.form.insert_field("start_timestamp", value);
        self
    }

    /// Sets a new value for the `supports_streaming` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the uploaded video is suitable for streaming.
    pub fn with_supports_streaming(mut self, value: bool) -> Self {
        self.form.insert_field("supports_streaming", value);
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail.
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 320.
    /// Ignored if the file is not uploaded using `multipart/form-data`.
    /// Thumbnails can’t be reused and can be only uploaded as a new file.
    pub fn with_thumbnail<T>(mut self, value: T) -> Result<Self, SendVideoError>
    where
        T: Into<InputFile>,
    {
        let value = value.into();
        if matches!(value, InputFile::Id(_)) {
            return Err(SendVideoError::InvalidThumbnail);
        }
        self.form.insert_field("thumbnail", value);
        Ok(self)
    }

    /// Sets a new width.
    ///
    /// # Arguments
    ///
    /// * `value` - Width.
    pub fn with_width(mut self, value: Integer) -> Self {
        self.form.insert_field("width", value);
        self
    }
}

impl Method for SendVideo {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendVideo", self.form)
    }
}

/// Represents an error when sending a video.
#[derive(Debug)]
pub enum SendVideoError {
    /// Thumbnails can not be reused.
    InvalidThumbnail,
}

impl fmt::Display for SendVideoError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidThumbnail => write!(out, "thumbnails can’t be reused and can be only uploaded as a new file"),
        }
    }
}

impl Error for SendVideoError {}

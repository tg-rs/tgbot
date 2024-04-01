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

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Animation {
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
    /// Original filename as defined by sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    /// MIME type as defined by sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Thumbnail as defined by sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

impl Animation {
    /// Creates a new `Animation`.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds.
    /// * `file_id` - Identifier of the file.
    /// * `file_unique_id` - Unique identifier of the file.
    /// * `height` - Height.
    /// * `width` - Width.
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
            file_name: None,
            file_size: None,
            mime_type: None,
            thumbnail: None,
        }
    }

    /// Sets a new file name.
    ///
    /// # Arguments
    ///
    /// * `value` - File name.
    pub fn with_file_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.file_name = Some(value.into());
        self
    }

    /// Sets a new file size.
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

/// Sends an animation file (GIF or H.264/MPEG-4 AVC video without sound).
///
/// Bots can currently send animation files of up to 50 MB in size,
/// this limit may be changed in the future.
#[derive(Debug)]
pub struct SendAnimation {
    form: Form,
}

impl SendAnimation {
    /// Creates a new `SendAnimation`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `animation` - Animation to send.
    pub fn new<A, B>(animation: A, chat_id: B) -> Self
    where
        A: Into<InputFile>,
        B: Into<ChatId>,
    {
        Self {
            form: Form::from([
                ("animation", animation.into().into()),
                ("chat_id", chat_id.into().into()),
            ]),
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * value - Unique identifier of the business connection.
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
    /// May also be used when resending animation by `file_id`.
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

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
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

    /// Sets a new value for a `has_spoiler` flag.
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

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value);
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
    pub fn with_thumbnail<T>(mut self, value: T) -> Result<Self, SendAnimationError>
    where
        T: Into<InputFile>,
    {
        let value = value.into();
        if matches!(value, InputFile::Id(_)) {
            return Err(SendAnimationError::InvalidThumbnail);
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

impl Method for SendAnimation {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendAnimation", self.form)
    }
}

/// Represents an error when sending an animation.
#[derive(Debug)]
pub enum SendAnimationError {
    /// Thumbnails can not be reused.
    InvalidThumbnail,
}

impl fmt::Display for SendAnimationError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidThumbnail => write!(out, "thumbnails can’t be reused and can be only uploaded as a new file"),
        }
    }
}

impl Error for SendAnimationError {}

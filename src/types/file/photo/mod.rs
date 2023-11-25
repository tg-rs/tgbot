use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{
        ChatId,
        InputFile,
        Integer,
        Message,
        ParseMode,
        ReplyMarkup,
        ReplyMarkupError,
        TextEntities,
        TextEntity,
        TextEntityError,
    },
};

#[cfg(test)]
mod tests;

/// Represents a size of a photo or a file / sticker thumbnail.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PhotoSize {
    /// Identifier of the file.
    ///
    /// Can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier of the file.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Height of the photo.
    pub height: Integer,
    /// Width of the photo.
    pub width: Integer,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}

impl PhotoSize {
    /// Creates a new `PhotoSize`.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Identifier of the file.
    /// * `file_unique_id` - Unique identifier of the file.
    /// * `height` - Height of the photo.
    /// * `width` - Width of the photo.
    pub fn new<A, B>(file_id: A, file_unique_id: B, height: Integer, width: Integer) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            height,
            width,
            file_size: None,
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
}

/// Sends a photo.
#[derive(Debug)]
pub struct SendPhoto {
    form: Form,
}

impl SendPhoto {
    /// Creates a new `SendPhoto`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `photo` - Photo to send.
    pub fn new<A, B>(chat_id: A, photo: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<InputFile>,
    {
        Self {
            form: Form::from([("chat_id", chat_id.into().into()), ("photo", photo.into().into())]),
        }
    }

    /// Sets a new value for an `allow_sending_without_reply` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the message should be sent even
    ///             if the specified replied-to message is not found.
    pub fn with_allow_sending_without_reply(mut self, value: bool) -> Self {
        self.form.insert_field("allow_sending_without_reply", value.to_string());
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

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value.to_string());
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

    /// Sets a new message ID for a reply.
    ///
    /// # Arguments
    ///
    /// * `value` - ID of the original message.
    pub fn with_reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.insert_field("reply_to_message_id", value.to_string());
        self
    }
}

impl Method for SendPhoto {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendPhoto", self.form)
    }
}

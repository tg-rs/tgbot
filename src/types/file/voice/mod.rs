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

/// Represents a voice file
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Voice {
    /// Duration in seconds as defined by sender
    pub duration: Integer,
    /// Identifier
    ///
    /// Can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    /// MIME type as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl Voice {
    /// Creates a new Voice
    ///
    /// # Arguments
    ///
    /// * duration - Duration in seconds
    /// * file_id - Identifier
    /// * file_unique_id - Unique identifier
    pub fn new<A, B>(duration: Integer, file_id: A, file_unique_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            duration,
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            file_size: None,
            mime_type: None,
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

    /// Sets a new MIME type
    ///
    /// # Arguments
    ///
    /// * value - MIME type
    pub fn with_mime_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.mime_type = Some(value.into());
        self
    }
}

/// Sends a voice message
///
/// Audio must be in an .ogg file encoded with OPUS.
/// Other formats may be sent as Audio or Document.
/// Bots can currently send voice messages of up to 50 MB in size,
/// this limit may be changed in the future.
#[derive(Debug)]
pub struct SendVoice {
    form: Form,
}

impl SendVoice {
    /// Creates a new SendVoice
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier of the target chat
    /// * voice - Audio file to send
    pub fn new<A, B>(chat_id: A, voice: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<InputFile>,
    {
        SendVoice {
            form: Form::from([("chat_id", chat_id.into().into()), ("voice", voice.into().into())]),
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

    /// Sets a new caption
    ///
    /// # Arguments
    ///
    /// * value - (0-1024 characters)
    ///
    /// May also be used when resending documents by `file_id`.
    pub fn with_caption<S: Into<String>>(mut self, value: S) -> Self {
        self.form.insert_field("caption", value.into());
        self
    }

    /// Sets a new caption entities
    ///
    /// # Arguments
    ///
    /// * value - List of special entities that appear in the caption
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value: TextEntities = value.into_iter().collect();
        self.form.insert_field("caption_entities", value.serialize()?);
        self.form.remove_field("parse_mode");
        Ok(self)
    }

    /// Sets a new caption parse mode
    ///
    /// # Arguments
    ///
    /// * value - Parse mode
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.form.insert_field("parse_mode", value);
        self.form.remove_field("caption_entities");
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
}

impl Method for SendVoice {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendVoice", self.form)
    }
}

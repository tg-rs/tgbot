use serde::Deserialize;

use crate::{
    form::Form,
    method::Method,
    request::Request,
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

/// Voice note
#[derive(Clone, Debug, Deserialize)]
pub struct Voice {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: Integer,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}

/// Send audio files, if you want Telegram clients to display the file as a playable voice message
///
/// For this to work, your audio must be in an .ogg file encoded with OPUS
/// (other formats may be sent as Audio or Document)
/// Bots can currently send voice messages of up to 50 MB in size,
/// this limit may be changed in the future
#[derive(Debug)]
pub struct SendVoice {
    form: Form,
}

impl SendVoice {
    /// Creates a new SendVoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * voice - Audio file to send
    pub fn new<C, V>(chat_id: C, voice: V) -> Self
    where
        C: Into<ChatId>,
        V: Into<InputFile>,
    {
        let mut form = Form::new();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("voice", voice.into());
        SendVoice { form }
    }

    /// Voice message caption, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, value: S) -> Self {
        self.form.insert_field("caption", value.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value: TextEntities = value.into_iter().collect();
        self.form.insert_field("caption_entities", value.serialize()?);
        self.form.remove_field("parse_mode");
        Ok(self)
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.form.insert_field("parse_mode", value);
        self.form.remove_field("caption_entities");
        self
    }

    /// Duration of the voice message in seconds
    pub fn duration(mut self, value: Integer) -> Self {
        self.form.insert_field("duration", value);
        self
    }

    // Sends the message silently
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value.to_string());
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.insert_field("reply_to_message_id", value);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, value: bool) -> Self {
        self.form.insert_field("allow_sending_without_reply", value.to_string());
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, value: R) -> Result<Self, ReplyMarkupError> {
        let value = value.into();
        self.form.insert_field("reply_markup", value.serialize()?);
        Ok(self)
    }
}

impl Method for SendVoice {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::form("sendVoice", self.form)
    }
}

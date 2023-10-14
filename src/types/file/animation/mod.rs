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
        TextEntities,
        TextEntity,
        TextEntityError,
    },
};

#[cfg(test)]
mod tests;

/// An animation file (GIF or H.264/MPEG-4 AVC video without sound)
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Animation width as defined by sender
    pub width: Integer,
    /// Animation height as defined by sender
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub duration: Integer,
    /// Animation thumbnail as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}

/// Send animation files (GIF or H.264/MPEG-4 AVC video without sound)
///
/// Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future
#[derive(Debug)]
pub struct SendAnimation {
    form: Form,
}

impl SendAnimation {
    /// Creates a new SendAnimation with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * animation - Animation to send
    pub fn new<C, A>(chat_id: C, animation: A) -> Self
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        let mut form = Form::new();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("animation", animation.into());
        SendAnimation { form }
    }

    /// Sets duration of sent animation in seconds
    pub fn duration(mut self, value: Integer) -> Self {
        self.form.insert_field("duration", value);
        self
    }

    /// Sets animation width
    pub fn width(mut self, value: Integer) -> Self {
        self.form.insert_field("width", value);
        self
    }

    /// Sets animation height
    pub fn height(mut self, value: Integer) -> Self {
        self.form.insert_field("height", value);
        self
    }

    /// Sets thumbnail of the file sent
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 320
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under <file_attach_name>
    pub fn thumb<V>(mut self, value: V) -> Self
    where
        V: Into<InputFile>,
    {
        self.form.insert_field("thumb", value.into());
        self
    }

    /// Sets animation caption
    ///
    /// May also be used when resending animation by file_id
    /// 0-1024 characters
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

    /// Whether to send the message silently
    ///
    /// Users will receive a notification with no sound if enabled
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

impl Method for SendAnimation {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendAnimation", self.form)
    }
}
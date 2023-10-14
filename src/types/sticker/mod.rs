use serde::{Deserialize, Serialize};

use crate::{
    form::Form,
    method::Method,
    request::Request,
    types::{ChatId, File, InputFile, Integer, Message, PhotoSize, ReplyMarkup, ReplyMarkupError},
};

pub use self::{mask::*, set::*};

#[cfg(test)]
mod tests;

mod mask;
mod set;

/// A new sticker to upload
#[derive(Debug)]
pub struct NewSticker {
    kind: NewStickerKind,
}

#[derive(Debug)]
enum NewStickerKind {
    Png(InputFile),
    Tgs(InputFile),
    Video(InputFile),
}

impl NewSticker {
    /// PNG image
    ///
    /// Must be up to 512 kilobytes in size, dimensions must not exceed 512px,
    /// and either width or height must be exactly 512px
    pub fn png<I>(file: I) -> Self
    where
        I: Into<InputFile>,
    {
        Self {
            kind: NewStickerKind::Png(file.into()),
        }
    }

    /// TGS animation
    ///
    /// See <https://core.telegram.org/animated_stickers#technical-requirements>
    /// for technical requirements
    pub fn tgs<I>(file: I) -> Self
    where
        I: Into<InputFile>,
    {
        Self {
            kind: NewStickerKind::Tgs(file.into()),
        }
    }

    /// WEBM video
    ///
    /// See <https://core.telegram.org/stickers#video-sticker-requirements>
    /// for technical requirements
    pub fn video<I>(file: I) -> Self
    where
        I: Into<InputFile>,
    {
        Self {
            kind: NewStickerKind::Video(file.into()),
        }
    }
}

/// Send .webp sticker
#[derive(Debug)]
pub struct SendSticker {
    form: Form,
}

impl SendSticker {
    /// Creates a new SendSticker with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * sticker - Sticker to send
    ///             Pass a file_id as String to send a file that exists on the Telegram servers (recommended),
    ///             pass an HTTP URL as a String for Telegram to get a .webp file from the Internet,
    ///             or upload a new one using multipart/form-data
    pub fn new<C, S>(chat_id: C, sticker: S) -> Self
    where
        C: Into<ChatId>,
        S: Into<InputFile>,
    {
        let mut form = Form::new();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("sticker", sticker.into());
        SendSticker { form }
    }

    /// Sends the message silently
    ///
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

impl Method for SendSticker {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::form("sendSticker", self.form)
    }
}

/// Sticker
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Sticker width
    pub width: Integer,
    /// Sticker height
    pub height: Integer,
    /// Sticker thumbnail in the .webp or .jpg format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    /// True, if the sticker is animated
    pub is_animated: bool,
    /// True, if the sticker is a video sticker
    pub is_video: bool,
}

/// Upload a .png file with a sticker for later use in createNewStickerSet and addStickerToSet methods
#[derive(Debug)]
pub struct UploadStickerFile {
    form: Form,
}

impl UploadStickerFile {
    /// Creates a new UploadStickerFile
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier of sticker file owner
    /// * png_sticker - Png image with the sticker, must be up to 512 kilobytes in size,
    ///                 dimensions must not exceed 512px, and either width or height must be exactly 512px
    pub fn new<P>(user_id: Integer, png_sticker: P) -> Self
    where
        P: Into<InputFile>,
    {
        let mut form = Form::new();
        form.insert_field("user_id", user_id);
        form.insert_field("png_sticker", png_sticker.into());
        UploadStickerFile { form }
    }
}

impl Method for UploadStickerFile {
    type Response = File;

    fn into_request(self) -> Request {
        Request::form("uploadStickerFile", self.form)
    }
}

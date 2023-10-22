use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{ChatId, File, InputFile, Integer, Message, PhotoSize, ReplyMarkup, ReplyMarkupError},
};

pub use self::{input::*, mask::*, set::*};

#[cfg(test)]
mod tests;

mod input;
mod mask;
mod set;

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
    /// Type of the sticker, currently one of “regular”, “mask”, “custom_emoji”
    ///
    /// The type of the sticker is independent from its format,
    /// which is determined by the fields is_animated and is_video.
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    /// Sticker width
    pub width: Integer,
    /// Sticker height
    pub height: Integer,
    /// True, if the sticker is animated
    pub is_animated: bool,
    /// True, if the sticker is a video sticker
    pub is_video: bool,
    /// Sticker thumbnail in the .webp or .jpg format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    /// For premium regular stickers, premium animation for the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,
    /// For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// For custom emoji stickers, unique identifier of the custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// True, if the sticker must be repainted to a text color in messages,
    /// the color of the Telegram Premium badge in emoji status,
    /// white color on chat photos, or another appropriate color in other places
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}

/// Format of stickers in the set
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StickerFormat {
    /// PNG or WEBP
    Static,
    /// TGS
    Animated,
    /// WEBM
    Video,
}

impl AsRef<str> for StickerFormat {
    fn as_ref(&self) -> &str {
        match self {
            Self::Static => "static",
            Self::Animated => "animated",
            Self::Video => "video",
        }
    }
}

/// Type of stickers in the set
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    /// Sticker contains a custom emoji
    CustomEmoji,
    /// Sticker contains a mask
    Mask,
    /// Regular sticker
    Regular,
}

impl AsRef<str> for StickerType {
    fn as_ref(&self) -> &str {
        match self {
            Self::CustomEmoji => "custom_emoji",
            Self::Mask => "mask",
            Self::Regular => "regular",
        }
    }
}

/// Get information about custom emoji stickers by their identifiers
#[derive(Clone, Debug, Serialize)]
pub struct GetCustomEmojiStickers {
    custom_emoji_ids: Vec<String>,
}

impl GetCustomEmojiStickers {
    /// Creates a new GetCustomEmojiStickers
    ///
    /// # Arguments
    ///
    /// * custom_emoji_ids - List of custom emoji identifiers.
    ///                      At most 200 custom emoji identifiers can be specified.
    pub fn new<A, B>(custom_emoji_ids: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        Self {
            custom_emoji_ids: custom_emoji_ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl Method for GetCustomEmojiStickers {
    type Response = Vec<Sticker>;

    fn into_payload(self) -> Payload {
        Payload::json("getCustomEmojiStickers", self)
    }
}

/// Send static .WEBP, animated .TGS, or video .WEBM stickers
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

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, value: bool) -> Self {
        self.form.insert_field("allow_sending_without_reply", value.to_string());
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Emoji associated with the sticker; only for just uploaded stickers
    pub fn emoji<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("emoji", value.into());
        self
    }

    /// Unique identifier for the target message thread (topic) of the forum;
    /// for forum supergroups only
    pub fn message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value.to_string());
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, value: R) -> Result<Self, ReplyMarkupError> {
        let value = value.into();
        self.form.insert_field("reply_markup", value.serialize()?);
        Ok(self)
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.insert_field("reply_to_message_id", value);
        self
    }
}

impl Method for SendSticker {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendSticker", self.form)
    }
}

/// Change the list of emoji assigned to a regular or custom emoji sticker
///
/// The sticker must belong to a sticker set created by the bot
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerEmojiList {
    sticker: String,
    emoji_list: Vec<String>,
}

impl SetStickerEmojiList {
    /// Creates a new SetStickerEmojiList
    ///
    /// * sticker - File identifier of the sticker
    /// * emoji_list - A list of 1-20 emoji associated with the sticker
    pub fn new<A, B, C>(sticker: A, emoji_list: B) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = C>,
        C: Into<String>,
    {
        Self {
            sticker: sticker.into(),
            emoji_list: emoji_list.into_iter().map(Into::into).collect(),
        }
    }
}

impl Method for SetStickerEmojiList {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setStickerEmojiList", self)
    }
}

/// Change search keywords assigned to a regular or custom emoji sticker
///
/// The sticker must belong to a sticker set created by the bot.
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerKeywords {
    sticker: String,
    keywords: Vec<String>,
}

impl SetStickerKeywords {
    /// Creates a new SetStickerKeywords
    ///
    /// * sticker - File identifier of the sticker
    /// * keywords - A list of 0-20 search keywords for the sticker
    ///              with total length of up to 64 characters
    pub fn new<A, B, C>(sticker: A, keywords: B) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = C>,
        C: Into<String>,
    {
        Self {
            sticker: sticker.into(),
            keywords: keywords.into_iter().map(Into::into).collect(),
        }
    }
}

impl Method for SetStickerKeywords {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setStickerKeywords", self)
    }
}

/// Change the mask position of a mask sticker
///
/// The sticker must belong to a sticker set created by the bot.
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerMaskPosition {
    sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<MaskPosition>,
}

impl SetStickerMaskPosition {
    /// Creates a new SetStickerMaskPosition
    ///
    /// * sticker - File identifier of the sticker
    pub fn new<T>(sticker: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            sticker: sticker.into(),
            mask_position: None,
        }
    }

    /// Position where the mask should be placed on faces
    ///
    /// Omit the parameter to remove the mask position.
    pub fn mask_position(mut self, value: MaskPosition) -> Self {
        self.mask_position = Some(value);
        self
    }
}

impl Method for SetStickerMaskPosition {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setStickerMaskPosition", self)
    }
}

/// Upload a file with a sticker for later use in
/// the createNewStickerSet and addStickerToSet methods
/// (the file can be used multiple times).
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
    /// * sticker - A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format.
    /// * sticker_format - Format of the sticker
    pub fn new<T>(user_id: Integer, sticker: T, sticker_format: StickerFormat) -> Self
    where
        T: Into<InputFile>,
    {
        Self {
            form: Form::from([
                ("user_id", user_id.into()),
                ("sticker", sticker.into().into()),
                ("sticker_format", sticker_format.as_ref().into()),
            ]),
        }
    }
}

impl Method for UploadStickerFile {
    type Response = File;

    fn into_payload(self) -> Payload {
        Payload::form("uploadStickerFile", self.form)
    }
}

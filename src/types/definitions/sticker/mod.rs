use serde::{Deserialize, Serialize};

pub use self::{input::*, mask::*, set::*};
use crate::{
    api::{Form, Method, Payload},
    types::{
        ChatId,
        File,
        InputFile,
        Integer,
        Message,
        PhotoSize,
        ReplyMarkup,
        ReplyMarkupError,
        ReplyParameters,
        ReplyParametersError,
        SuggestedPostParameters,
        SuggestedPostParametersError,
    },
};

mod input;
mod mask;
mod set;

/// Represents a sticker.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Sticker {
    /// Identifier of the file.
    ///
    /// Can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier of the file.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Sticker height.
    pub height: Integer,
    /// Indicates whether the sticker is animated.
    pub is_animated: bool,
    /// Indicates whether the sticker is a video sticker.
    pub is_video: bool,
    /// Type of the sticker.
    ///
    /// The type of the sticker is independent from its format,
    /// which is determined by the fields `is_animated` and `is_video`.
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    /// Sticker width.
    pub width: Integer,
    /// For custom emoji stickers, unique identifier of the custom emoji.
    pub custom_emoji_id: Option<String>,
    /// Emoji associated with the sticker.
    pub emoji: Option<String>,
    /// File size in bytes.
    pub file_size: Option<Integer>,
    /// For mask stickers, the position where the mask should be placed.
    pub mask_position: Option<MaskPosition>,
    /// Indicates whether the sticker must be repainted to a text color in messages,
    /// the color of the Telegram Premium badge in emoji status,
    /// white color on chat photos, or another appropriate color in other places.
    pub needs_repainting: Option<bool>,
    /// For premium regular stickers, premium animation for the sticker.
    pub premium_animation: Option<File>,
    /// Name of the sticker set to which the sticker belongs.
    pub set_name: Option<String>,
    /// Sticker thumbnail in the WEBP or JPEG format.
    pub thumbnail: Option<PhotoSize>,
}

impl Sticker {
    /// Creates a new `Sticker`.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Identifier for the file.
    /// * `file_unique_id` - Unique identifier for the file.
    /// * `sticker_type` - Type of the sticker.
    /// * `height` - Sticker height.
    /// * `width` - Sticker width.
    pub fn new<A, B>(file_id: A, file_unique_id: B, sticker_type: StickerType, height: Integer, width: Integer) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            height,
            is_animated: false,
            is_video: false,
            sticker_type,
            width,
            custom_emoji_id: None,
            emoji: None,
            file_size: None,
            mask_position: None,
            needs_repainting: None,
            premium_animation: None,
            set_name: None,
            thumbnail: None,
        }
    }

    /// Sets a new value for the `is_animated` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the sticker is animated.
    pub fn with_is_animated(mut self, value: bool) -> Self {
        self.is_animated = value;
        self
    }

    /// Sets a new value for the `is_video` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the sticker is a video sticker.
    pub fn with_is_video(mut self, value: bool) -> Self {
        self.is_video = value;
        self
    }

    /// Sets a new custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom emoji ID.
    pub fn with_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_emoji_id = Some(value.into());
        self
    }

    /// Sets a new emoji.
    ///
    /// # Arguments
    ///
    /// * `value` - Emoji.
    pub fn with_emoji<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.emoji = Some(value.into());
        self
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

    /// Sets a new mask position.
    ///
    /// # Arguments
    ///
    /// * `value` - Mask position.
    pub fn with_mask_position(mut self, value: MaskPosition) -> Self {
        self.mask_position = Some(value);
        self
    }
    /// Sets a new value for the `needs_repainting` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Value of the flag.
    pub fn with_needs_repainting(mut self, value: bool) -> Self {
        self.needs_repainting = Some(value);
        self
    }

    /// Sets a new premium animation.
    ///
    /// # Arguments
    ///
    /// * `value` - Premium animation.
    pub fn with_premium_animation(mut self, value: File) -> Self {
        self.premium_animation = Some(value);
        self
    }

    /// Sets a new sticker set name.
    ///
    /// # Arguments
    ///
    /// * `value` - Name of a sticker set.
    pub fn with_set_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.set_name = Some(value.into());
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

/// Represents a format of stickers in the set.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StickerFormat {
    /// PNG or WEBP.
    Static,
    /// TGS.
    Animated,
    /// WEBM.
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

/// Represents a type of stickers in the set.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    /// Sticker contains a custom emoji.
    CustomEmoji,
    /// Sticker contains a mask.
    Mask,
    /// Regular sticker.
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

/// Returns information about custom emoji stickers by their identifiers.
#[derive(Clone, Debug, Serialize)]
pub struct GetCustomEmojiStickers {
    custom_emoji_ids: Vec<String>,
}

impl GetCustomEmojiStickers {
    /// Creates a new `GetCustomEmojiStickers`.
    ///
    /// # Arguments
    ///
    /// * `custom_emoji_ids` - List of custom emoji identifiers; at most 200 custom emoji identifiers can be specified.
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

/// Sends a static WEBP, animated TGS, or video WEBM sticker.
#[derive(Debug)]
pub struct SendSticker {
    form: Form,
}

impl SendSticker {
    /// Creates a new `SendSticker`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `sticker` - Sticker to send.
    pub fn new<A, B>(chat_id: A, sticker: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<InputFile>,
    {
        Self {
            form: Form::from([("chat_id", chat_id.into().into()), ("sticker", sticker.into().into())]),
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

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.form.insert_field("direct_messages_topic_id", value);
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

    /// Sets a new emoji.
    ///
    /// # Arguments
    ///
    /// * `value` - Emoji associated with the sticker; only for just uploaded stickers.
    pub fn with_emoji<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("emoji", value.into());
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

    /// Sets a new suggested post parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - An object containing the parameters of the suggested post to send.
    ///
    /// For direct messages chats only.
    ///
    /// If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub fn with_suggested_post_parameters(
        mut self,
        value: &SuggestedPostParameters,
    ) -> Result<Self, SuggestedPostParametersError> {
        let value = serde_json::to_string(value).map_err(SuggestedPostParametersError::Serialize)?;
        self.form.insert_field("suggested_post_parameters", value);
        Ok(self)
    }
}

impl Method for SendSticker {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendSticker", self.form)
    }
}

/// Changes the list of emoji assigned to a regular or custom emoji sticker.
///
/// The sticker must belong to a sticker set created by the bot.
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerEmojiList {
    sticker: String,
    emoji_list: Vec<String>,
}

impl SetStickerEmojiList {
    /// Creates a new `SetStickerEmojiList`.
    ///
    /// * `sticker` - File identifier of the sticker.
    /// * `emoji_list` - A list of 1-20 emoji associated with the sticker.
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

/// Changes search keywords assigned to a regular or custom emoji sticker.
///
/// The sticker must belong to a sticker set created by the bot.
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerKeywords {
    sticker: String,
    keywords: Vec<String>,
}

impl SetStickerKeywords {
    /// Creates a new `SetStickerKeywords`.
    ///
    /// * `sticker` - File identifier of the sticker.
    /// * `keywords` - A list of 0-20 search keywords for the sticker
    ///   with total length of up to 64 characters.
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

/// Changes the mask position of a mask sticker.
///
/// The sticker must belong to a sticker set created by the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerMaskPosition {
    sticker: String,
    mask_position: Option<MaskPosition>,
}

impl SetStickerMaskPosition {
    /// Creates a new `SetStickerMaskPosition`.
    ///
    /// * `sticker` - File identifier of the sticker.
    pub fn new<T>(sticker: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            sticker: sticker.into(),
            mask_position: None,
        }
    }

    /// Sets a new mask position.
    ///
    /// # Arguments
    ///
    /// * `value` - Position where the mask should be placed on faces.
    ///
    /// Omit the parameter to remove the mask position.
    pub fn with_mask_position(mut self, value: MaskPosition) -> Self {
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

/// Uploads a file with a sticker for later use in
/// the [`CreateNewStickerSet`] and [`AddStickerToSet`] methods.
///
/// The file can be used multiple times.
#[derive(Debug)]
pub struct UploadStickerFile {
    form: Form,
}

impl UploadStickerFile {
    /// Creates a new `UploadStickerFile`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier of sticker file owner.
    /// * `sticker` - A file with the sticker in WEBP, PNG, TGS, or WEBM format.
    /// * `sticker_format` - Format of the sticker.
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

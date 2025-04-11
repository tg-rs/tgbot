use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{
        InputFile,
        InputSticker,
        InputStickerError,
        InputStickers,
        Integer,
        PhotoSize,
        Sticker,
        StickerFormat,
        StickerType,
    },
};

#[cfg(test)]
mod tests;

/// Represents a sticker set.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StickerSet {
    /// Name of the sticker set.
    pub name: String,
    /// Type of stickers.
    pub sticker_type: StickerType,
    /// List of stickers.
    pub stickers: Vec<Sticker>,
    /// Title of the sticker set.
    pub title: String,
    /// Sticker set thumbnail in the WEBP or TGS format.
    pub thumbnail: Option<PhotoSize>,
}

impl StickerSet {
    /// Creates a new `StickerSet`.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the sticker set.
    /// * `sticker_type` - Type of stickers.
    /// * `stickers` - List of stickers.
    /// * `title` - Title of the sticker set.
    pub fn new<A, B, C>(name: A, sticker_type: StickerType, stickers: C, title: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: IntoIterator<Item = Sticker>,
    {
        Self {
            name: name.into(),
            sticker_type,
            stickers: stickers.into_iter().collect(),
            title: title.into(),
            thumbnail: None,
        }
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

/// Adds a new sticker to a set created by the bot.
///
/// The format of the added sticker must match the format of the other stickers in the set.
/// Emoji sticker sets can have up to 200 stickers.
/// Animated and video sticker sets can have up to 50 stickers.
/// Static sticker sets can have up to 120 stickers.
#[derive(Debug)]
pub struct AddStickerToSet {
    form: Form,
}

impl AddStickerToSet {
    /// Creates a new `AddStickerToSet`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier of sticker set owner.
    /// * `name` - Sticker set name.
    /// * `sticker` - Sticker file.
    pub fn new<T>(user_id: Integer, name: T, sticker: InputSticker) -> Result<Self, InputStickerError>
    where
        T: Into<String>,
    {
        let mut form: Form = sticker.try_into()?;
        form.insert_field("user_id", user_id);
        form.insert_field("name", name.into());
        Ok(Self { form })
    }
}

impl Method for AddStickerToSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::form("addStickerToSet", self.form)
    }
}

/// Creates a new sticker set owned by a user.
///
/// The bot will be able to edit the created sticker set.
#[derive(Debug)]
pub struct CreateNewStickerSet {
    form: Form,
}

impl CreateNewStickerSet {
    /// Creates a new `CreateNewStickerSet`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier of created sticker set owner.
    /// * `name` - Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals);
    ///   can contain only english letters, digits and underscores;
    ///   must begin with a letter, can't contain consecutive underscores
    ///   and must end in `_by_<bot username>`;
    ///   <bot_username> is case insensitive;
    ///   1-64 characters.
    /// * `title` - Sticker set title; 1-64 characters.
    /// * `stickers` - A list of 1-50 initial stickers to be added to the sticker set.
    pub fn new<A, B>(user_id: Integer, name: A, title: B, stickers: InputStickers) -> Result<Self, InputStickerError>
    where
        A: Into<String>,
        B: Into<String>,
    {
        let mut form: Form = stickers.try_into()?;
        form.insert_field("user_id", user_id);
        form.insert_field("name", name.into());
        form.insert_field("title", title.into());
        Ok(Self { form })
    }

    /// Sets a new value for the `needs_repainting` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether stickers in the sticker set must be repainted to the color
    ///   of text when used in messages, the accent color if used as emoji status,
    ///   white on chat photos, or another appropriate color based on context;
    ///   for custom emoji sticker sets only.
    pub fn with_needs_repainting(mut self, value: bool) -> Self {
        self.form.insert_field("needs_repainting", value);
        self
    }

    /// Sets a new sticker type.
    ///
    /// # Arguments
    ///
    /// * `value` - Type of stickers in the set.
    ///
    /// By default, a regular sticker set is created.
    pub fn with_sticker_type(mut self, value: StickerType) -> Self {
        self.form.insert_field("sticker_type", value.as_ref());
        self
    }
}

impl Method for CreateNewStickerSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::form("createNewStickerSet", self.form)
    }
}

/// Deletes a sticker from a set created by the bot.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteStickerFromSet {
    sticker: String,
}

impl DeleteStickerFromSet {
    /// Creates a new `DeleteStickerFromSet`.
    ///
    /// # Arguments
    ///
    /// * `sticker` - File identifier of the sticker.
    pub fn new<T>(sticker: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            sticker: sticker.into(),
        }
    }
}

impl Method for DeleteStickerFromSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteStickerFromSet", self)
    }
}

/// Deletes a sticker set that was created by the bot.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteStickerSet {
    name: String,
}

impl DeleteStickerSet {
    /// Creates a new `DeleteStickerSet`.
    ///
    /// # Arguments
    ///
    /// * `name` - Sticker set name.
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl Method for DeleteStickerSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteStickerSet", self)
    }
}

/// Returns a sticker set.
#[derive(Clone, Debug, Serialize)]
pub struct GetStickerSet {
    name: String,
}

impl GetStickerSet {
    /// Creates a new `GetStickerSet`.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the sticker set.
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl Method for GetStickerSet {
    type Response = StickerSet;

    fn into_payload(self) -> Payload {
        Payload::json("getStickerSet", self)
    }
}

/// Replaces an existing sticker in a sticker set with a new one.
///
/// The method is equivalent to calling [`crate::types::DeleteStickerFromSet`],
/// then [`crate::types::AddStickerToSet`],
/// then [`crate::types::SetStickerPositionInSet`].
#[derive(Debug)]
pub struct ReplaceStickerInSet {
    form: Form,
}

impl ReplaceStickerInSet {
    /// Creates a new `ReplaceStickerInSet`.
    ///
    /// # Arguments
    ///
    /// * `name` - Sticker set name.
    /// * `old_sticker` - File identifier of the replaced sticker.
    /// * `sticker` -  Information about the added sticker;
    ///   if exactly the same sticker had already been added to the set, then the set remains unchanged.
    /// * `user_id` - User identifier of the sticker set owner.
    pub fn new<A, B>(
        name: A,
        old_sticker: B,
        sticker: InputSticker,
        user_id: Integer,
    ) -> Result<Self, InputStickerError>
    where
        A: Into<String>,
        B: Into<String>,
    {
        let mut form: Form = sticker.try_into()?;
        form.insert_field("name", name.into());
        form.insert_field("old_sticker", old_sticker.into());
        form.insert_field("user_id", user_id);
        Ok(Self { form })
    }
}

impl Method for ReplaceStickerInSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::form("replaceStickerInSet", self.form)
    }
}

/// Sets the thumbnail of a custom emoji sticker set.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetCustomEmojiStickerSetThumbnail {
    name: String,
    custom_emoji_id: Option<String>,
}

impl SetCustomEmojiStickerSetThumbnail {
    /// Creates a new `SetCustomEmojiStickerSetThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `name` - Sticker set name.
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            custom_emoji_id: None,
        }
    }

    /// Sets a new custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom emoji identifier of a sticker from the sticker set.
    ///
    /// Pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    pub fn with_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_emoji_id = Some(value.into());
        self
    }
}

impl Method for SetCustomEmojiStickerSetThumbnail {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setCustomEmojiStickerSetThumbnail", self)
    }
}

/// Moves a sticker in a set created by the bot to a specific position.
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerPositionInSet {
    position: Integer,
    sticker: String,
}

impl SetStickerPositionInSet {
    /// Creates a new `SetStickerPositionInSet`.
    ///
    /// # Arguments
    ///
    /// * `position` - New sticker position in the set, zero-based.
    /// * `sticker` - File identifier of the sticker.
    pub fn new<T>(position: Integer, sticker: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            position,
            sticker: sticker.into(),
        }
    }
}

impl Method for SetStickerPositionInSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setStickerPositionInSet", self)
    }
}

/// Sets a title of a created sticker set.
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerSetTitle {
    name: String,
    title: String,
}

impl SetStickerSetTitle {
    /// Creates a new `SetStickerSetTitle`.
    ///
    /// # Arguments
    ///
    /// * `name` - Sticker set name.
    /// * `title` - Sticker set title; 1-64 characters.
    pub fn new<A, B>(name: A, title: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            name: name.into(),
            title: title.into(),
        }
    }
}

impl Method for SetStickerSetTitle {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setStickerSetTitle", self)
    }
}

/// Sets a thumbnail of a sticker set.
#[derive(Debug)]
pub struct SetStickerSetThumbnail {
    form: Form,
}

impl SetStickerSetThumbnail {
    /// Creates a new `SetStickerSetThumbnail`.
    ///
    /// # Arguments
    ///
    /// * `name` - Sticker set name.
    /// * `user_id` - User identifier of the sticker set owner.
    /// * `format` - Format of the thumbnail.
    pub fn new<N>(name: N, user_id: Integer, format: StickerFormat) -> Self
    where
        N: Into<String>,
    {
        Self {
            form: Form::from([
                ("name", name.into().into()),
                ("user_id", user_id.into()),
                ("format", format.as_ref().into()),
            ]),
        }
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - A WEBP or PNG image with the thumbnail.
    ///
    /// Must be up to 128 kilobytes in size and have a width and height of exactly 100px,
    /// or a .TGS animation with a thumbnail up to 32 kilobytes in size
    /// (see <https://core.telegram.org/stickers#animated-sticker-requirements> for animated sticker
    /// technical requirements), or a WEBM video with the thumbnail up to 32 kilobytes in size;
    /// see <https://core.telegram.org/stickers#video-sticker-requirements> for video sticker
    /// technical requirements.
    ///
    /// Animated and video sticker set thumbnails can't be uploaded via HTTP URL.
    /// If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    pub fn with_thumbnail<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.form.insert_field("thumbnail", value.into());
        self
    }
}

impl Method for SetStickerSetThumbnail {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::form("setStickerSetThumbnail", self.form)
    }
}

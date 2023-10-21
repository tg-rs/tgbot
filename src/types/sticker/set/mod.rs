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

/// Sticker set
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Type of stickers in the set
    pub sticker_type: StickerType,
    /// True, if the sticker set contains animated stickers
    pub is_animated: bool,
    /// True, if the sticker set contains video stickers
    pub is_video: bool,
    /// Sticker set thumbnail in the .WEBP or .TGS format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}

/// Add a new sticker to a set created by the bot
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
    /// Creates a new AddStickerToSet
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier of sticker set owner
    /// * name - Sticker set name
    /// * sticker - Sticker file
    pub fn new<T>(user_id: Integer, name: T, sticker: InputSticker) -> Result<Self, InputStickerError>
    where
        T: Into<String>,
    {
        let mut form: Form = sticker.try_into()?;
        form.insert_field("user_id", user_id);
        form.insert_field("name", name.into());
        Ok(AddStickerToSet { form })
    }
}

impl Method for AddStickerToSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::form("addStickerToSet", self.form)
    }
}

/// Create new sticker set owned by a user
///
/// The bot will be able to edit the created sticker set
#[derive(Debug)]
pub struct CreateNewStickerSet {
    form: Form,
}

impl CreateNewStickerSet {
    /// Creates a new CreateNewStickerSet with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier of created sticker set owner
    /// * name - Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals)
    ///          Can contain only english letters, digits and underscores
    ///          Must begin with a letter, can't contain consecutive underscores
    ///          and must end in “_by_<bot username>”
    ///          <bot_username> is case insensitive
    ///          1-64 characters
    /// * title - Sticker set title, 1-64 characters
    /// * stickers - A list of 1-50 initial stickers to be added to the sticker set
    /// * sticker_format - Format of stickers in the set,
    pub fn new<A, B>(
        user_id: Integer,
        name: A,
        title: B,
        stickers: InputStickers,
        sticker_format: StickerFormat,
    ) -> Result<Self, InputStickerError>
    where
        A: Into<String>,
        B: Into<String>,
    {
        let mut form: Form = stickers.try_into()?;
        form.insert_field("user_id", user_id);
        form.insert_field("name", name.into());
        form.insert_field("title", title.into());
        form.insert_field("sticker_format", sticker_format.as_ref());
        Ok(CreateNewStickerSet { form })
    }

    /// Pass True if stickers in the sticker set must be repainted to the color
    /// of text when used in messages, the accent color if used as emoji status,
    /// white on chat photos, or another appropriate color based on context;
    /// for custom emoji sticker sets only
    pub fn needs_repainting(mut self, value: bool) -> Self {
        self.form.insert_field("needs_repainting", value);
        self
    }

    /// Type of stickers in the set, pass “regular”, “mask”, or “custom_emoji”
    ///
    /// By default, a regular sticker set is created.
    pub fn sticker_type(mut self, value: StickerType) -> Self {
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

/// Delete a sticker from a set created by the bot
#[derive(Clone, Debug, Serialize)]
pub struct DeleteStickerFromSet {
    sticker: String,
}

impl DeleteStickerFromSet {
    /// Creates a new DeleteStickerFromSet
    ///
    /// # Arguments
    ///
    /// * sticker - File identifier of the sticker
    pub fn new<S: Into<String>>(sticker: S) -> Self {
        DeleteStickerFromSet {
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

/// Get a sticker set
#[derive(Clone, Debug, Serialize)]
pub struct GetStickerSet {
    name: String,
}

impl GetStickerSet {
    /// Creates a new GetStickerSet
    ///
    /// # Arguments
    ///
    /// * name - Name of the sticker set
    pub fn new<S: Into<String>>(name: S) -> Self {
        GetStickerSet { name: name.into() }
    }
}

impl Method for GetStickerSet {
    type Response = StickerSet;

    fn into_payload(self) -> Payload {
        Payload::json("getStickerSet", self)
    }
}

/// Move a sticker in a set created by the bot to a specific position
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerPositionInSet {
    sticker: String,
    position: Integer,
}

impl SetStickerPositionInSet {
    /// Creates a new SetStickerPositionInSet
    ///
    /// # Arguments
    ///
    /// * sticker - File identifier of the sticker
    /// * position - New sticker position in the set, zero-based
    pub fn new<S: Into<String>>(sticker: S, position: Integer) -> Self {
        SetStickerPositionInSet {
            sticker: sticker.into(),
            position,
        }
    }
}

impl Method for SetStickerPositionInSet {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setStickerPositionInSet", self)
    }
}

/// Use this method to set the thumbnail of a sticker set
#[derive(Debug)]
pub struct SetStickerSetThumb {
    form: Form,
}

impl SetStickerSetThumb {
    /// Creates a new SetStickerSetThumb
    ///
    /// # Arguments
    ///
    /// * name - Sticker set name
    /// * user_id - User identifier of the sticker set owner
    pub fn new<N>(name: N, user_id: Integer) -> Self
    where
        N: Into<String>,
    {
        let mut form = Form::new();
        form.insert_field("name", name.into());
        form.insert_field("user_id", user_id);
        Self { form }
    }

    /// Set a PNG image or TGS animation with the thumbnail
    pub fn thumb<T>(mut self, thumb: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.form.insert_field("thumb", thumb.into());
        self
    }
}

impl Method for SetStickerSetThumb {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::form("setStickerSetThumb", self.form)
    }
}

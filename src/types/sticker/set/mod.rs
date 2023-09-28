use serde::{Deserialize, Serialize};

use crate::{
    form::Form,
    method::Method,
    request::Request,
    types::{InputFile, Integer, MaskPosition, MaskPositionError, NewSticker, PhotoSize, Sticker},
};

use super::NewStickerKind;

#[cfg(test)]
mod tests;

/// Sticker set
#[derive(Clone, Debug, Deserialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// True, if the sticker set contains masks
    pub contains_masks: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// True, if the sticker set contains animated stickers
    pub is_animated: bool,
    /// True, if the sticker set contains video stickers
    pub is_video: bool,
    /// Sticker set thumbnail in the .WEBP or .TGS format
    pub thumb: Option<PhotoSize>,
}

/// Add a new sticker to a set created by the bot
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
    /// * emojis - One or more emoji corresponding to the sticker
    pub fn new<N, E>(user_id: Integer, name: N, sticker: NewSticker, emojis: E) -> Self
    where
        N: Into<String>,
        E: Into<String>,
    {
        let mut form = Form::new();
        form.insert_field("user_id", user_id);
        form.insert_field("name", name.into());
        match sticker.kind {
            NewStickerKind::Png(file) => {
                form.insert_field("png_sticker", file);
            }
            NewStickerKind::Tgs(file) => {
                form.insert_field("tgs_sticker", file);
            }
            NewStickerKind::Video(file) => {
                form.insert_field("webm_sticker", file);
            }
        };
        form.insert_field("emojis", emojis.into());
        AddStickerToSet { form }
    }

    /// Position where the mask should be placed on faces
    pub fn mask_position(mut self, value: MaskPosition) -> Result<Self, MaskPositionError> {
        self.form.insert_field("mask_position", value.serialize()?);
        Ok(self)
    }
}

impl Method for AddStickerToSet {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::form("addStickerToSet", self.form)
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

    fn into_request(self) -> Request {
        Request::json("deleteStickerFromSet", self)
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

    fn into_request(self) -> Request {
        Request::json("getStickerSet", self)
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
    /// * sticker - Sticker file
    /// * emojis - One or more emoji corresponding to the sticker
    pub fn new<N, T, E>(user_id: Integer, name: N, title: T, sticker: NewSticker, emojis: E) -> Self
    where
        N: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        let mut form = Form::new();
        form.insert_field("user_id", user_id);
        form.insert_field("name", name.into());
        form.insert_field("title", title.into());
        match sticker.kind {
            NewStickerKind::Png(file) => {
                form.insert_field("png_sticker", file);
            }
            NewStickerKind::Tgs(file) => {
                form.insert_field("tgs_sticker", file);
            }
            NewStickerKind::Video(file) => {
                form.insert_field("webm_sticker", file);
            }
        };
        form.insert_field("emojis", emojis.into());
        CreateNewStickerSet { form }
    }

    /// Pass True, if a set of mask stickers should be created
    pub fn contains_masks(mut self, value: bool) -> Self {
        self.form.insert_field("contains_masks", value);
        self
    }

    /// Position where the mask should be placed on faces
    pub fn mask_position(mut self, value: MaskPosition) -> Result<Self, MaskPositionError> {
        self.form.insert_field("mask_position", value.serialize()?);
        Ok(self)
    }
}

impl Method for CreateNewStickerSet {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::form("createNewStickerSet", self.form)
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

    fn into_request(self) -> Request {
        Request::json("setStickerPositionInSet", self)
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

    fn into_request(self) -> Request {
        Request::form("setStickerSetThumb", self.form)
    }
}

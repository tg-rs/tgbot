use crate::{
    methods::Method,
    request::{Form, Request},
    types::{Integer, MaskPosition, MaskPositionError, NewSticker, NewStickerKind},
};

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
    /// * sticker - PNG image or TGS animation
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::{InputFile, MaskPosition, MaskPositionPoint},
    };

    #[test]
    fn create_new_sticker_set_with_png() {
        let request = CreateNewStickerSet::new(
            1,
            "name",
            "title",
            NewSticker::png(InputFile::file_id("sticker-id")),
            "^_^",
        )
        .contains_masks(true)
        .mask_position(MaskPosition {
            point: MaskPositionPoint::Forehead,
            x_shift: 1.0,
            y_shift: 2.0,
            scale: 3.0,
        })
        .unwrap()
        .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/createNewStickerSet"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
            assert_eq!(form.fields["name"].get_text().unwrap(), "name");
            assert_eq!(form.fields["title"].get_text().unwrap(), "title");
            assert!(form.fields["png_sticker"].get_file().is_some());
            assert_eq!(form.fields["emojis"].get_text().unwrap(), "^_^");
            assert!(form.fields["mask_position"].get_text().is_some());
            assert_eq!(form.fields["contains_masks"].get_text().unwrap(), "true");
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn create_new_sticker_set_with_tgs() {
        let request = CreateNewStickerSet::new(
            1,
            "name",
            "title",
            NewSticker::tgs(InputFile::file_id("sticker-id")),
            "^_^",
        )
        .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/createNewStickerSet"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
            assert_eq!(form.fields["name"].get_text().unwrap(), "name");
            assert_eq!(form.fields["title"].get_text().unwrap(), "title");
            assert!(form.fields["tgs_sticker"].get_file().is_some());
            assert_eq!(form.fields["emojis"].get_text().unwrap(), "^_^");
        } else {
            panic!("Unexpected request body");
        }
    }
}

use crate::{
    methods::Method,
    request::{Form, Request},
    types::{Integer, MaskPosition, MaskPositionError, NewSticker, NewStickerKind},
};

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
    /// * sticker - PNG image or TGS animation
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::{InputFile, MaskPosition, MaskPositionPoint},
    };

    #[test]
    fn add_sticker_to_set_png() {
        let request = AddStickerToSet::new(1, "name", NewSticker::png(InputFile::file_id("sticker-id")), "^_^")
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
            "base-url/bottoken/addStickerToSet"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
            assert_eq!(form.fields["name"].get_text().unwrap(), "name");
            assert!(form.fields["png_sticker"].get_file().is_some());
            assert_eq!(form.fields["emojis"].get_text().unwrap(), "^_^");
            assert!(form.fields["mask_position"].get_text().is_some());
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn add_sticker_to_set_tgs() {
        let request =
            AddStickerToSet::new(1, "name", NewSticker::tgs(InputFile::file_id("sticker-id")), "^_^").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/addStickerToSet"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
            assert_eq!(form.fields["name"].get_text().unwrap(), "name");
            assert!(form.fields["tgs_sticker"].get_file().is_some());
            assert_eq!(form.fields["emojis"].get_text().unwrap(), "^_^");
        } else {
            panic!("Unexpected request body");
        }
    }
}

use crate::{
    methods::Method,
    request::{Form, Request},
    types::{InputFile, Integer},
};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn set_sticker_set_thumb() {
        let request = SetStickerSetThumb::new("name", 1)
            .thumb(InputFile::file_id("file-id"))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/setStickerSetThumb"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["name"].get_text().unwrap(), "name");
            assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
            assert!(form.fields["thumb"].get_file().is_some());
        } else {
            panic!("Unexpected request body");
        }
    }
}

use crate::{
    methods::Method,
    request::{Form, Request},
    types::{ChatId, Integer, MediaGroup, Message},
};

/// Send a group of photos or videos as an album
#[derive(Debug)]
pub struct SendMediaGroup {
    form: Form,
}

impl SendMediaGroup {
    /// Creates a new SendMediaGroup with empty optional parameters
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * media - Photos and videos to be sent, must include 2–10 items
    pub fn new<C: Into<ChatId>>(chat_id: C, media: MediaGroup) -> Self {
        let mut form: Form = media.into();
        form.insert_field("chat_id", chat_id.into());
        SendMediaGroup { form }
    }

    /// Sends the messages silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Protects the contents of the sent messages from forwarding and saving
    pub fn protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value);
        self
    }

    /// If the messages are a reply, ID of the original message
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
}

impl Method for SendMediaGroup {
    type Response = Vec<Message>;

    fn into_request(self) -> Request {
        Request::form("sendMediaGroup", self.form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::{InputFile, InputFileReader, InputMediaPhoto, InputMediaVideo, MediaGroupItem},
    };
    use std::io::Cursor;

    #[test]
    fn send_media_group() {
        let request = SendMediaGroup::new(
            1,
            MediaGroup::new(vec![
                MediaGroupItem::photo(InputFileReader::from(Cursor::new("test")), InputMediaPhoto::default()),
                MediaGroupItem::video(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default()),
                MediaGroupItem::video(InputFile::file_id("file-id"), InputMediaVideo::default())
                    .with_thumb(InputFile::url("thumb-url")),
            ])
            .unwrap(),
        )
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/sendMediaGroup"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
            assert!(form.fields.get("media").is_some());
            assert!(form.fields.get("tgbot_im_file_0").is_some());
            assert!(form.fields.get("tgbot_im_file_1").is_some());
            assert_eq!(form.fields["disable_notification"].get_text().unwrap(), "true");
            assert_eq!(form.fields["protect_content"].get_text().unwrap(), "true");
            assert_eq!(form.fields["reply_to_message_id"].get_text().unwrap(), "1");
            assert_eq!(form.fields["allow_sending_without_reply"].get_text().unwrap(), "true");
        } else {
            panic!("Unexpected request body");
        }
    }
}

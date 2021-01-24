use crate::{
    methods::Method,
    request::{Form, Request},
    types::{
        serialize_text_entities, ChatId, InputFile, Integer, Message, ParseMode, ReplyMarkup, ReplyMarkupError,
        TextEntity, TextEntityError,
    },
};

/// Send video file
///
/// Telegram clients support mp4 videos (other formats may be sent as Document)
/// Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future
#[derive(Debug)]
pub struct SendVideo {
    form: Form,
}

impl SendVideo {
    /// Creates a new SendVideo with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * video - Video to send
    pub fn new<C, V>(chat_id: C, video: V) -> Self
    where
        C: Into<ChatId>,
        V: Into<InputFile>,
    {
        let mut form = Form::new();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("video", video.into());
        SendVideo { form }
    }

    /// Duration of sent video in seconds
    pub fn duration(mut self, value: Integer) -> Self {
        self.form.insert_field("duration", value);
        self
    }

    /// Video width
    pub fn width(mut self, value: Integer) -> Self {
        self.form.insert_field("width", value);
        self
    }

    /// Video height
    pub fn height(mut self, value: Integer) -> Self {
        self.form.insert_field("height", value);
        self
    }

    /// Thumbnail of the file sent
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 320
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>” if the thumbnail was
    /// uploaded using multipart/form-data under <file_attach_name>
    pub fn thumb<V>(mut self, value: V) -> Self
    where
        V: Into<InputFile>,
    {
        self.form.insert_field("thumb", value.into());
        self
    }

    /// Video caption
    ///
    /// May also be used when resending videos by file_id
    /// 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, value: S) -> Self {
        self.form.insert_field("caption", value.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities(mut self, value: &[TextEntity]) -> Result<Self, TextEntityError> {
        self.form
            .insert_field("caption_entities", serialize_text_entities(value)?);
        self.form.remove_field("parse_mode");
        Ok(self)
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.form.insert_field("parse_mode", value);
        self.form.remove_field("caption_entities");
        self
    }

    /// Pass True, if the uploaded video is suitable for streaming
    pub fn supports_streaming(mut self, value: bool) -> Self {
        self.form.insert_field("supports_streaming", value);
        self
    }

    // Sends the message silently
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
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

impl Method for SendVideo {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::form("sendVideo", self.form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::ForceReply,
    };

    #[test]
    fn send_video() {
        let request = SendVideo::new(1, InputFile::file_id("file-id"))
            .duration(100)
            .width(200)
            .height(300)
            .thumb(InputFile::file_id("thumb-id"))
            .caption("caption")
            .parse_mode(ParseMode::Markdown)
            .supports_streaming(true)
            .disable_notification(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .unwrap()
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendVideo");
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
            assert!(form.fields["video"].get_file().is_some());
            assert_eq!(form.fields["duration"].get_text().unwrap(), "100");
            assert_eq!(form.fields["width"].get_text().unwrap(), "200");
            assert_eq!(form.fields["height"].get_text().unwrap(), "300");
            assert!(form.fields["thumb"].get_file().is_some());
            assert_eq!(form.fields["caption"].get_text().unwrap(), "caption");
            assert_eq!(form.fields["parse_mode"].get_text().unwrap(), "Markdown");
            assert_eq!(form.fields["supports_streaming"].get_text().unwrap(), "true");
            assert_eq!(form.fields["disable_notification"].get_text().unwrap(), "true");
            assert_eq!(form.fields["reply_to_message_id"].get_text().unwrap(), "1");
            assert_eq!(form.fields["allow_sending_without_reply"].get_text().unwrap(), "true");
            assert_eq!(
                form.fields["reply_markup"].get_text().unwrap(),
                r#"{"force_reply":true}"#
            );
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn send_video_caption() {
        let mut method = SendVideo::new(1, InputFile::file_id("file-id"));
        method = method.parse_mode(ParseMode::Markdown);
        assert_eq!(method.form.fields["parse_mode"].get_text().unwrap(), "Markdown");
        method = method.caption_entities(&[TextEntity::bold(0..10)]).unwrap();
        assert!(!method.form.fields.contains_key("parse_mode"));
        let caption_entities = method.form.fields["caption_entities"].get_text().unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&caption_entities).unwrap(),
            serde_json::json!([{"type": "bold", "offset":0, "length": 10}])
        );
    }
}

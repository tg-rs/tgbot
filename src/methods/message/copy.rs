use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer, MessageId, ParseMode, ReplyMarkup, TextEntity},
};
use serde::Serialize;

/// Copy messages of any kind
///
/// The method is analogous to the method forwardMessages,
/// but the copied message doesn't have a link to the original message
#[derive(Clone, Debug, Serialize)]
pub struct CopyMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<TextEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl CopyMessage {
    /// Creates a new CopyMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * from_chat_id - Unique identifier for the chat where the original message was sent
    /// * message_id - Message identifier in the chat specified in from_chat_id
    pub fn new<T, F>(chat_id: T, from_chat_id: F, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
        F: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    /// New caption for media, 0-1024 characters after entities parsing
    ///
    /// If not specified, the original caption is kept
    pub fn caption<C: Into<String>>(mut self, caption: C) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the new caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities(mut self, caption_entities: Vec<TextEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self.parse_mode = None;
        self
    }

    /// Mode for parsing entities in the new caption
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.caption_entities = None;
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for CopyMessage {
    type Response = MessageId;

    fn into_request(self) -> Request {
        Request::json("copyMessage", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::ForceReply,
    };
    use serde_json::Value;

    #[test]
    fn serialize_copy_message_full() {
        let request = CopyMessage::new(1, 2, 3)
            .caption("caption")
            .parse_mode(ParseMode::Markdown)
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(ForceReply::new(true))
            .allow_sending_without_reply(true)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/copyMessage");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["from_chat_id"], 2);
            assert_eq!(data["message_id"], 3);
            assert_eq!(data["caption"], "caption");
            assert_eq!(data["parse_mode"], "Markdown");
            assert_eq!(data["disable_notification"], true);
            assert_eq!(data["reply_to_message_id"], 1);
            assert_eq!(data["reply_markup"]["force_reply"], true);
            assert_eq!(data["allow_sending_without_reply"], true)
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn serialize_copy_message_partial() {
        let request = CopyMessage::new(1, 2, 3).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/copyMessage");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "from_chat_id": 2,
                    "message_id": 3
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}

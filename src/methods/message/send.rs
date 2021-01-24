use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer, Message, ParseMode, ReplyMarkup, TextEntity},
};
use serde::Serialize;

/// Send text messages
#[derive(Clone, Debug, Serialize)]
pub struct SendMessage {
    chat_id: ChatId,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<TextEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    /// Creates a new SendMessage with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * text - Text of the message to be sent
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, text: S) -> Self {
        SendMessage {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    /// Sets parse mode
    ///
    /// Entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.entities = None;
        self
    }

    /// List of special entities that appear in message text
    ///
    /// Parse mode will be set to None when this method is called
    pub fn entities(mut self, entities: Vec<TextEntity>) -> Self {
        self.entities = Some(entities);
        self.parse_mode = None;
        self
    }

    /// Disables link previews for links in this message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
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

impl Method for SendMessage {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendMessage", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::{ForceReply, TextEntity},
    };
    use serde_json::Value;

    #[test]
    fn send_message() {
        let request = SendMessage::new(1, "text")
            .parse_mode(ParseMode::Markdown)
            .entities(vec![TextEntity::bold(0..2)])
            .disable_web_page_preview(true)
            .disable_notification(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendMessage");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "text": "text",
                    "entities": [{
                        "type": "bold",
                        "offset": 0,
                        "length": 2
                    }],
                    "disable_web_page_preview": true,
                    "disable_notification": true,
                    "reply_to_message_id": 1,
                    "allow_sending_without_reply": true,
                    "reply_markup": {
                        "force_reply": true
                    }
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}

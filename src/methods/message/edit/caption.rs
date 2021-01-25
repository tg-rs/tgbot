use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, EditMessageResult, InlineKeyboardMarkup, Integer, ParseMode, TextEntity},
};
use serde::Serialize;

/// Edit caption of message sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageCaption {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<TextEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageCaption {
    /// Creates a new EditMessageCaption
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        EditMessageCaption {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageCaption
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        EditMessageCaption {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// New caption of the message
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities(mut self, caption_entities: Vec<TextEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self.parse_mode = None;
        self
    }

    /// Parse mode
    ///
    /// Entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.caption_entities = None;
        self
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageCaption {
    type Response = EditMessageResult;

    fn into_request(self) -> Request {
        Request::json("editMessageCaption", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::InlineKeyboardButton,
    };
    use serde_json::Value;

    #[test]
    fn edit_message_caption() {
        let request = EditMessageCaption::new(1, 2)
            .caption("caption")
            .parse_mode(ParseMode::Markdown)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/editMessageCaption"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "message_id": 2,
                    "caption": "caption",
                    "parse_mode": "Markdown",
                    "reply_markup": {
                        "inline_keyboard": [
                            [
                                {"text": "text", "url": "url"}
                            ]
                        ]
                    }
                })
            );
        } else {
            panic!("Unexpected request body");
        }

        let request = EditMessageCaption::with_inline_message_id("msg-id")
            .caption_entities(vec![TextEntity::bold(0..10)])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/editMessageCaption"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "inline_message_id": "msg-id",
                    "caption_entities": [{"type": "bold", "offset": 0, "length": 10}]
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}

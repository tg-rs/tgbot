use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer, Message, ReplyMarkup},
};
use serde::Serialize;

/// Use this method to send a dice
///
/// Dice will have a random value from 1 to 6
#[derive(Clone, Debug, Serialize)]
pub struct SendDice {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendDice {
    /// Creates a new SendDice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
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

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendDice {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendDice", self)
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
    fn send_dice() {
        let request = SendDice::new(1)
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendDice");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["disable_notification"], true);
            assert_eq!(data["reply_to_message_id"], 1);
            assert_eq!(data["reply_markup"]["force_reply"], true);
        } else {
            panic!("Unexpected request body");
        }
    }
}
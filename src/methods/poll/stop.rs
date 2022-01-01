use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, InlineKeyboardMarkup, Integer, Poll},
};
use serde::Serialize;

/// Use this method to stop a poll which was sent by the bot
///
/// On success, the stopped Poll with the final results is returned
#[derive(Clone, Debug, Serialize)]
pub struct StopPoll {
    chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to stop a quiz which was sent by the bot
///
/// On success, the stopped Quiz with the final results is returned
pub type StopQuiz = StopPoll;

impl StopPoll {
    /// Creates a new StopPoll
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the original message with the poll
    pub fn new<C>(chat_id: C, message_id: Integer) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// A JSON-serialized object for a new message inline keyboard
    pub fn reply_markup<R: Into<InlineKeyboardMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for StopPoll {
    type Response = Poll;

    fn into_request(self) -> Request {
        Request::json("stopPoll", self)
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
    fn stop_poll() {
        let request = StopPoll::new(1, 2)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/stopPoll");
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
                assert_eq!(data["chat_id"], 1);
                assert_eq!(data["message_id"], 2);
                assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }
}

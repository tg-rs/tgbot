use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer, Message, ParseMode, PollKind, ReplyMarkup, TextEntity},
};
use serde::Serialize;

use super::parameters::PollParameters;

/// Use this method to send a quiz
///
/// On success, the sent Message is returned
#[derive(Clone, Debug, Serialize)]
pub struct SendQuiz {
    #[serde(flatten)]
    inner: PollParameters,
}

impl SendQuiz {
    /// Creates a new SendQuiz
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * question - Quiz question, 1-300 characters
    pub fn new<C, Q>(chat_id: C, question: Q) -> Self
    where
        C: Into<ChatId>,
        Q: Into<String>,
    {
        Self {
            inner: PollParameters::new(chat_id.into(), question.into(), PollKind::Quiz),
        }
    }

    /// Adds an answer option (1-100 characters)
    pub fn option<O>(mut self, option: O) -> Self
    where
        O: Into<String>,
    {
        self.inner.options.push(option.into());
        self
    }

    /// True, if the quiz needs to be anonymous, defaults to True
    #[allow(clippy::wrong_self_convention)]
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.inner.is_anonymous = Some(is_anonymous);
        self
    }

    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    pub fn correct_option_id(mut self, correct_option_id: Integer) -> Self {
        self.inner.correct_option_id = Some(correct_option_id);
        self
    }

    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon
    ///
    /// 0-200 characters with at most 2 line feeds after entities parsing
    pub fn explanation<E: Into<String>>(mut self, explanation: E) -> Self {
        self.inner.explanation = Some(explanation.into());
        self
    }

    /// Mode for parsing entities in the explanation
    ///
    /// Explanation entities will be set to None when this method is called
    pub fn explanation_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.inner.explanation_parse_mode = Some(parse_mode);
        self.inner.explanation_entities = None;
        self
    }

    /// List of special entities that appear in the poll explanation
    ///
    /// Explanation parse mode will be set to None when this method is called
    pub fn explanation_entities(mut self, entities: Vec<TextEntity>) -> Self {
        self.inner.explanation_entities = Some(entities);
        self.inner.explanation_parse_mode = None;
        self
    }

    /// Amount of time in seconds the poll will be active after creation, 5-600
    ///
    /// Can't be used together with close_date (close_date will be set to None)
    pub fn open_period(mut self, period: Integer) -> Self {
        self.inner.open_period = Some(period);
        self.inner.close_date = None;
        self
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed
    ///
    /// Must be at least 5 and no more than 600 seconds in the future.
    /// Can't be used together with open_period (open_period will be set to None)
    pub fn close_date(mut self, close_date: Integer) -> Self {
        self.inner.close_date = Some(close_date);
        self.inner.open_period = None;
        self
    }

    /// Pass True, if the poll needs to be immediately closed
    #[allow(clippy::wrong_self_convention)]
    pub fn is_closed(mut self, is_closed: bool) -> Self {
        self.inner.is_closed = Some(is_closed);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.inner.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.inner.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.inner.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.inner.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self
    where
        R: Into<ReplyMarkup>,
    {
        self.inner.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendQuiz {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendPoll", self)
    }
}

/// Use this method to send a native poll
///
/// On success, the sent Message is returned
#[derive(Clone, Debug, Serialize)]
pub struct SendPoll {
    #[serde(flatten)]
    inner: PollParameters,
}

impl SendPoll {
    /// Creates a new SendPoll
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * question - Poll question, 1-300 characters
    pub fn new<C, Q>(chat_id: C, question: Q) -> Self
    where
        C: Into<ChatId>,
        Q: Into<String>,
    {
        Self {
            inner: PollParameters::new(chat_id.into(), question.into(), PollKind::Regular),
        }
    }

    /// Adds an answer option (1-100 characters)
    pub fn option<O>(mut self, option: O) -> Self
    where
        O: Into<String>,
    {
        self.inner.options.push(option.into());
        self
    }

    /// True, if the poll needs to be anonymous, defaults to True
    #[allow(clippy::wrong_self_convention)]
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.inner.is_anonymous = Some(is_anonymous);
        self
    }

    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    pub fn allows_multiple_answers(mut self, allows_multiple_answers: bool) -> Self {
        self.inner.allows_multiple_answers = Some(allows_multiple_answers);
        self
    }

    /// Amount of time in seconds the poll will be active after creation, 5-600
    ///
    /// Can't be used together with close_date (close_date will be set to None)
    pub fn open_period(mut self, period: Integer) -> Self {
        self.inner.open_period = Some(period);
        self.inner.close_date = None;
        self
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed
    ///
    /// Must be at least 5 and no more than 600 seconds in the future.
    /// Can't be used together with open_period (open_period will be set to None)
    pub fn close_date(mut self, close_date: Integer) -> Self {
        self.inner.close_date = Some(close_date);
        self.inner.open_period = None;
        self
    }

    /// Pass True, if the poll needs to be immediately closed
    #[allow(clippy::wrong_self_convention)]
    pub fn is_closed(mut self, is_closed: bool) -> Self {
        self.inner.is_closed = Some(is_closed);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.inner.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.inner.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.inner.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.inner.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self
    where
        R: Into<ReplyMarkup>,
    {
        self.inner.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendPoll {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendPoll", self)
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
    fn send_quiz() {
        let request = SendQuiz::new(1, "Q")
            .option("O1")
            .option("O2")
            .is_anonymous(false)
            .correct_option_id(0)
            .is_closed(false)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendPoll");
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
                assert_eq!(
                    data,
                    serde_json::json!({
                        "chat_id": 1,
                        "question": "Q",
                        "type": "quiz",
                        "options": ["O1", "O2"],
                        "is_anonymous": false,
                        "is_closed": false,
                        "correct_option_id": 0,
                        "disable_notification": true,
                        "protect_content": true,
                        "reply_to_message_id": 1,
                        "allow_sending_without_reply": true,
                        "reply_markup": {
                            "force_reply": true
                        }
                    })
                );
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }

    #[test]
    fn send_poll() {
        let request = SendPoll::new(1, "Q")
            .option("O1")
            .option("O2")
            .is_anonymous(false)
            .allows_multiple_answers(true)
            .is_closed(false)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendPoll");
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();

                assert_eq!(
                    data,
                    serde_json::json!({
                        "chat_id": 1,
                        "question": "Q",
                        "type": "regular",
                        "options": ["O1", "O2"],
                        "is_anonymous": false,
                        "is_closed": false,
                        "allows_multiple_answers": true,
                        "disable_notification": true,
                        "protect_content": true,
                        "reply_to_message_id": 1,
                        "allow_sending_without_reply": true,
                        "reply_markup": {
                            "force_reply": true
                        }
                    })
                );
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }
}

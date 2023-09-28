use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    method::Method,
    request::Request,
    types::{
        text::Text,
        ChatId,
        InlineKeyboardMarkup,
        Integer,
        Message,
        ParseMode,
        ReplyMarkup,
        TextEntities,
        TextEntity,
        User,
    },
};

#[cfg(test)]
mod tests;

/// Contains information about a poll
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Poll {
    /// A regular poll
    Regular(RegularPoll),
    /// A quiz
    Quiz(Quiz),
}

/// Kind of a poll
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PollKind {
    /// A quiz
    Quiz,
    /// A regular poll
    Regular,
}

/// A regular poll
#[derive(Clone, Debug, Deserialize)]
pub struct RegularPoll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: Integer,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<Integer>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<Integer>,
}

/// A quiz
#[derive(Clone, Debug, Deserialize)]
pub struct Quiz {
    /// Unique quiz identifier
    pub id: String,
    /// Quiz question, 1-255 characters
    pub question: String,
    /// List of quiz options
    pub options: Vec<PollOption>,
    /// Total number of users that answered to the quiz
    pub total_voter_count: Integer,
    /// True, if the quiz is closed
    pub is_closed: bool,
    /// True, if the quiz is anonymous
    pub is_anonymous: bool,
    /// 0-based identifier of the correct answer option
    ///
    /// Available only for a closed quiz,
    /// or was sent (not forwarded) by the bot or
    /// to the private chat with the bot
    pub correct_option_id: Integer,
    /// Text that is shown when a user chooses an incorrect answer or
    /// taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[serde(deserialize_with = "deserialize_explanation")]
    #[serde(flatten)]
    pub explanation: Option<Text>,
    /// Amount of time in seconds the quiz will be active after creation
    pub open_period: Option<Integer>,
    /// Point in time (Unix timestamp) when the quiz will be automatically closed
    pub close_date: Option<Integer>,
}

fn deserialize_explanation<'de, D>(deserializer: D) -> Result<Option<Text>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper {
        explanation: String,
        explanation_entities: Option<TextEntities>,
    }

    Option::<Wrapper>::deserialize(deserializer).map(|wrapper| {
        wrapper.map(
            |Wrapper {
                 explanation: data,
                 explanation_entities: entities,
             }| Text { data, entities },
        )
    })
}

/// Contains information about one answer option in a poll
#[derive(Clone, Debug, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: Integer,
}

/// An answer of a user in a non-anonymous poll
#[derive(Clone, Debug, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The user, who changed the answer to the poll
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user
    ///
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<Integer>,
}

#[derive(Clone, Debug, Serialize)]
struct PollParameters {
    chat_id: ChatId,
    question: String,
    options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    kind: Option<PollKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correct_option_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl PollParameters {
    fn new(chat_id: ChatId, question: String, kind: PollKind) -> Self {
        Self {
            chat_id,
            question,
            options: Vec::new(),
            is_anonymous: None,
            kind: Some(kind),
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

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
    pub fn explanation_entities<T>(mut self, entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.inner.explanation_entities = Some(entities.into_iter().collect());
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

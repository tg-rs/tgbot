use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    api::{Method, Payload},
    types::{
        Chat,
        ChatId,
        InlineKeyboardMarkup,
        Integer,
        Message,
        ParseMode,
        ReplyMarkup,
        ReplyParameters,
        Text,
        TextEntities,
        TextEntity,
        User,
    },
};

#[cfg(test)]
mod tests;

/// Represents a poll.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Poll {
    /// A regular poll.
    Regular(RegularPoll),
    /// A quiz.
    Quiz(Quiz),
}

/// Represents a type of a poll.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PollType {
    /// A quiz.
    Quiz,
    /// A regular poll.
    Regular,
}

#[derive(Deserialize, Serialize)]
struct RawQuestion {
    question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_entities: Option<TextEntities>,
}

impl RawQuestion {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<Text, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self::deserialize(deserializer)?;
        Ok(Text {
            data: value.question,
            entities: value.question_entities,
        })
    }

    fn serialize_value<S>(value: &Text, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self {
            question: value.data.clone(),
            question_entities: value.entities.clone(),
        }
        .serialize(serializer)
    }
}

/// Represents a regular poll.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct RegularPoll {
    /// Indicates whether the poll allows multiple answers.
    pub allows_multiple_answers: bool,
    /// Unique identifier of the poll.
    pub id: String,
    /// Indicates whether the poll is anonymous.
    pub is_anonymous: bool,
    /// Indicates whether the poll is closed.
    pub is_closed: bool,
    /// List of options.
    pub options: Vec<PollOption>,
    /// Question; 1-255 characters.
    #[serde(
        flatten,
        deserialize_with = "RawQuestion::deserialize_value",
        serialize_with = "RawQuestion::serialize_value"
    )]
    pub question: Text,
    /// Total number of users that voted in the poll.
    pub total_voter_count: Integer,
    /// Point in time (Unix timestamp) when the poll will be automatically closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<Integer>,
    /// Amount of time in seconds the poll will be active after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<Integer>,
}

impl RegularPoll {
    /// Creates a new `RegularPoll`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier.
    /// * `question` - Question; 1-255 characters.
    pub fn new<A, B>(id: A, question: B) -> Self
    where
        A: Into<String>,
        B: Into<Text>,
    {
        Self {
            allows_multiple_answers: false,
            id: id.into(),
            is_anonymous: false,
            is_closed: false,
            options: vec![],
            question: question.into(),
            total_voter_count: 0,
            close_date: None,
            open_period: None,
        }
    }

    /// Sets a new value for an `allows_multiple_answers` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll allows multiple answers.
    pub fn with_allows_multiple_answers(mut self, value: bool) -> Self {
        self.allows_multiple_answers = value;
        self
    }

    /// Sets a new close date.
    ///
    /// # Arguments
    ///
    /// * `value` - Point in time (Unix timestamp) when the poll will be automatically closed.
    pub fn with_close_date(mut self, value: Integer) -> Self {
        self.close_date = Some(value);
        self
    }

    /// Sets a new value for an `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the quiz is anonymous.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.is_anonymous = value;
        self
    }

    /// Sets a new value for an `is_closed` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the quiz is closed.
    pub fn with_is_closed(mut self, value: bool) -> Self {
        self.is_closed = value;
        self
    }

    /// Sets a new open period.
    ///
    /// # Arguments
    ///
    /// * `value` - Amount of time in seconds the poll will be active after creation.
    pub fn with_open_period(mut self, value: Integer) -> Self {
        self.open_period = Some(value);
        self
    }

    /// Sets a new list of options.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of options.
    pub fn with_options<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PollOption>,
    {
        self.options = value.into_iter().collect();
        self
    }

    /// Sets a new correct total voter count.
    ///
    /// # Arguments
    ///
    /// * `value` - Total number of users that answered to the quiz.
    pub fn with_total_voter_count(mut self, value: Integer) -> Self {
        self.total_voter_count = value;
        self
    }
}

/// Represents a quiz.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Quiz {
    /// 0-based identifier of the correct answer option.
    ///
    /// Available only for a closed quiz,
    /// or was sent (not forwarded) by the bot or
    /// to the private chat with the bot.
    pub correct_option_id: Integer,
    /// Unique identifier of the quiz.
    pub id: String,
    /// Indicates whether the quiz is anonymous.
    pub is_anonymous: bool,
    /// Indicates whether the quiz is closed.
    pub is_closed: bool,
    /// List of options.
    pub options: Vec<PollOption>,
    /// Question; 1-255 characters.
    #[serde(
        flatten,
        deserialize_with = "RawQuestion::deserialize_value",
        serialize_with = "RawQuestion::serialize_value"
    )]
    pub question: Text,
    /// Total number of users that answered to the quiz.
    pub total_voter_count: Integer,
    /// Point in time (Unix timestamp) when the quiz will be automatically closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<Integer>,
    /// Text that is shown when a user chooses an incorrect answer or
    /// taps on the lamp icon; 0-200 characters.
    #[serde(
        flatten,
        deserialize_with = "QuizExplanation::deserialize_value",
        serialize_with = "QuizExplanation::serialize_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub explanation: Option<Text>,
    /// Amount of time in seconds the quiz will be active after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<Integer>,
}

impl Quiz {
    /// Creates a new `Quiz`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier.
    /// * `question` - Question; 1-255 characters.
    pub fn new<A, B>(id: A, question: B) -> Self
    where
        A: Into<String>,
        B: Into<Text>,
    {
        Self {
            correct_option_id: 0,
            id: id.into(),
            is_anonymous: false,
            is_closed: false,
            options: vec![],
            question: question.into(),
            total_voter_count: 0,
            close_date: None,
            explanation: None,
            open_period: None,
        }
    }

    /// Sets a new close date.
    ///
    /// # Arguments
    ///
    /// * `value` - Point in time (Unix timestamp) when the quiz will be automatically closed.
    pub fn with_close_date(mut self, value: Integer) -> Self {
        self.close_date = Some(value);
        self
    }

    /// Sets a new correct option ID.
    ///
    /// # Arguments
    ///
    /// * `value` - 0-based identifier of the correct answer option.
    pub fn with_correct_option_id(mut self, value: Integer) -> Self {
        self.correct_option_id = value;
        self
    }

    /// Sets a new explanation.
    ///
    /// # Arguments
    ///
    /// * `value` - Text that is shown when a user chooses
    ///             an incorrect answer or
    ///             taps on the lamp icon; 0-200 characters.
    pub fn with_explanation<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.explanation = Some(value.into());
        self
    }

    /// Sets a new value for the `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the quiz is anonymous.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.is_anonymous = value;
        self
    }

    /// Sets a new value for the `is_closed` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the quiz is closed.
    pub fn with_is_closed(mut self, value: bool) -> Self {
        self.is_closed = value;
        self
    }

    /// Sets a new open period.
    ///
    /// # Arguments
    ///
    /// * `value` - Amount of time in seconds the quiz will be active after creation.
    pub fn with_open_period(mut self, value: Integer) -> Self {
        self.open_period = Some(value);
        self
    }

    /// Sets a new list of options.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of options.
    pub fn with_options<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PollOption>,
    {
        self.options = value.into_iter().collect();
        self
    }

    /// Sets a new correct total voter count.
    ///
    /// # Arguments
    ///
    /// * `value` - Total number of users that answered to the quiz.
    pub fn with_total_voter_count(mut self, value: Integer) -> Self {
        self.total_voter_count = value;
        self
    }
}

#[derive(Deserialize, Serialize)]
struct QuizExplanation {
    explanation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_entities: Option<TextEntities>,
}

impl QuizExplanation {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<Option<Text>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<QuizExplanation>::deserialize(deserializer).map(|x| {
            x.map(|value| Text {
                data: value.explanation,
                entities: value.explanation_entities,
            })
        })
    }

    fn serialize_value<S>(value: &Option<Text>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = value.clone().map(|value| QuizExplanation {
            explanation: value.data,
            explanation_entities: value.entities,
        });
        value.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
struct RawPollOptionText {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_entities: Option<TextEntities>,
}

impl RawPollOptionText {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<Text, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self::deserialize(deserializer)?;
        Ok(Text {
            data: value.text,
            entities: value.text_entities,
        })
    }

    fn serialize_value<S>(value: &Text, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self {
            text: value.data.clone(),
            text_entities: value.entities.clone(),
        }
        .serialize(serializer)
    }
}

/// Represents an answer option in a poll.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PollOption {
    /// Option text; 1-100 characters.
    #[serde(
        flatten,
        deserialize_with = "RawPollOptionText::deserialize_value",
        serialize_with = "RawPollOptionText::serialize_value"
    )]
    pub text: Text,
    /// Number of users that voted for this option.
    pub voter_count: Integer,
}

impl PollOption {
    /// Creates a new `PollOption`.
    ///
    /// # Arguments
    ///
    /// * `text` - Option text; 1-100 characters.
    /// * `voter_count` - Number of users that voted for this option.
    pub fn new<T>(text: T, voter_count: Integer) -> Self
    where
        T: Into<Text>,
    {
        Self {
            text: text.into(),
            voter_count,
        }
    }
}

/// Represents an answer of a user in a non-anonymous poll.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PollAnswer {
    /// 0-based identifiers of answer options, chosen by the user.
    ///
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<Integer>,
    /// Unique poll identifier.
    pub poll_id: String,
    /// The chat or the user that changed answer to the poll.
    #[serde(flatten)]
    pub voter: PollAnswerVoter,
}

impl PollAnswer {
    /// Creates a new `PollAnswer`.
    ///
    /// # Arguments
    ///
    /// * `option_ids` - 0-based identifiers of answer options, chosen by the user.
    /// * `poll_id` - Unique poll identifier.
    /// * `voter` - The chat or the user that changed answer to the poll.
    pub fn new<A, B, C>(option_ids: A, poll_id: B, voter: C) -> Self
    where
        A: IntoIterator<Item = Integer>,
        B: Into<String>,
        C: Into<PollAnswerVoter>,
    {
        Self {
            option_ids: option_ids.into_iter().collect(),
            poll_id: poll_id.into(),
            voter: voter.into(),
        }
    }
}

/// Represents the chat or the user that changed answer to the poll.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PollAnswerVoter {
    /// The chat that changed the answer to the poll, if the voter is anonymous.
    Chat(Chat),
    /// The user that changed the answer to the poll, if the voter isn't anonymous.
    User(User),
}

/// Contains information about one answer option in a poll to send.
#[derive(Clone, Debug, Serialize)]
pub struct InputPollOption {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_entities: Option<TextEntities>,
}

impl InputPollOption {
    /// Creates a new `InputPollOption`.
    ///
    /// # Arguments
    ///
    /// * `text` - Option text; 1-100 characters.
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: text.into(),
            text_parse_mode: None,
            text_entities: None,
        }
    }

    /// Sets a new list of text entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the poll option text.
    ///
    /// Text parse mode will be set to [`None`] when this method is called.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.text_entities = Some(value.into_iter().collect());
        self.text_parse_mode = None;
        self
    }

    /// Sets a new text parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` -  Mode for parsing entities in the text.
    ///
    /// Currently, only custom emoji entities are allowed.
    /// Text entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.text_parse_mode = Some(value);
        self.text_entities = None;
        self
    }
}

impl<T> From<T> for InputPollOption
where
    T: Into<Text>,
{
    fn from(value: T) -> Self {
        let value = value.into();
        Self {
            text: value.data,
            text_entities: value.entities,
            text_parse_mode: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct PollParameters {
    chat_id: ChatId,
    options: Vec<InputPollOption>,
    question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correct_option_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    poll_type: Option<PollType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
}

impl PollParameters {
    fn new<A, B>(chat_id: ChatId, question: String, poll_type: PollType, options: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<InputPollOption>,
    {
        Self {
            chat_id,
            options: options.into_iter().map(Into::into).collect(),
            question,
            allows_multiple_answers: None,
            business_connection_id: None,
            close_date: None,
            correct_option_id: None,
            disable_notification: None,
            explanation: None,
            explanation_entities: None,
            explanation_parse_mode: None,
            is_anonymous: None,
            is_closed: None,
            message_effect_id: None,
            message_thread_id: None,
            open_period: None,
            poll_type: Some(poll_type),
            question_entities: None,
            question_parse_mode: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
        }
    }
}

/// Sends a quiz.
///
/// On success, the sent [`Message`] is returned.
#[derive(Clone, Debug, Serialize)]
pub struct SendQuiz {
    #[serde(flatten)]
    inner: PollParameters,
}

impl SendQuiz {
    /// Creates a new `SendQuiz`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `question` - Question; 1-300 characters.
    /// * `correct_option_id` - 0-based identifier of the correct answer option.
    /// * `options` - Answer options; 2-10 strings 1-100 characters each.
    pub fn new<A, B, C, D>(chat_id: A, question: B, correct_option_id: Integer, options: C) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
        C: IntoIterator<Item = D>,
        D: Into<InputPollOption>,
    {
        let mut parameters = PollParameters::new(chat_id.into(), question.into(), PollType::Quiz, options);
        parameters.correct_option_id = Some(correct_option_id);
        Self { inner: parameters }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new close date.
    ///
    /// # Arguments
    ///
    /// * `value` - Point in time (Unix timestamp) when the quiz will be automatically closed.
    ///
    /// Must be at least 5 and no more than 600 seconds in the future.
    /// Can't be used together with [`Self::with_open_period`] (open period will be set to [`None`]).
    pub fn with_close_date(mut self, value: Integer) -> Self {
        self.inner.close_date = Some(value);
        self.inner.open_period = None;
        self
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.inner.disable_notification = Some(value);
        self
    }

    /// Sets a new explanation.
    ///
    /// # Arguments
    ///
    /// * `value` - Text that is shown when a user chooses
    ///             an incorrect answer or taps on the lamp icon;
    ///             0-200 characters with at most 2 line feeds after entities parsing.
    pub fn with_explanation<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.explanation = Some(value.into());
        self
    }

    /// Sets a new list of explanation entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the quiz explanation.
    ///
    /// Explanation parse mode will be set to [`None`] when this method is called.
    pub fn with_explanation_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.inner.explanation_entities = Some(value.into_iter().collect());
        self.inner.explanation_parse_mode = None;
        self
    }

    /// Sets a new explanation parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the explanation.
    ///
    /// Explanation entities will be set to [`None`] when this method is called.
    pub fn with_explanation_parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.explanation_parse_mode = Some(value);
        self.inner.explanation_entities = None;
        self
    }

    /// Sets a new value for an `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the quiz needs to be anonymous; default - `true`.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.inner.is_anonymous = Some(value);
        self
    }

    /// Sets a new value for an `is_closed` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the quiz needs to be immediately closed.
    pub fn with_is_closed(mut self, value: bool) -> Self {
        self.inner.is_closed = Some(value);
        self
    }

    /// Sets a new message effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect to be added to the message; for private chats only.
    pub fn with_message_effect_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.message_effect_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.inner.message_thread_id = Some(value);
        self
    }

    /// Sets a new open period.
    ///
    /// # Arguments
    ///
    /// * `value` - Amount of time in seconds the quiz will be active after creation; 5-600.
    ///
    /// Can't be used together with [`Self::with_close_date`] (close date will be set to [`None`]).
    pub fn with_open_period(mut self, value: Integer) -> Self {
        self.inner.open_period = Some(value);
        self.inner.close_date = None;
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.inner.protect_content = Some(value);
        self
    }

    /// Sets a new list of question entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the poll question.
    ///
    /// Question parse mode will be set to [`None`] when this method is called.
    pub fn with_question_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.inner.question_entities = Some(value.into_iter().collect());
        self.inner.question_parse_mode = None;
        self
    }

    /// Sets a new question parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the question.
    ///
    /// Question entities will be set to [`None`] when this method is called.
    pub fn with_question_parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.question_parse_mode = Some(value);
        self.inner.question_entities = None;
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.inner.reply_markup = Some(value.into());
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.inner.reply_parameters = Some(value);
        self
    }
}

impl Method for SendQuiz {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendPoll", self)
    }
}

/// Sends a native poll.
///
/// On success, the sent [`Message`] is returned.
#[derive(Clone, Debug, Serialize)]
pub struct SendPoll {
    #[serde(flatten)]
    inner: PollParameters,
}

impl SendPoll {
    /// Creates a new `SendPoll`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `question` - Question; 1-300 characters.
    /// * `options` - Answer options; 2-10 strings 1-100 characters each.
    pub fn new<A, B, C, D>(chat_id: A, question: B, options: C) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
        C: IntoIterator<Item = D>,
        D: Into<InputPollOption>,
    {
        Self {
            inner: PollParameters::new(chat_id.into(), question.into(), PollType::Regular, options),
        }
    }

    /// Sets a new value for an `allows_multiple_answers` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll allows multiple answers; default - `false`.
    pub fn with_allows_multiple_answers(mut self, value: bool) -> Self {
        self.inner.allows_multiple_answers = Some(value);
        self
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new close date.
    ///
    /// # Arguments
    ///
    /// * `value` - Point in time (Unix timestamp) when the poll will be automatically closed.
    ///
    /// Must be at least 5 and no more than 600 seconds in the future.
    /// Can't be used together with [`Self::with_open_period`] (open period will be set to [`None`])
    pub fn with_close_date(mut self, value: Integer) -> Self {
        self.inner.close_date = Some(value);
        self.inner.open_period = None;
        self
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.inner.disable_notification = Some(value);
        self
    }

    /// Sets a new value for an `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll needs to be anonymous; default - `true`.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.inner.is_anonymous = Some(value);
        self
    }

    /// Sets a new value for an `is_closed` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll needs to be immediately closed.
    pub fn with_is_closed(mut self, value: bool) -> Self {
        self.inner.is_closed = Some(value);
        self
    }

    /// Sets a new message effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect to be added to the message; for private chats only.
    pub fn with_message_effect_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.message_effect_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.inner.message_thread_id = Some(value);
        self
    }

    /// Sets a new open period.
    ///
    /// # Arguments
    ///
    /// * `value` - Amount of time in seconds the poll will be active after creation; 5-600.
    ///
    /// Can't be used together with `close_date` (`close_date` will be set to [`None`]).
    pub fn with_open_period(mut self, value: Integer) -> Self {
        self.inner.open_period = Some(value);
        self.inner.close_date = None;
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.inner.protect_content = Some(value);
        self
    }

    /// Sets a new list of question entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the poll question.
    ///
    /// Question parse mode will be set to [`None`] when this method is called.
    pub fn with_question_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.inner.question_entities = Some(value.into_iter().collect());
        self.inner.question_parse_mode = None;
        self
    }

    /// Sets a new question parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the question.
    ///
    /// Question entities will be set to [`None`] when this method is called.
    pub fn with_question_parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.question_parse_mode = Some(value);
        self.inner.question_entities = None;
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.inner.reply_markup = Some(value.into());
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.inner.reply_parameters = Some(value);
        self
    }
}

impl Method for SendPoll {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendPoll", self)
    }
}

/// Stops a poll which was sent by the bot.
///
/// On success, the stopped [`Poll`] with the final results is returned.
#[derive(Clone, Debug, Serialize)]
pub struct StopPoll {
    chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Stops a quiz which was sent by the bot.
///
/// On success, the stopped [`Quiz`] with the final results is returned.
pub type StopQuiz = StopPoll;

impl StopPoll {
    /// Creates a new `StopPoll`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the original message with the poll.
    pub fn new<T>(chat_id: T, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for StopPoll {
    type Response = Poll;

    fn into_payload(self) -> Payload {
        Payload::json("stopPoll", self)
    }
}

use std::{error::Error, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    api::{Form, Method, Payload},
    types::{
        Animation,
        Audio,
        Chat,
        ChatId,
        Document,
        InlineKeyboardMarkup,
        InputMedia,
        InputMediaData,
        InputMediaError,
        Integer,
        LivePhoto,
        Location,
        Message,
        ParseMode,
        PhotoSize,
        ReplyMarkup,
        ReplyMarkupError,
        ReplyParameters,
        ReplyParametersError,
        Sticker,
        Text,
        TextEntities,
        TextEntity,
        TextEntityError,
        User,
        Venue,
        Video,
    },
};

/// Represents a poll.
#[derive(Clone, Debug, derive_more::From, Deserialize, Serialize)]
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

impl fmt::Display for PollType {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Quiz => write!(out, "quiz"),
            Self::Regular => write!(out, "regular"),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize)]
struct RawQuestion {
    question: String,
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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegularPoll {
    /// Indicates whether the poll allows multiple answers.
    pub allows_multiple_answers: bool,
    /// Whether the poll allows to change the chosen answer options.
    pub allows_revoting: bool,
    /// Unique identifier of the poll.
    pub id: String,
    /// Indicates whether the poll is anonymous.
    pub is_anonymous: bool,
    /// Indicates whether the poll is closed.
    pub is_closed: bool,
    /// Indicates whether voting is limited to users
    /// who have been members of the chat where
    /// the poll was originally sent for more than 24 hours.
    pub members_only: bool,
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
    pub close_date: Option<Integer>,
    /// A list of two-letter ISO 3166-1 alpha-2 country codes
    /// indicating the countries from which users can vote in the poll.
    ///
    /// The country code “FT” is used for users with anonymous numbers.
    /// If omitted, then users from any country can participate in the poll.
    pub country_codes: Option<Vec<String>>,
    /// Description of the poll.
    ///
    /// For a poll inside the message object only.
    #[serde(
        flatten,
        deserialize_with = "PollDescription::deserialize_value",
        serialize_with = "PollDescription::serialize_value"
    )]
    pub description: Option<Text>,
    /// Media added to the poll description; for polls inside the Message object only.
    pub media: Option<PollMedia>,
    /// Amount of time in seconds the poll will be active after creation.
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
            allows_revoting: false,
            id: id.into(),
            is_anonymous: false,
            is_closed: false,
            members_only: false,
            options: vec![],
            question: question.into(),
            total_voter_count: 0,
            close_date: None,
            country_codes: None,
            description: None,
            media: None,
            open_period: None,
        }
    }

    /// Sets a new value for the `allows_multiple_answers` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll allows multiple answers.
    pub fn with_allows_multiple_answers(mut self, value: bool) -> Self {
        self.allows_multiple_answers = value;
        self
    }

    /// Sets a new value for the `allows_revoting` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the poll allows to change the chosen answer options.
    pub fn with_allows_revoting(mut self, value: bool) -> Self {
        self.allows_revoting = value;
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

    /// Sets a new list of country codes.
    ///
    /// # Arguments
    ///
    /// * `value` - ISO-3166-1 alpha-2 country codes.
    pub fn with_country_codes<A, B>(mut self, value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        self.country_codes = Some(value.into_iter().map(Into::into).collect());
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the poll.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.description = Some(value.into());
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

    /// Sets a new media.
    ///
    /// # Arguments
    ///
    /// * `value` - Media added to the poll description.
    pub fn with_media(mut self, value: PollMedia) -> Self {
        self.media = Some(value);
        self
    }

    /// Sets a new value for the `members_only` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether voting is limited to the chat members.
    pub fn with_members_only(mut self, value: bool) -> Self {
        self.members_only = value;
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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Quiz {
    /// Whether the poll allows to change the chosen answer options.
    pub allows_revoting: bool,
    /// Unique identifier of the quiz.
    pub id: String,
    /// Indicates whether the quiz is anonymous.
    pub is_anonymous: bool,
    /// Indicates whether the quiz is closed.
    pub is_closed: bool,
    /// Indicates whether voting is limited to users
    /// who have been members of the chat where
    /// the poll was originally sent for more than 24 hours.
    pub members_only: bool,
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
    pub close_date: Option<Integer>,
    /// Array of 0-based identifiers of the correct answer options.
    ///
    /// Available only for polls in quiz mode which are closed
    /// or were sent (not forwarded) by the bot or to the private chat with the bot.
    pub correct_option_ids: Option<Vec<Integer>>,
    /// A list of two-letter ISO 3166-1 alpha-2 country codes
    /// indicating the countries from which users can vote in the poll.
    ///
    /// The country code “FT” is used for users with anonymous numbers.
    /// If omitted, then users from any country can participate in the poll.
    pub country_codes: Option<Vec<String>>,
    /// Description of the quiz.
    ///
    /// For a quiz inside the message object only.
    #[serde(
        flatten,
        deserialize_with = "PollDescription::deserialize_value",
        serialize_with = "PollDescription::serialize_value"
    )]
    pub description: Option<Text>,
    /// Text that is shown when a user chooses an incorrect answer or
    /// taps on the lamp icon; 0-200 characters.
    #[serde(
        flatten,
        deserialize_with = "QuizExplanation::deserialize_value",
        serialize_with = "QuizExplanation::serialize_value"
    )]
    pub explanation: Option<Text>,
    /// Media added to the quiz explanation.
    pub explanation_media: Option<PollMedia>,
    /// Amount of time in seconds the quiz will be active after creation.
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
            allows_revoting: false,
            id: id.into(),
            is_anonymous: false,
            is_closed: false,
            members_only: true,
            options: vec![],
            question: question.into(),
            total_voter_count: 0,
            close_date: None,
            correct_option_ids: None,
            country_codes: None,
            description: None,
            explanation: None,
            explanation_media: None,
            open_period: None,
        }
    }

    /// Sets a new value for the `allows_revoting` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the poll allows to change the chosen answer options.
    pub fn with_allows_revoting(mut self, value: bool) -> Self {
        self.allows_revoting = value;
        self
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

    /// Sets a new list of correct option IDs.
    ///
    /// # Arguments
    ///
    /// * `value` - 0-based identifiers of the correct answer options.
    pub fn with_correct_option_ids<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = Integer>,
    {
        self.correct_option_ids = Some(value.into_iter().collect());
        self
    }

    /// Sets a new list of country codes.
    ///
    /// # Arguments
    ///
    /// * `value` - ISO-3166-1 alpha-2 country codes.
    pub fn with_country_codes<A, B>(mut self, value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        self.country_codes = Some(value.into_iter().map(Into::into).collect());
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the quiz.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.description = Some(value.into());
        self
    }

    /// Sets a new explanation.
    ///
    /// # Arguments
    ///
    /// * `value` - Text that is shown when a user chooses
    ///   an incorrect answer or
    ///   taps on the lamp icon; 0-200 characters.
    pub fn with_explanation<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.explanation = Some(value.into());
        self
    }

    /// Sets a new explanation media.
    ///
    /// # Arguments
    ///
    /// * `value` - Media added to the quiz explanation.
    pub fn with_explanation_media(mut self, value: PollMedia) -> Self {
        self.explanation_media = Some(value);
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

    /// Sets a new value for the `members_only` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether voting is limited to the chat members.
    pub fn with_members_only(mut self, value: bool) -> Self {
        self.members_only = value;
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

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize)]
struct PollDescription {
    description: String,
    description_entities: Option<TextEntities>,
}

impl PollDescription {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<Option<Text>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<PollDescription>::deserialize(deserializer).map(|x| {
            x.map(|value| Text {
                data: value.description,
                entities: value.description_entities,
            })
        })
    }

    fn serialize_value<S>(value: &Option<Text>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = value.clone().map(|value| PollDescription {
            description: value.data,
            description_entities: value.entities,
        });
        value.serialize(serializer)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize)]
struct QuizExplanation {
    explanation: String,
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

/// Represents the content of a poll description or a quiz explanation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PollMedia {
    /// Media is an animation.
    Animation(Animation),
    /// Media is an audio file.
    Audio(Audio),
    /// Media is a general file.
    Document(Document),
    /// Media is a live photo.
    LivePhoto(LivePhoto),
    /// Media is a shared location.
    Location(Location),
    /// Media is a photo.
    Photo(Vec<PhotoSize>),
    /// Media is a sticker.
    Sticker(Sticker),
    /// Media is a venue.
    Venue(Venue),
    /// Media is a video.
    Video(Video),
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize)]
struct RawPollOptionText {
    text: String,
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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PollOption {
    /// Unique identifier of the option, persistent on option addition and deletion.
    pub persistent_id: String,
    /// Option text; 1-100 characters.
    #[serde(
        flatten,
        deserialize_with = "RawPollOptionText::deserialize_value",
        serialize_with = "RawPollOptionText::serialize_value"
    )]
    pub text: Text,
    /// Number of users that voted for this option.
    pub voter_count: Integer,
    /// Chat that added the option.
    ///
    /// Omitted if the option wasn't added by a chat after poll creation.
    pub added_by_chat: Option<Chat>,
    /// User who added the option.
    ///
    /// Omitted if the option wasn't added by a user after poll creation.
    pub added_by_user: Option<User>,
    /// Point in time (Unix timestamp) when the option was added.
    ///
    /// Omitted if the option existed in the original poll.
    pub addition_date: Option<Integer>,
    /// Media added to the poll option.
    pub media: Option<PollMedia>,
}

impl PollOption {
    /// Creates a new `PollOption`.
    ///
    /// # Arguments
    ///
    /// * `persistent_id` - Unique identifier of the option.
    /// * `text` - Option text; 1-100 characters.
    /// * `voter_count` - Number of users that voted for this option.
    pub fn new<A, B>(persistent_id: A, text: B, voter_count: Integer) -> Self
    where
        A: Into<String>,
        B: Into<Text>,
    {
        Self {
            persistent_id: persistent_id.into(),
            text: text.into(),
            voter_count,
            added_by_chat: None,
            added_by_user: None,
            addition_date: None,
            media: None,
        }
    }

    /// Sets a new chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat that added the option.
    pub fn with_added_by_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.added_by_chat = Some(value.into());
        self
    }

    /// Sets a new user.
    ///
    /// # Arguments
    ///
    /// * `value` - User who added the option.
    pub fn with_added_by_user(mut self, value: User) -> Self {
        self.added_by_user = Some(value);
        self
    }

    /// Sets a new addition date.
    ///
    /// # Arguments
    ///
    /// * `value` - Point in time when the option was added.
    pub fn with_addition_date(mut self, value: Integer) -> Self {
        self.addition_date = Some(value);
        self
    }

    /// Sets a new media.
    ///
    /// # Arguments
    ///
    /// * `value` - Media added to the option.
    pub fn with_media(mut self, value: PollMedia) -> Self {
        self.media = Some(value);
        self
    }
}

/// Represents an answer of a user in a non-anonymous poll.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PollAnswer {
    /// 0-based identifiers of answer options, chosen by the user.
    ///
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<Integer>,
    /// Persistent identifiers of the chosen answer options.
    ///
    /// May be empty if the vote was retracted.
    pub option_persistent_ids: Vec<String>,
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
            option_persistent_ids: vec![],
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
#[derive(Debug)]
pub struct InputPollOption {
    data: InputPollOptionData,
    media_form: Option<Form>,
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
            data: InputPollOptionData {
                text: text.into(),
                text_parse_mode: None,
                text_entities: None,
                media: None,
            },
            media_form: None,
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
        self.data.text_entities = Some(value.into_iter().collect());
        self.data.text_parse_mode = None;
        self
    }

    /// Sets a new media.
    ///
    /// # Arguments
    ///
    /// * `value` - Media added to the poll option.
    pub fn with_media(mut self, value: InputMedia) -> Self {
        let (form, data) = value.into_parts();
        self.data.media = Some(data);
        self.media_form = Some(form);
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
        self.data.text_parse_mode = Some(value);
        self.data.text_entities = None;
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
            data: InputPollOptionData {
                text: value.data,
                text_entities: value.entities,
                text_parse_mode: None,
                media: None,
            },
            media_form: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
struct InputPollOptionData {
    text: String,
    text_parse_mode: Option<ParseMode>,
    text_entities: Option<TextEntities>,
    media: Option<InputMediaData>,
}

#[derive(Debug)]
struct PollParameters {
    form: Form,
    allow_adding_options: bool,
    is_anonymous: bool,
}

impl PollParameters {
    fn new<A, B>(chat_id: ChatId, question: String, poll_type: PollType, options: A) -> Result<Self, PollError>
    where
        A: IntoIterator<Item = B>,
        B: Into<InputPollOption>,
    {
        let mut form = Form::from([
            ("chat_id", chat_id.into()),
            ("question", question.into()),
            ("type", poll_type.into()),
        ]);
        let mut options_data = Vec::with_capacity(20);
        for (idx, option) in options.into_iter().map(Into::into).enumerate() {
            if let Some(option_form) = option.media_form {
                form.extend(option_form.with_suffix(format!("{idx}")))
            }
            options_data.push(option.data);
        }
        form.insert_field(
            "options",
            serde_json::to_string(&options_data).map_err(PollError::SerializeOptions)?,
        );
        Ok(Self {
            form,
            allow_adding_options: false,
            is_anonymous: false,
        })
    }

    fn set_allow_adding_options(&mut self, value: bool) {
        self.form.insert_field("allow_adding_options", value);
        self.allow_adding_options = value;
        if value && self.is_anonymous {
            self.form.remove_field("is_anonymous");
            self.is_anonymous = false;
        }
    }

    fn set_description_entities<T>(&mut self, value: T) -> Result<(), TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value: TextEntities = value.into_iter().collect();
        self.form.insert_field("description_entities", value.serialize()?);
        self.form.remove_field("description_parse_mode");
        Ok(())
    }

    fn set_description_parse_mode(&mut self, value: ParseMode) {
        self.form.insert_field("description_parse_mode", value);
        self.form.remove_field("description_entities");
    }

    fn set_is_anonymous(&mut self, value: bool) {
        self.is_anonymous = value;
        self.form.insert_field("is_anonymous", value);
        if value && self.allow_adding_options {
            self.allow_adding_options = false;
            self.form.remove_field("allow_adding_options")
        }
    }
}

/// Sends a quiz.
///
/// On success, the sent [`Message`] is returned.
#[derive(Debug)]
pub struct SendQuiz {
    inner: PollParameters,
}

impl SendQuiz {
    /// Creates a new `SendQuiz`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `question` - Question; 1-300 characters.
    /// * `correct_option_ids` - 0-based identifiers of the correct answer options.
    /// * `options` - Answer options; 2-12.
    pub fn new<A, B, C, D, DI>(chat_id: A, question: B, correct_option_ids: C, options: D) -> Result<Self, PollError>
    where
        A: Into<ChatId>,
        B: Into<String>,
        C: IntoIterator<Item = Integer>,
        D: IntoIterator<Item = DI>,
        DI: Into<InputPollOption>,
    {
        let mut parameters = PollParameters::new(chat_id.into(), question.into(), PollType::Quiz, options)?;
        let correct_option_ids: Vec<Integer> = correct_option_ids.into_iter().collect();
        let correct_option_ids_data =
            serde_json::to_string(&correct_option_ids).map_err(PollError::SerializeCorrectOptionIds)?;
        parameters
            .form
            .insert_field("correct_option_ids", correct_option_ids_data);
        Ok(Self { inner: parameters })
    }

    /// Sets a new value for the `allow_adding_options` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether answer options can be added to the poll after creation.
    ///
    /// `is_anonymous` will be set to [`None`] when it is set to `true` and the value is `true`.
    pub fn with_allow_adding_options(mut self, value: bool) -> Self {
        self.inner.set_allow_adding_options(value);
        self
    }

    /// Sets a new value for the `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///   for a fee of 0.1 Telegram Stars per message.
    ///   The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.inner.form.insert_field("allow_paid_broadcast", value);
        self
    }

    /// Sets a new value for the `allows_multiple_answers` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll allows multiple answers; default - `false`.
    pub fn with_allows_multiple_answers(mut self, value: bool) -> Self {
        self.inner.form.insert_field("allows_multiple_answers", value);
        self
    }

    /// Sets a new value for the `allows_revoting` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether if the poll allows to change chosen answer options,
    ///   defaults to false.
    pub fn with_allows_revoting(mut self, value: bool) -> Self {
        self.inner.form.insert_field("allows_revoting", value);
        self
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
        self.inner.form.insert_field("business_connection_id", value.into());
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
        self.inner.form.insert_field("close_date", value);
        self.inner.form.remove_field("open_period");
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Description; 0-1024 characters after entities parsing.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.form.insert_field("description", value.into());
        self
    }

    /// Sets a new list of description entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the description.
    ///
    /// Parse mode will be set to [`None`].
    pub fn with_description_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.inner.set_description_entities(value)?;
        Ok(self)
    }

    /// Sets a new description parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the description.
    ///
    /// Entities will be set to [`None`].
    pub fn with_description_parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.set_description_parse_mode(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.inner.form.insert_field("disable_notification", value);
        self
    }

    /// Sets a new explanation.
    ///
    /// # Arguments
    ///
    /// * `value` - Text that is shown when a user chooses
    ///   an incorrect answer or taps on the lamp icon;
    ///   0-200 characters with at most 2 line feeds after entities parsing.
    pub fn with_explanation<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.form.insert_field("explanation", value.into());
        self
    }

    /// Sets a new explanation media.
    ///
    /// # Arguments
    ///
    /// * `value` - Media added to the quiz explanation.
    pub fn with_explanation_media(mut self, value: InputMedia) -> Result<Self, InputMediaError> {
        self.inner.form.extend(value.try_into_form("explanation_media")?);
        Ok(self)
    }

    /// Sets a new list of explanation entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the quiz explanation.
    ///
    /// Explanation parse mode will be removed when this method is called.
    pub fn with_explanation_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value: TextEntities = value.into_iter().collect();
        self.inner.form.insert_field("explanation_entities", value.serialize()?);
        self.inner.form.remove_field("explanation_parse_mode");
        Ok(self)
    }

    /// Sets a new explanation parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the explanation.
    ///
    /// Explanation entities will be removed when this method is called.
    pub fn with_explanation_parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.form.insert_field("explanation_parse_mode", value);
        self.inner.form.remove_field("explanation_entities");
        self
    }

    /// Sets a new value for the `hide_results_until_closes` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the poll results must be shown only after the poll closes.
    pub fn with_hide_results_until_closes(mut self, value: bool) -> Self {
        self.inner.form.insert_field("hide_results_until_closes", value);
        self
    }

    /// Sets a new value for the `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the quiz needs to be anonymous; default - `true`.
    ///
    /// `allow_adding_options` will be set to [`None`] if the `value` is `true`.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.inner.set_is_anonymous(value);
        self
    }

    /// Sets a new value for the `is_closed` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the quiz needs to be immediately closed.
    pub fn with_is_closed(mut self, value: bool) -> Self {
        self.inner.form.insert_field("is_closed", value);
        self
    }

    /// Sets a new value for the `members_only` flag.
    ///
    /// # Arguments
    ///
    /// * `value` -  Whether voting is limited to users
    ///   who have been members of the chat where the poll
    ///   is being sent for more than 24 hours; for channel chats only
    pub fn with_members_only(mut self, value: bool) -> Self {
        self.inner.form.insert_field("members_only", value);
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
        self.inner.form.insert_field("message_effect_id", value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.inner.form.insert_field("message_thread_id", value);
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
        self.inner.form.insert_field("open_period", value);
        self.inner.form.remove_field("close_date");
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.inner.form.insert_field("protect_content", value);
        self
    }

    /// Sets a new list of question entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the poll question.
    ///
    /// Question parse mode will be removed when this method is called.
    pub fn with_question_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value: TextEntities = value.into_iter().collect();
        self.inner.form.insert_field("question_entities", value.serialize()?);
        self.inner.form.remove_field("question_parse_mode");
        Ok(self)
    }

    /// Sets a new question parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the question.
    ///
    /// Question entities will be removed when this method is called.
    pub fn with_question_parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.form.insert_field("question_parse_mode", value);
        self.inner.form.remove_field("question_entities");
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Result<Self, ReplyMarkupError>
    where
        T: Into<ReplyMarkup>,
    {
        self.inner.form.insert_field("reply_markup", value.into().serialize()?);
        Ok(self)
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Result<Self, ReplyParametersError> {
        self.inner.form.insert_field("reply_parameters", value.serialize()?);
        Ok(self)
    }

    /// Sets a new value for the `shuffle_options` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the poll options must be shown in random order.
    pub fn with_shuffle_options(mut self, value: bool) -> Self {
        self.inner.form.insert_field("shuffle_options", value);
        self
    }
}

impl Method for SendQuiz {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendPoll", self.inner.form)
    }
}

/// Sends a native poll.
///
/// On success, the sent [`Message`] is returned.
#[derive(Debug)]
pub struct SendPoll {
    inner: PollParameters,
}

impl SendPoll {
    /// Creates a new `SendPoll`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `question` - Question; 1-300 characters.
    /// * `options` - Answer options; 2-12.
    pub fn new<A, B, C, D>(chat_id: A, question: B, options: C) -> Result<Self, PollError>
    where
        A: Into<ChatId>,
        B: Into<String>,
        C: IntoIterator<Item = D>,
        D: Into<InputPollOption>,
    {
        Ok(Self {
            inner: PollParameters::new(chat_id.into(), question.into(), PollType::Regular, options)?,
        })
    }

    /// Sets a new value for the `allow_adding_options` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether answer options can be added to the poll after creation.
    ///
    /// `is_anonymous` will be set to [`None`] when it is set to `true` and the value is `true`.
    pub fn with_allow_adding_options(mut self, value: bool) -> Self {
        self.inner.set_allow_adding_options(value);
        self
    }

    /// Sets a new value for the `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///   for a fee of 0.1 Telegram Stars per message.
    ///   The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.inner.form.insert_field("allow_paid_broadcast", value);
        self
    }

    /// Sets a new value for the `allows_multiple_answers` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll allows multiple answers; default - `false`.
    pub fn with_allows_multiple_answers(mut self, value: bool) -> Self {
        self.inner.form.insert_field("allows_multiple_answers", value);
        self
    }

    /// Sets a new value for the `allows_revoting` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether if the poll allows to change chosen answer options,
    ///   defaults to True.
    pub fn with_allows_revoting(mut self, value: bool) -> Self {
        self.inner.form.insert_field("allows_revoting", value);
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
        self.inner.form.insert_field("business_connection_id", value.into());
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
        self.inner.form.insert_field("close_date", value);
        self.inner.form.remove_field("open_period");
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Description; 0-1024 characters after entities parsing.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.form.insert_field("description", value.into());
        self
    }

    /// Sets a new list of description entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the description.
    ///
    /// Parse mode will be set to [`None`].
    pub fn with_description_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.inner.set_description_entities(value)?;
        Ok(self)
    }

    /// Sets a new description parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the description.
    ///
    /// Entities will be set to [`None`].
    pub fn with_description_parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.set_description_parse_mode(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.inner.form.insert_field("disable_notification", value);
        self
    }

    /// Sets a new value for the `hide_results_until_closes` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the poll results must be shown only after the poll closes.
    pub fn with_hide_results_until_closes(mut self, value: bool) -> Self {
        self.inner.form.insert_field("hide_results_until_closes", value);
        self
    }

    /// Sets a new value for the `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll needs to be anonymous; default - `true`.
    ///
    /// `allow_adding_options` will be set to [`None`] if the `value` is `true`.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.inner.set_is_anonymous(value);
        self
    }

    /// Sets a new value for the `is_closed` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the poll needs to be immediately closed.
    pub fn with_is_closed(mut self, value: bool) -> Self {
        self.inner.form.insert_field("is_closed", value);
        self
    }

    /// Sets a new media.
    ///
    /// # Arguments
    ///
    /// * `value` - Media added to the poll description.
    pub fn with_media(mut self, value: InputMedia) -> Result<Self, InputMediaError> {
        self.inner.form.extend(value.try_into_form("media")?);
        Ok(self)
    }

    /// Sets a new value for the `members_only` flag.
    ///
    /// # Arguments
    ///
    /// * `value` -  Whether voting is limited to users
    ///   who have been members of the chat where the poll
    ///   is being sent for more than 24 hours; for channel chats only
    pub fn with_members_only(mut self, value: bool) -> Self {
        self.inner.form.insert_field("members_only", value);
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
        self.inner.form.insert_field("message_effect_id", value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.inner.form.insert_field("message_thread_id", value);
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
        self.inner.form.insert_field("open_period", value);
        self.inner.form.remove_field("close_date");
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.inner.form.insert_field("protect_content", value);
        self
    }

    /// Sets a new list of question entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the poll question.
    ///
    /// Question parse mode will be set to [`None`] when this method is called.
    pub fn with_question_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value: TextEntities = value.into_iter().collect();
        self.inner.form.insert_field("question_entities", value.serialize()?);
        self.inner.form.remove_field("question_parse_mode");
        Ok(self)
    }

    /// Sets a new question parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the question.
    ///
    /// Question entities will be set to [`None`] when this method is called.
    pub fn with_question_parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.form.insert_field("question_parse_mode", value);
        self.inner.form.remove_field("question_entities");
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Result<Self, ReplyMarkupError>
    where
        T: Into<ReplyMarkup>,
    {
        self.inner.form.insert_field("reply_markup", value.into().serialize()?);
        Ok(self)
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Result<Self, ReplyParametersError> {
        self.inner.form.insert_field("reply_parameters", value.serialize()?);
        Ok(self)
    }

    /// Sets a new value for the `shuffle_options` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the poll options must be shown in random order.
    pub fn with_shuffle_options(mut self, value: bool) -> Self {
        self.inner.form.insert_field("shuffle_options", value);
        self
    }
}

impl Method for SendPoll {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendPoll", self.inner.form)
    }
}

/// Stops a poll which was sent by the bot.
///
/// On success, the stopped [`Poll`] with the final results is returned.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct StopPoll {
    chat_id: ChatId,
    message_id: Integer,
    business_connection_id: Option<String>,
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
            business_connection_id: None,
            reply_markup: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the message to be edited was sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
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

/// A poll error.
#[derive(Debug)]
pub enum PollError {
    /// Failed to serialize correct option IDs.
    SerializeCorrectOptionIds(serde_json::Error),
    /// Failed to serialize options.
    SerializeOptions(serde_json::Error),
}

impl Error for PollError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::SerializeCorrectOptionIds(err) => err,
            Self::SerializeOptions(err) => err,
        })
    }
}

impl fmt::Display for PollError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SerializeCorrectOptionIds(err) => write!(out, "can not serialize correct option ids: {}", err),
            Self::SerializeOptions(err) => write!(out, "can not serialize options: {}", err),
        }
    }
}

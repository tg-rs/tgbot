use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    api::{Method, Payload},
    types::{
        Animation,
        EditMessageResult,
        InlineKeyboardMarkup,
        Integer,
        Message,
        PhotoSize,
        ReplyParameters,
        Text,
        TextEntities,
        User,
    },
};

/// Represents a Game.
///
/// Use BotFather to create and edit games,
/// their short names will act as unique identifiers.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Game {
    /// Description of the game.
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Title of the game.
    pub title: String,
    /// Animation that will be displayed in the game message in chats.
    ///
    /// Upload via BotFather.
    pub animation: Option<Animation>,
    /// Brief description or high scores included in the game message; 0-4096 characters.
    ///
    /// Can be automatically edited to include current high scores for
    /// when the bot calls [`SetGameScore`],
    /// or manually edited using [`crate::types::EditMessageText`].
    #[serde(
        flatten,
        deserialize_with = "GameText::deserialize_value",
        serialize_with = "GameText::serialize_value"
    )]
    pub text: Option<Text>,
}

impl Game {
    /// Creates a new `Game`.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the game.
    /// * `photo` - Photo of the game.
    /// * `title` - Title of the game.
    pub fn new<A, B, C>(description: A, photo: B, title: C) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = PhotoSize>,
        C: Into<String>,
    {
        Self {
            description: description.into(),
            photo: photo.into_iter().collect(),
            title: title.into(),
            animation: None,
            text: None,
        }
    }

    /// Sets a new animation.
    ///
    /// # Arguments
    ///
    /// * `value` - Animation that will be displayed in the game message in chats.
    pub fn with_animation(mut self, value: Animation) -> Self {
        self.animation = Some(value);
        self
    }

    /// Sets a new text.
    ///
    /// # Arguments
    ///
    /// * `value` - Brief description or high scores included in the game message;
    ///   0-4096 characters.
    pub fn with_text<T>(mut self, value: T) -> Self
    where
        T: Into<Text>,
    {
        self.text = Some(value.into());
        self
    }
}

#[derive(Deserialize, Serialize)]
struct GameText {
    text: String,
    text_entities: Option<TextEntities>,
}

impl GameText {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<Option<Text>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<GameText>::deserialize(deserializer).map(|wrapper| {
            wrapper.map(
                |GameText {
                     text: data,
                     text_entities: entities,
                 }| Text { data, entities },
            )
        })
    }

    fn serialize_value<S>(value: &Option<Text>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = value.clone().map(|value| GameText {
            text: value.data,
            text_entities: value.entities,
        });
        value.serialize(serializer)
    }
}

/// Represents a row of the high scores table for a game.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct GameHighScore {
    /// Position in the high score table.
    pub position: Integer,
    /// Score achieved by the user.
    pub score: Integer,
    /// User associated with the high score.
    pub user: User,
}

impl GameHighScore {
    /// Creates a new `GameHighScore`.
    ///
    /// # Arguments
    ///
    /// * `position` - Position in the high score table.
    /// * `score` - Score achieved by the user.
    /// * `user` - User associated with the high score.
    pub fn new(position: Integer, score: Integer, user: User) -> Self {
        Self { position, score, user }
    }
}

/// Returns data for high score tables.
///
/// Will return the score of the specified user and several of his neighbors in a game.
/// This method will currently return scores for the target user,
/// plus two of his closest neighbors on each side.
/// Will also return the top three users if the user and his neighbors are not among them.
/// Please note that this behavior is subject to change.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct GetGameHighScores {
    user_id: Integer,
    chat_id: Option<Integer>,
    inline_message_id: Option<String>,
    message_id: Option<Integer>,
}

impl GetGameHighScores {
    /// Creates a new `GetGameHighScores` for a chat message.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Target user ID.
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `message_id` - Identifier of the sent message.
    pub fn for_chat_message(user_id: Integer, chat_id: Integer, message_id: Integer) -> Self {
        Self {
            user_id,
            chat_id: Some(chat_id),
            inline_message_id: None,
            message_id: Some(message_id),
        }
    }

    /// Creates a new `GetGameHighScores` for an inline message.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Target user ID.
    /// * `inline_message_id` - Identifier of the inline message.
    pub fn for_inline_message<T>(user_id: Integer, inline_message_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            user_id,
            chat_id: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
        }
    }
}

impl Method for GetGameHighScores {
    type Response = Vec<GameHighScore>;

    fn into_payload(self) -> Payload {
        Payload::json("getGameHighScores", self)
    }
}

/// Sends a game.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendGame {
    chat_id: Integer,
    game_short_name: String,
    allow_paid_broadcast: Option<bool>,
    business_connection_id: Option<String>,
    disable_notification: Option<bool>,
    message_effect_id: Option<String>,
    message_thread_id: Option<Integer>,
    protect_content: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
    reply_parameters: Option<ReplyParameters>,
}

impl SendGame {
    /// Creates a new `SendGame`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `game_short_name` - Short name of the game, serves as the unique identifier for the game.
    pub fn new<T>(chat_id: Integer, game_short_name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            chat_id,
            game_short_name: game_short_name.into(),
            allow_paid_broadcast: None,
            business_connection_id: None,
            disable_notification: None,
            message_effect_id: None,
            message_thread_id: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
        }
    }

    /// Sets a new value for the `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///   for a fee of 0.1 Telegram Stars per message.
    ///   The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.allow_paid_broadcast = Some(value);
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
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
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
        self.message_effect_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
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

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.reply_parameters = Some(value);
        self
    }
}

impl Method for SendGame {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendGame", self)
    }
}

/// Sets the score of the specified user in a game.
///
/// Returns an error, if the new score is not greater
/// than the user's current score in the chat and force is `false`.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetGameScore {
    user_id: Integer,
    score: Integer,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
    chat_id: Option<Integer>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
}

impl SetGameScore {
    /// Creates a new `SetGameScore`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the sent message.
    /// * `user_id` - User identifier.
    /// * `score` - New score, must be non-negative.
    pub fn for_chat_message(chat_id: Integer, message_id: Integer, user_id: Integer, score: Integer) -> Self {
        Self {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
        }
    }

    /// Creates a new `SetGameScore`.
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message.
    /// * `user_id` - User identifier.
    /// * `score` - New score, must be non-negative.
    pub fn for_inline_message<T>(inline_message_id: T, user_id: Integer, score: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
        }
    }

    /// Sets a new value for the `disable_edit_message` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the game message should not be automatically
    ///   edited to include the current scoreboard.
    pub fn with_disable_edit_message(mut self, value: bool) -> Self {
        self.disable_edit_message = Some(value);
        self
    }

    /// Sets a new value for the `force` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the high score is allowed to decrease.
    ///
    /// This can be useful when fixing mistakes or banning cheaters.
    pub fn with_force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }
}

impl Method for SetGameScore {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("setGameScore", self)
    }
}

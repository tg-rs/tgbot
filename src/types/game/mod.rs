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
        Text,
        TextEntities,
        User,
    },
};

#[cfg(test)]
mod tests;

/// Game
///
/// Use BotFather to create and edit games,
/// their short names will act as unique identifiers
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game message
    /// Can be automatically edited to include current high scores for the game
    /// when the bot calls setGameScore, or manually edited using editMessageText
    /// 0-4096 characters
    #[serde(flatten)]
    #[serde(deserialize_with = "GameText::deserialize_value")]
    #[serde(serialize_with = "GameText::serialize_value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    /// Animation that will be displayed in the game message in chats
    /// Upload via BotFather
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
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

/// One row of the high scores table for a game
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: Integer,
    /// User
    pub user: User,
    /// Score
    pub score: Integer,
}

/// Get data for high score tables
///
/// Will return the score of the specified user and several of his neighbors in a game
/// This method will currently return scores for the target user,
/// plus two of his closest neighbors on each side
/// Will also return the top three users if the user and his neighbors are not among them
/// Please note that this behavior is subject to change
#[derive(Clone, Debug, Serialize)]
pub struct GetGameHighScores {
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
}

impl GetGameHighScores {
    /// Creates a new GetGameHighScores
    ///
    /// # Arguments
    ///
    /// * user_id - Target user id
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new(user_id: Integer, chat_id: Integer, message_id: Integer) -> Self {
        GetGameHighScores {
            user_id,
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
        }
    }

    /// Creates a new GetGameHighScores
    ///
    /// # Arguments
    ///
    /// * user_id - Target user id
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(user_id: Integer, inline_message_id: S) -> Self {
        GetGameHighScores {
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
        }
    }
}

impl Method for GetGameHighScores {
    type Response = Vec<GameHighScore>;

    fn into_payload(self) -> Payload {
        Payload::json("getGameHighScores", self)
    }
}

/// Use this method to send a game
#[derive(Clone, Debug, Serialize)]
pub struct SendGame {
    chat_id: Integer,
    game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
}

impl SendGame {
    /// Creates a new SendGame with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * game_short_name - Short name of the game, serves as the unique identifier for the game
    pub fn new<S: Into<String>>(chat_id: Integer, game_short_name: S) -> Self {
        SendGame {
            chat_id,
            game_short_name: game_short_name.into(),
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
            message_thread_id: None,
        }
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
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
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Unique identifier for the target message thread (topic) of the forum;
    /// for forum supergroups only
    pub fn message_thread_id(mut self, message_thread_id: Integer) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
}

impl Method for SendGame {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendGame", self)
    }
}

/// Set the score of the specified user in a game
///
/// Returns an error, if the new score is not greater
/// than the user's current score in the chat and force is False
#[derive(Clone, Debug, Serialize)]
pub struct SetGameScore {
    user_id: Integer,
    score: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
}

impl SetGameScore {
    /// Creates a new SetGameScore
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * user_id - User identifier
    /// * score - New score, must be non-negative
    pub fn new(chat_id: Integer, message_id: Integer, user_id: Integer, score: Integer) -> Self {
        SetGameScore {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
        }
    }

    /// Creates a new SetGameScore
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * user_id - User identifier
    /// * score - New score, must be non-negative
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, user_id: Integer, score: Integer) -> Self {
        SetGameScore {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
        }
    }

    /// Pass True, if the high score is allowed to decrease
    ///
    /// This can be useful when fixing mistakes or banning cheaters
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }

    /// Pass True, if the game message should not be automatically
    /// edited to include the current scoreboard
    pub fn disable_edit_message(mut self, disable_edit_message: bool) -> Self {
        self.disable_edit_message = Some(disable_edit_message);
        self
    }
}

impl Method for SetGameScore {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("setGameScore", self)
    }
}

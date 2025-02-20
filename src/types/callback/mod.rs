use std::{error::Error, fmt};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Error as JsonError;

use crate::{
    api::{Method, Payload},
    types::{Integer, MaybeInaccessibleMessage, User},
};

#[cfg(test)]
mod tests;

/// Represents an incoming callback query from a callback button in an inline keyboard.
///
/// If the button that originated the query was attached to a message sent by the bot,
/// the field message will be present.
///
/// If the button was attached to a message sent via the bot (in inline mode),
/// the field `inline_message_id` will be present.
///
/// Exactly one of the fields data or `game_short_name` will be present.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CallbackQuery {
    /// Sender of the query.
    pub from: User,
    /// Unique identifier of the query.
    pub id: String,
    /// Global identifier, uniquely corresponding
    /// to the chat to which the message with the
    /// callback button was sent.
    ///
    /// Useful for high scores in games.
    pub chat_instance: Option<String>,
    /// Data associated with the callback button.
    ///
    /// Be aware that a bad client can send arbitrary data in this field.
    pub data: Option<String>,
    /// Short name of a Game to be returned,
    /// serves as the unique identifier for the game.
    pub game_short_name: Option<String>,
    /// Identifier of the message sent via the bot
    /// in inline mode, that originated the query.
    pub inline_message_id: Option<String>,
    /// Message with the callback button that originated the query.
    ///
    /// Note that message content and message date
    /// will not be available if the message is too old.
    pub message: Option<MaybeInaccessibleMessage>,
}

impl CallbackQuery {
    /// Creates a new `CallbackQuery`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the query.
    /// * `from` - Sender of the query.
    pub fn new<T>(id: T, from: User) -> Self
    where
        T: Into<String>,
    {
        Self {
            from,
            id: id.into(),
            chat_instance: None,
            data: None,
            game_short_name: None,
            inline_message_id: None,
            message: None,
        }
    }

    /// Parses callback data using [`serde_json`].
    pub fn parse_data<T: DeserializeOwned>(&self) -> Result<Option<T>, CallbackQueryError> {
        if let Some(ref data) = self.data {
            serde_json::from_str(data)
                .map(Some)
                .map_err(CallbackQueryError::ParseJsonData)
        } else {
            Ok(None)
        }
    }

    /// Sets a new chat instance.
    ///
    /// # Arguments
    ///
    /// * `value` - Global identifier, uniquely corresponding to the chat.
    pub fn with_chat_instance<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.chat_instance = Some(value.into());
        self
    }

    /// Sets a new data for callback button.
    ///
    /// # Arguments
    ///
    /// * `value` - Data associated with the callback button.
    pub fn with_data<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data = Some(value.into());
        self
    }

    /// Sets a new short name for a game.
    ///
    /// # Arguments
    ///
    /// * `value` - Short name of the game.
    pub fn with_game_short_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.game_short_name = Some(value.into());
        self
    }

    /// Sets a new inline message ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the message sent via the bot in inline mode.
    pub fn with_inline_message_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = Some(value.into());
        self
    }

    /// Sets a new message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message with the callback button that originated the query.
    pub fn with_message(mut self, value: MaybeInaccessibleMessage) -> Self {
        self.message = Some(value);
        self
    }
}

/// Represents an error that can occur while parsing data from a callback query.
#[derive(Debug)]
pub enum CallbackQueryError {
    /// Failed to parse JSON data
    ParseJsonData(JsonError),
}

impl Error for CallbackQueryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseJsonData(err) => Some(err),
        }
    }
}

impl fmt::Display for CallbackQueryError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseJsonData(err) => write!(out, "failed to parse callback query data: {}", err),
        }
    }
}

/// Sends an answer to a callback query sent from an inline keyboard.
///
/// The answer will be displayed to the user as a notification at the top of the chat screen or as an alert.
///
/// Alternatively, the user can be redirected to the specified Game URL.
///
/// For this option to work, you must first create a game for your bot via Bot Father and accept the terms.
///
/// Otherwise, you may use links like `t.me/your_bot?start=XXX` that open your bot with a parameter.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct AnswerCallbackQuery {
    callback_query_id: String,
    cache_time: Option<Integer>,
    show_alert: Option<bool>,
    text: Option<String>,
    url: Option<String>,
}

impl AnswerCallbackQuery {
    /// Creates a new `AnswerCallbackQuery`.
    ///
    /// # Arguments
    ///
    /// * `callback_query_id` - Unique identifier of the query to be answered.
    pub fn new<T>(callback_query_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            callback_query_id: callback_query_id.into(),
            cache_time: None,
            show_alert: None,
            text: None,
            url: None,
        }
    }

    /// Sets a new cache time.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum amount of time in seconds that the result
    ///             of the callback query may be cached client-side;
    ///             telegram apps will support caching starting in version 3.14;
    ///             default - 0.
    pub fn with_cache_time(mut self, value: Integer) -> Self {
        self.cache_time = Some(value);
        self
    }

    /// Sets a new value for a `short_alert` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - An alert will be shown by the client instead
    ///             of a notification at the top of the chat screen;
    ///             default - `false`.
    pub fn with_show_alert(mut self, value: bool) -> Self {
        self.show_alert = Some(value);
        self
    }

    /// Sets a new text.
    ///
    /// # Arguments
    ///
    /// * `value` - Text of the notification;
    ///             if not specified, nothing will be shown to the user;
    ///             0-200 characters.
    pub fn with_text<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.text = Some(value.into());
        self
    }

    /// Sets a new URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL that will be opened by the user's client.
    ///
    /// If you have created a game and accepted the conditions via Bot Father,
    /// specify the URL that opens your game – note that this will only work
    /// if the query comes from a callback game button.
    ///
    /// Otherwise, you may use links like `t.me/your_bot?start=XXX`
    /// that open your bot with a parameter.
    pub fn with_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.url = Some(value.into());
        self
    }
}

impl Method for AnswerCallbackQuery {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("answerCallbackQuery", self)
    }
}

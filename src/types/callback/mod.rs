use std::{error::Error, fmt};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::{
    api::{Method, Payload},
    types::{Integer, Message, User},
};

#[cfg(test)]
mod tests;

/// Incoming callback query from a callback button in an inline keyboard
///
/// If the button that originated the query was attached to a message sent by the bot,
/// the field message will be present
/// If the button was attached to a message sent via the bot (in inline mode),
/// the field inline_message_id will be present
/// Exactly one of the fields data or game_short_name will be present
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Message with the callback button that originated the query
    /// Note that message content and message date
    /// will not be available if the message is too old
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// Identifier of the message sent via the bot
    /// in inline mode, that originated the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding
    /// to the chat to which the message with the
    /// callback button was sent
    /// Useful for high scores in games
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_instance: Option<String>,
    /// Data associated with the callback button.
    /// Be aware that a bad client can send arbitrary data in this field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Short name of a Game to be returned,
    /// serves as the unique identifier for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

impl CallbackQuery {
    /// Parses callback data using serde_json
    pub fn parse_data<T: DeserializeOwned>(&self) -> Result<Option<T>, CallbackQueryError> {
        if let Some(ref data) = self.data {
            serde_json::from_str(data)
                .map(Some)
                .map_err(CallbackQueryError::ParseJsonData)
        } else {
            Ok(None)
        }
    }
}

/// An error occurred in callback query
#[derive(Debug)]
pub enum CallbackQueryError {
    /// Failed to parse JSON data
    ParseJsonData(JsonError),
}

impl Error for CallbackQueryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CallbackQueryError::ParseJsonData(err) => Some(err),
        }
    }
}

impl fmt::Display for CallbackQueryError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CallbackQueryError::ParseJsonData(err) => write!(out, "failed to parse callback query data: {}", err),
        }
    }
}

/// Send answer to callback query sent from inline keyboard
///
/// The answer will be displayed to the user as a notification at the top of the chat screen or as an alert
/// Alternatively, the user can be redirected to the specified Game URL
/// For this option to work, you must first create a game for your bot via Bot Father and accept the terms
/// Otherwise, you may use links like t.me/your_bot?start=XXX that open your bot with a parameter
#[derive(Clone, Debug, Serialize)]
pub struct AnswerCallbackQuery {
    callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
}

impl AnswerCallbackQuery {
    /// Creates a new AnswerCallbackQuery with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * callback_query_id - Unique identifier for the query to be answered
    pub fn new<S: Into<String>>(callback_query_id: S) -> Self {
        AnswerCallbackQuery {
            callback_query_id: callback_query_id.into(),
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    /// Text of the notification
    ///
    /// If not specified, nothing will be shown to the user, 0-200 characters
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    /// An alert will be shown by the client instead of a notification at the top of the chat screen
    ///
    /// Defaults to false
    pub fn show_alert(mut self, show_alert: bool) -> Self {
        self.show_alert = Some(show_alert);
        self
    }

    /// URL that will be opened by the user's client
    ///
    /// If you have created a Game and accepted the conditions via Bot Father,
    /// specify the URL that opens your game – note that this will only work
    /// if the query comes from a callback_game button
    ///
    /// Otherwise, you may use links like t.me/your_bot?start=XXX that open your bot with a parameter
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side
    ///
    /// Telegram apps will support caching starting in version 3.14
    /// Defaults to 0
    pub fn cache_time(mut self, cache_time: Integer) -> Self {
        self.cache_time = Some(cache_time);
        self
    }
}

impl Method for AnswerCallbackQuery {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("answerCallbackQuery", self)
    }
}

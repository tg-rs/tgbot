use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::types::{ChatId, Integer, ParseMode, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// Describes reply parameters for the message that is being sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReplyParameters {
    message_id: Integer,
    allow_sending_without_reply: Option<bool>,
    chat_id: Option<ChatId>,
    #[serde(flatten)]
    quote: Option<ReplyQuote>,
}

impl ReplyParameters {
    /// Creates a new `ReplyParameters`.
    ///
    /// # Arguments
    ///
    /// * `message_id` - Identifier of the message that will be replied to in the current chat,
    ///   or in the chat chat_id if it is specified.
    pub fn new(message_id: Integer) -> Self {
        Self {
            message_id,
            allow_sending_without_reply: None,
            chat_id: None,
            quote: None,
        }
    }

    /// Sets a new value for the `allow_sending_without_reply` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message should be sent even if the specified message to be replied to is not found.
    ///
    /// Can be used only for replies in the same chat and forum topic.
    pub fn with_allow_sending_without_reply(mut self, value: bool) -> Self {
        self.allow_sending_without_reply = Some(value);
        self
    }

    /// Sets a new chat ID.
    ///
    /// # Arguments
    ///
    /// * `value` - If the message to be replied to is from a different chat, unique identifier for the chat.
    pub fn with_chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = Some(value.into());
        self
    }

    /// Sets a new quote.
    ///
    /// # Arguments
    ///
    /// * `value` - Quoted part of the message to be replied to.
    pub fn with_quote(mut self, value: ReplyQuote) -> Self {
        self.quote = Some(value);
        self
    }

    pub(crate) fn serialize(&self) -> Result<String, ReplyParametersError> {
        serde_json::to_string(self).map_err(ReplyParametersError::Serialize)
    }
}

/// Quoted part of the message to be replied to.
///
/// The text must contain 0-1024 characters after entities parsing.
/// The quote must be an exact substring of the message to be replied to,
/// including bold, italic, underline, strikethrough, spoiler, and custom_emoji entities.
/// The message will fail to send if the quote isn't found in the original message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReplyQuote {
    #[serde(rename = "quote_position")]
    position: Integer,
    #[serde(rename = "quote")]
    text: String,
    #[serde(rename = "quote_entities")]
    entities: Option<TextEntities>,
    #[serde(rename = "quote_parse_mode")]
    parse_mode: Option<ParseMode>,
}

impl ReplyQuote {
    /// Creates a new `ReplyQuote`.
    ///
    /// # Arguments
    ///
    /// * `position` - Position of the quote in the original message in UTF-16 code units.
    /// * `text` -  Quoted part of the message to be replied to.
    pub fn new<T>(position: Integer, text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            position,
            text: text.into(),
            entities: None,
            parse_mode: None,
        }
    }

    /// Sets a new list of entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the quote.
    ///
    /// It can be specified instead of parse mode.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the quote.
    ///
    /// It can be specified instead of entities.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.entities = None;
        self
    }
}

/// Represents an error occurred with reply markup.
#[derive(Debug)]
pub enum ReplyParametersError {
    /// Can not serialize markup
    Serialize(JsonError),
}

impl Error for ReplyParametersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReplyParametersError::Serialize(err) => Some(err),
        }
    }
}

impl fmt::Display for ReplyParametersError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplyParametersError::Serialize(err) => write!(out, "can not serialize reply parameters: {}", err),
        }
    }
}

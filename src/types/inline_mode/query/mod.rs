use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{InlineQueryResult, InlineQueryResultsButton, Integer, Location, User},
};

#[cfg(test)]
mod tests;

/// Incoming inline query
///
/// When the user sends an empty query, your bot could return some default or trending results
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Text of the query (up to 512 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// Type of the chat, from which the inline query was sent
    ///
    /// Can be either “sender” for a private chat with the inline query sender,
    /// “private”, “group”, “supergroup”, or “channel”.
    /// The chat type should be always known for requests sent from official
    /// clients and most third-party clients,
    /// unless the request was sent from a secret chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<InlineQueryChatType>,
    /// Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// Type of the chat, from which the inline query was sent
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InlineQueryChatType {
    /// Private chat with the inline query sender
    Sender,
    /// Private chat
    Private,
    /// Group
    Group,
    /// Supergroup
    Supergroup,
    /// Channel
    Channel,
}

/// Use this method to send answers to an inline query
///
/// No more than 50 results per query are allowed
#[derive(Clone, Debug, Serialize)]
pub struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<InlineQueryResultsButton>,
}

impl AnswerInlineQuery {
    /// Creates a new AnswerInlineQuery with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * inline_query_id - Unique identifier for the answered query
    /// * results - An array of results for the inline query
    pub fn new<S: Into<String>>(inline_query_id: S, results: Vec<InlineQueryResult>) -> Self {
        AnswerInlineQuery {
            inline_query_id: inline_query_id.into(),
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            button: None,
        }
    }

    /// Maximum amount of time in seconds that the result of the inline query may be cached on the server
    ///
    /// Defaults to 300
    pub fn cache_time(mut self, cache_time: Integer) -> Self {
        self.cache_time = Some(cache_time);
        self
    }

    /// Cache results on the server side only for the user that sent the query
    ///
    /// By default, results may be returned to any user who sends the same query
    pub fn personal(mut self, is_personal: bool) -> Self {
        self.is_personal = Some(is_personal);
        self
    }

    /// Offset that a client should send in the next query with the same text to receive more results
    ///
    /// Pass an empty string if there are no more results or if you don‘t support pagination
    /// Offset length can’t exceed 64 bytes
    pub fn next_offset<S: Into<String>>(mut self, next_offset: S) -> Self {
        self.next_offset = Some(next_offset.into());
        self
    }

    /// An object describing a button to be shown above inline query results
    pub fn button(mut self, value: InlineQueryResultsButton) -> Self {
        self.button = Some(value);
        self
    }
}

impl Method for AnswerInlineQuery {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("answerInlineQuery", self)
    }
}

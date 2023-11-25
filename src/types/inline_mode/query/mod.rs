use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{InlineQueryResult, InlineQueryResultsButton, Integer, Location, User},
};

#[cfg(test)]
mod tests;

/// Represents an incoming inline query.
///
/// When the user sends an empty query, your bot could return some default or trending results.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InlineQuery {
    /// Sender of the query.
    pub from: User,
    /// Unique identifier of the query.
    pub id: String,
    /// Offset of the results to be returned, can be controlled by the bot.
    pub offset: String,
    /// Text of the query; up to 256 characters.
    pub query: String,
    /// Type of the chat, from which the inline query was sent.
    ///
    /// Can be either “sender” for a private chat with the inline query sender,
    /// “private”, “group”, “supergroup”, or “channel”.
    /// The chat type should be always known for requests sent from official
    /// clients and most third-party clients,
    /// unless the request was sent from a secret chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<InlineQueryChatType>,
    /// Sender location, only for bots that request user location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

impl InlineQuery {
    /// Creates a new `InlineQuery`.
    ///
    /// # Arguments
    ///
    /// * `from` - Sender of the query.
    /// * `id` - Unique identifier of the query.
    /// * `offset` - Offset of the results.
    /// * `query` - Text of the query; up to 256 characters.
    pub fn new<A, B, C>(from: User, id: A, offset: B, query: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            from,
            id: id.into(),
            offset: offset.into(),
            query: query.into(),
            chat_type: None,
            location: None,
        }
    }

    /// Sets a new chat type.
    ///
    /// # Arguments
    ///
    /// * `value` - Type of the chat, from which the inline query was sent.
    pub fn with_chat_type(mut self, value: InlineQueryChatType) -> Self {
        self.chat_type = Some(value);
        self
    }

    /// Sets a new location.
    ///
    /// # Arguments
    ///
    /// * `value` - Sender location.
    pub fn with_location(mut self, value: Location) -> Self {
        self.location = Some(value);
        self
    }
}

/// Represents a type of the chat, from which the inline query was sent.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InlineQueryChatType {
    /// A private chat with the sender of the inline query.
    Sender,
    /// A private chat.
    Private,
    /// A group chat.
    Group,
    /// A supergroup chat.
    Supergroup,
    /// A channel chat.
    Channel,
}

/// Sends an answer to an inline query.
///
/// No more than 50 results per query are allowed.
#[derive(Clone, Debug, Serialize)]
pub struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<InlineQueryResultsButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<String>,
}

impl AnswerInlineQuery {
    /// Creates a new `AnswerInlineQuery`.
    ///
    /// # Arguments
    ///
    /// * `inline_query_id` - Unique identifier of the answered query.
    /// * `results` - An array of results.
    pub fn new<A, B>(inline_query_id: A, results: B) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = InlineQueryResult>,
    {
        Self {
            inline_query_id: inline_query_id.into(),
            results: results.into_iter().collect(),
            button: None,
            cache_time: None,
            is_personal: None,
            next_offset: None,
        }
    }

    /// Sets a new button.
    ///
    /// # Arguments
    ///
    /// * `value` - An object describing a button to be shown above inline query results.
    pub fn with_button(mut self, value: InlineQueryResultsButton) -> Self {
        self.button = Some(value);
        self
    }

    /// Sets a new cache time.
    ///
    /// # Arguments
    ///
    /// * `value` - Maximum amount of time in seconds that the result
    ///             of the inline query may be cached on the server;
    ///             default - 300.
    pub fn with_cache_time(mut self, value: Integer) -> Self {
        self.cache_time = Some(value);
        self
    }

    /// Sets a new value for an `is_personal` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the cache results on the server side
    ///             are only for the user that sent the query;
    ///             by default, results may be returned to any user who sends the same query.
    pub fn with_is_personal(mut self, value: bool) -> Self {
        self.is_personal = Some(value);
        self
    }

    /// Sets a new next offset.
    ///
    ///
    /// # Arguments
    ///
    /// * `value` - Offset that a client should send in the next query with
    ///             the same text to receive more results.
    ///
    /// Pass an empty string if there are no more results or if you don‘t support pagination.
    /// Offset length can’t exceed 64 bytes.
    pub fn with_next_offset<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.next_offset = Some(value.into());
        self
    }
}

impl Method for AnswerInlineQuery {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("answerInlineQuery", self)
    }
}

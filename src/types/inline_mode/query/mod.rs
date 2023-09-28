use serde::Deserialize;

use crate::types::{Location, User};

#[cfg(test)]
mod tests;

/// Incoming inline query
///
/// When the user sends an empty query, your bot could return some default or trending results
#[derive(Clone, Debug, Deserialize)]
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
    pub chat_type: Option<InlineQueryChatType>,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
}

/// Type of the chat, from which the inline query was sent
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd)]
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

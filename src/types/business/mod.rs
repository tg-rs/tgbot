use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Chat, Integer, Sticker, User},
};

#[cfg(test)]
mod tests;

/// Describes the connection of the bot with a business account.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BusinessConnection {
    /// Whether the bot can act on behalf of the business account in chats that were active in the last 24 hours.
    pub can_reply: bool,
    /// Date the connection was established in Unix time.
    pub date: Integer,
    /// Unique identifier of the business connection.
    pub id: String,
    /// Whether the connection is active.
    pub is_enabled: bool,
    /// Business account user that created the business connection.
    pub user: User,
    /// Identifier of a private chat with the user who created the business connection.
    pub user_chat_id: Integer,
}

impl BusinessConnection {
    /// Creates a new `BusinessConnection`.
    ///
    /// # Arguments
    ///
    /// * date - Date the connection was established in Unix time.
    /// * id - Unique identifier of the business connection.
    /// * user - Business account user that created the business connection.
    /// * user_chat_id - Identifier of a private chat with the user who created the business connection.
    pub fn new<T>(date: Integer, id: T, user: User, user_chat_id: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            can_reply: false,
            date,
            id: id.into(),
            is_enabled: false,
            user,
            user_chat_id,
        }
    }

    /// Sets a new value for the `can_reply` flag.
    ///
    /// # Arguments
    ///
    /// * value - Whether the bot can act on behalf of the business account
    ///           in chats that were active in the last 24 hours.
    pub fn with_can_reply(mut self, value: bool) -> Self {
        self.can_reply = value;
        self
    }

    /// Sets a new value for the `is_enabled` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the connection is active.
    pub fn with_is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = value;
        self
    }
}

/// Represents the intro of the business.
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BusinessIntro {
    /// Message text of the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Sticker of the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    /// Title text of the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl BusinessIntro {
    /// Sets a new message.
    ///
    /// # Arguments
    ///
    /// * `value` - .Message text of the business intro.
    pub fn with_message<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.message = Some(value.into());
        self
    }

    /// Sets a new sticker.
    ///
    /// # Arguments
    ///
    /// * `value` - .Sticker of the business intro.
    pub fn with_sticker(mut self, value: Sticker) -> Self {
        self.sticker = Some(value);
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - .Title text of the business intro.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }
}

/// Provides information about messages deleted from a connected business account.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection.
    pub business_connection_id: String,
    /// Information about a chat in the business account.
    /// The bot may not have access to the chat or the corresponding user.
    pub chat: Chat,
    /// A list of identifiers of deleted messages in the chat of the business account.
    pub message_ids: Vec<Integer>,
}

impl BusinessMessagesDeleted {
    /// Creates a new `BusinessMessagesDeleted`.
    ///
    /// # Arguments
    ///
    /// * business_connection_id - Unique identifier of the business connection.
    /// * chat - Information about a chat in the business account.
    /// * message_ids - A list of identifiers of deleted messages in the chat of the business account.
    pub fn new<A, B, C>(business_connection_id: A, chat: B, message_ids: C) -> Self
    where
        A: Into<String>,
        B: Into<Chat>,
        C: IntoIterator<Item = Integer>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            chat: chat.into(),
            message_ids: message_ids.into_iter().collect(),
        }
    }
}

/// Returns information about the connection of the bot with a business account.
#[derive(Clone, Debug, Serialize)]
pub struct GetBusinessConnection {
    business_connection_id: String,
}

impl GetBusinessConnection {
    /// Creates a new `GetBusinessConnection`.
    ///
    /// # Arguments
    ///
    /// * business_connection_id - Unique identifier of the business connection.
    pub fn new<T>(business_connection_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
        }
    }
}

impl Method for GetBusinessConnection {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("getBusinessConnection", self)
    }
}

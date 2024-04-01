use serde::{Deserialize, Serialize};

use crate::types::{Integer, User};

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

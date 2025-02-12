use serde::{Deserialize, Serialize};

use crate::types::{Chat, Integer, User};

#[cfg(test)]
mod tests;

/// Describes the origin of a message.
#[derive(Clone, Debug, Deserialize, derive_more::From, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MessageOrigin {
    /// The message was originally sent to a channel chat.
    Channel(MessageOriginChannel),
    /// The message was originally sent on behalf of a chat to a group chat.
    Chat(MessageOriginChat),
    /// The message was originally sent by an unknown user.
    HiddenUser(MessageOriginHiddenUser),
    /// The message was originally sent by a known user.
    User(MessageOriginUser),
}

/// The message was originally sent to a channel chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageOriginChannel {
    /// Chat that sent the message originally.
    pub chat: Chat,
    /// Date the message was sent originally in Unix time.
    pub date: Integer,
    /// Unique message identifier inside the chat.
    pub message_id: Integer,
    /// Signature of the original post author.
    pub author_signature: Option<String>,
}

impl MessageOriginChannel {
    /// Creates a new `MessageOriginChannel`.
    ///
    /// # Arguments
    ///
    /// * `chat` - Chat that sent the message originally.
    /// * `date` - Date the message was sent originally in Unix time.
    /// * `message_id` - Unique message identifier inside the chat.
    pub fn new<T>(chat: T, date: Integer, message_id: Integer) -> Self
    where
        T: Into<Chat>,
    {
        Self {
            chat: chat.into(),
            date,
            message_id,
            author_signature: None,
        }
    }

    /// Sets a new author signature
    ///
    /// # Arguments
    ///
    /// * `value` - Signature of the original post author.
    pub fn with_author_signature<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.author_signature = Some(value.into());
        self
    }
}

/// The message was originally sent on behalf of a chat to a group chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageOriginChat {
    /// Date the message was sent originally in Unix time.
    pub date: Integer,
    /// Chat that sent the message originally.
    pub sender_chat: Chat,
    /// For messages originally sent by an anonymous chat administrator, original message author signature.
    pub author_signature: Option<String>,
}

impl MessageOriginChat {
    /// Creates a new `MessageOriginChat`.
    ///
    /// # Arguments
    ///
    /// * `date` - Date the message was sent originally in Unix time.
    /// * `sender_chat` - Chat that sent the message originally.
    pub fn new<T>(date: Integer, sender_chat: T) -> Self
    where
        T: Into<Chat>,
    {
        Self {
            date,
            sender_chat: sender_chat.into(),
            author_signature: None,
        }
    }

    /// Sets a new author signature
    ///
    /// # Arguments
    ///
    /// * `value` - Signature of the original post author.
    pub fn with_author_signature<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.author_signature = Some(value.into());
        self
    }
}

/// The message was originally sent by an unknown user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageOriginHiddenUser {
    /// Date the message was sent originally in Unix time.
    pub date: Integer,
    /// Name of the user that sent the message originally.
    pub sender_user_name: String,
}

impl MessageOriginHiddenUser {
    /// Creates a new `MessageOriginHiddenUser`.
    ///
    /// # Arguments
    ///
    /// * `date` - Date the message was sent originally in Unix time.
    /// * `sender_user_name` - Name of the user that sent the message originally.
    pub fn new<T>(date: Integer, sender_user_name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            date,
            sender_user_name: sender_user_name.into(),
        }
    }
}

/// The message was originally sent by a known user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageOriginUser {
    /// Date the message was sent originally in Unix time.
    pub date: Integer,
    /// User that sent the message originally.
    pub sender_user: User,
}

impl MessageOriginUser {
    /// Creates a new `MessageOriginUser`.
    ///
    /// # Arguments
    ///
    /// * `date` - Date the message was sent originally in Unix time.
    /// * `sender_user` - User that sent the message originally.
    pub fn new(date: Integer, sender_user: User) -> Self {
        Self { date, sender_user }
    }
}

use serde::{Deserialize, Serialize};

use crate::types::{Chat, Integer, User};

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

/// The message was originally sent by an unknown user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageOriginHiddenUser {
    /// Date the message was sent originally in Unix time.
    pub date: Integer,
    /// Name of the user that sent the message originally.
    pub sender_user_name: String,
}

/// The message was originally sent by a known user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageOriginUser {
    /// Date the message was sent originally in Unix time.
    pub date: Integer,
    /// User that sent the message originally.
    pub sender_user: User,
}

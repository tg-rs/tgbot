use serde::Deserialize;

use crate::types::{ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Private chat
#[derive(Clone, Debug, Deserialize)]
pub struct PrivateChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// First name of the other party
    pub first_name: String,
    /// Last name of the other party
    pub last_name: Option<String>,
    /// Username of a chat
    pub username: Option<String>,
    /// Chat photo
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Bio of the other party
    ///
    /// Returned only in getChat
    pub bio: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// True, if privacy settings of the other party
    /// in the private chat allows to use tg://user?id=<user_id>
    /// links only in chats with the user.
    ///
    /// Returned only in getChat
    pub has_private_forwards: Option<bool>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    /// Returned only in getChat.
    pub message_auto_delete_time: Option<Integer>,
}

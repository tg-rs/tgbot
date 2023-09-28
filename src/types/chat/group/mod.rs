use serde::Deserialize;

use crate::types::{ChatPermissions, ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Group chat
#[derive(Clone, Debug, Deserialize)]
pub struct GroupChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Chat photo
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Invite link
    ///
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions, for groups and supergroups
    ///
    /// Returned only in getChat
    pub permissions: Option<ChatPermissions>,
    /// True, if messages from the chat can't be forwarded to other chats
    ///
    /// Returned only in getChat.
    pub has_protected_content: Option<bool>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    /// Returned only in getChat.
    pub message_auto_delete_time: Option<Integer>,
}

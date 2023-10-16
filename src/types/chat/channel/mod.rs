use serde::{Deserialize, Serialize};

use crate::types::{ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Channel chat
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChannelChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Chat photo
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Description of a channel
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Invite link
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Unique identifier for the linked discussion group
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<Integer>,
    /// True, if messages from the chat can't be forwarded to other chats
    ///
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// List of all active chat usernames
    ///
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
}

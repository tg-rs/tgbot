use serde::Deserialize;

use crate::types::{ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Channel chat
#[derive(Clone, Debug, Deserialize)]
pub struct ChannelChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a channel
    pub username: Option<String>,
    /// Chat photo
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Description of a channel
    ///
    /// Returned only in getChat
    pub description: Option<String>,
    /// Invite link
    ///
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// Unique identifier for the linked discussion group
    ///
    /// Returned only in getChat
    pub linked_chat_id: Option<Integer>,
    /// True, if messages from the chat can't be forwarded to other chats
    ///
    /// Returned only in getChat.
    pub has_protected_content: Option<bool>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    /// Returned only in getChat.
    pub message_auto_delete_time: Option<Integer>,
}

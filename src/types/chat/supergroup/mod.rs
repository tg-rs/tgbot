use serde::{Deserialize, Serialize};

use crate::types::{ChatLocation, ChatPermissions, ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Supergroup chat
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SupergroupChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a supergroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Photo of a supergroup
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Description of a supergroup
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
    /// For supergroups, name of group sticker set
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    /// Default chat member permissions, for groups and supergroups
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    /// The minimum allowed delay between consecutive messages sent by each unprivileged user
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<Integer>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Unique identifier for the linked channel
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<Integer>,
    /// The location to which the supergroup is connected
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
    /// True, if messages from the chat can't be forwarded to other chats
    ///
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
}

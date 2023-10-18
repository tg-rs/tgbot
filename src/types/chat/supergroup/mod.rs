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
    /// True, if users need to join the supergroup before they can send messages
    ///
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    /// True, if all users directly joining the supergroup
    /// need to be approved by supergroup administrators
    ///
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    /// True, if the supergroup chat is a forum (has topics enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// List of all active chat usernames
    ///
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// True, if non-administrators can only get the list of bots
    /// and administrators in the chat.
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// True, if aggressive anti-spam checks are enabled in the supergroup.
    ///
    /// The field is only available to chat administrators.
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
}

use serde::Deserialize;

use crate::types::{ChatLocation, ChatPermissions, ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Supergroup chat
#[derive(Clone, Debug, Deserialize)]
pub struct SupergroupChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a supergroup
    pub username: Option<String>,
    /// Photo of a supergroup
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Description of a supergroup
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
    /// For supergroups, name of group sticker set
    ///
    /// Returned only in getChat
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set
    ///
    /// Returned only in getChat
    pub can_set_sticker_set: Option<bool>,
    /// Default chat member permissions, for groups and supergroups
    ///
    /// Returned only in getChat
    pub permissions: Option<ChatPermissions>,
    /// The minimum allowed delay between consecutive messages sent by each unpriviledged user
    ///
    /// Returned only in getChat
    pub slow_mode_delay: Option<Integer>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    /// Returned only in getChat.
    pub message_auto_delete_time: Option<Integer>,
    /// Unique identifier for the linked channel
    ///
    /// Returned only in getChat
    pub linked_chat_id: Option<Integer>,
    /// The location to which the supergroup is connected
    ///
    /// Returned only in getChat
    pub location: Option<ChatLocation>,
    /// True, if messages from the chat can't be forwarded to other chats
    ///
    /// Returned only in getChat.
    pub has_protected_content: Option<bool>,
}

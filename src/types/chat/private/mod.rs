use serde::{Deserialize, Serialize};

use crate::types::{ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Private chat
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrivateChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// First name of the other party
    pub first_name: String,
    /// Last name of the other party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Username of a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Chat photo
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Bio of the other party
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// True, if privacy settings of the other party
    /// in the private chat allows to use tg://user?id=<user_id>
    /// links only in chats with the user.
    ///
    /// Returned only in getChat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// True, if the privacy settings of the other party
    /// restrict sending voice and video note messages in the private chat.
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// List of all active chat usernames
    ///
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// Custom emoji identifier of emoji status of the other party in a private chat.
    ///
    /// Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
}

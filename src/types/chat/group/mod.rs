use serde::{Deserialize, Serialize};

use crate::types::{ChatPermissions, ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Represents a group chat
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GroupChat {
    /// Unique identifier
    pub id: Integer,
    /// Title
    pub title: String,
    /// Whether non-administrators can only get the list of bots
    /// and administrators in the group
    ///
    /// Returned only in `GetChat`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// Whether messages from the group can't be forwarded to other chats
    ///
    /// Returned only in `GetChat`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Invite link
    ///
    /// Returned only in `GetChat`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// The time after which all messages sent to the group
    /// will be automatically deleted; in seconds
    ///
    /// Returned only in `GetChat`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Default chat member permissions
    ///
    /// Returned only in `GetChat`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    /// Photo
    ///
    /// Returned only in `GetChat`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Latest pinned message
    ///
    /// Returned only in `GetChat`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
}

impl GroupChat {
    /// Creates a new GroupChat
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier
    /// * title - Title
    pub fn new<T>(id: Integer, title: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            id,
            title: title.into(),
            has_hidden_members: None,
            has_protected_content: None,
            invite_link: None,
            message_auto_delete_time: None,
            permissions: None,
            photo: None,
            pinned_message: None,
        }
    }

    /// Sets a value for the `has_hidden_members` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether non-administrators can only get the list
    ///           of bots and administrators in the chat
    pub fn with_has_hidden_members(mut self, value: bool) -> Self {
        self.has_hidden_members = Some(value);
        self
    }

    /// Sets a new value for the `has_protected_content` flag
    ///
    /// * value - Whether messages from the group can't be forwarded to other chats
    pub fn with_has_protected_content(mut self, value: bool) -> Self {
        self.has_protected_content = Some(value);
        self
    }

    /// Sets a new invite link
    ///
    /// # Arguments
    ///
    /// * value - Invite link
    pub fn with_invite_link<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.invite_link = Some(value.into());
        self
    }

    /// Sets a time after which all messages sent to the group will be automatically deleted
    ///
    /// # Arguments
    ///
    /// * value - Value in seconds
    pub fn with_message_auto_delete_time(mut self, value: Integer) -> Self {
        self.message_auto_delete_time = Some(value);
        self
    }

    /// Sets default permissions
    ///
    /// # Arguments
    ///
    /// * value - Default permissions
    pub fn with_permissions(mut self, value: ChatPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Sets a new photo
    ///
    /// # Arguments
    ///
    /// * value - Photo
    pub fn with_photo(mut self, value: ChatPhoto) -> Self {
        self.photo = Some(value);
        self
    }

    /// Sets a new pinned message
    ///
    /// # Arguments
    ///
    /// * value - Pinned message
    pub fn with_pinned_message(mut self, value: Message) -> Self {
        self.pinned_message = Some(Box::new(value));
        self
    }
}

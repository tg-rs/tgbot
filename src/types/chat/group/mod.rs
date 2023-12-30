use serde::{Deserialize, Serialize};

use crate::types::{ChatPeerId, ChatPermissions, ChatPhoto, Integer, Message, ReactionType};

#[cfg(test)]
mod tests;

/// Represents a group chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GroupChat {
    /// Unique identifier of the group.
    pub id: ChatPeerId,
    /// Title of the group.
    pub title: String,
    /// List of available reactions allowed in the chat.
    ///
    /// If omitted, then all emoji reactions are allowed.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,
    /// Custom emoji identifier of emoji status.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status of in Unix time, if any.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<Integer>,
    /// Indicates whether non-administrators can only get the list of bots
    /// and administrators in the group.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// Indicates whether messages from the group can't be forwarded to other chats.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Invite link for the group.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// The time after which all messages sent to the group
    /// will be automatically deleted; in seconds.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Default chat member permissions.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    /// Photo associated with the group.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Latest pinned message in the group.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
}

impl GroupChat {
    /// Creates a new `GroupChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the group.
    /// * `title` - Title of the group.
    pub fn new<A, B>(id: A, title: B) -> Self
    where
        A: Into<ChatPeerId>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            available_reactions: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
            has_hidden_members: None,
            has_protected_content: None,
            invite_link: None,
            message_auto_delete_time: None,
            permissions: None,
            photo: None,
            pinned_message: None,
        }
    }

    /// Sets a new list of available reactions.
    ///
    /// # Arguments
    ///
    /// `value` - The list of all available reactions.
    pub fn with_available_reactions<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = ReactionType>,
    {
        self.available_reactions = Some(value.into_iter().collect());
        self
    }

    /// Sets a new custom emoji identifier of emoji status.
    ///
    /// # Arguments
    ///
    /// * `value` - Emoji identifier.
    pub fn with_emoji_status_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.emoji_status_custom_emoji_id = Some(value.into());
        self
    }

    /// Sets a new emoji status expiration date.
    ///
    /// # Arguments
    ///
    /// * `value` - Unix timestamp; in seconds.
    pub fn with_emoji_status_expiration_date(mut self, value: Integer) -> Self {
        self.emoji_status_expiration_date = Some(value);
        self
    }

    /// Sets a value for a `has_hidden_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether non-administrators can only get the list
    ///             of bots and administrators in the chat.
    pub fn with_has_hidden_members(mut self, value: bool) -> Self {
        self.has_hidden_members = Some(value);
        self
    }

    /// Sets a new value for a `has_protected_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether messages from the chat can't be forwarded to other chats.
    pub fn with_has_protected_content(mut self, value: bool) -> Self {
        self.has_protected_content = Some(value);
        self
    }

    /// Sets a new invite link.
    ///
    /// # Arguments
    ///
    /// * `value` - The invite link for the group.
    pub fn with_invite_link<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.invite_link = Some(value.into());
        self
    }

    /// Sets a time after which all messages sent to the group will be automatically deleted.
    ///
    /// # Arguments
    ///
    /// * `value` - Value in seconds.
    pub fn with_message_auto_delete_time(mut self, value: Integer) -> Self {
        self.message_auto_delete_time = Some(value);
        self
    }

    /// Sets default permissions.
    ///
    /// # Arguments
    ///
    /// * `value` - Default permissions.
    pub fn with_permissions(mut self, value: ChatPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Sets a new photo.
    ///
    /// # Arguments
    ///
    /// * `value` - The photo associated with the group.
    pub fn with_photo(mut self, value: ChatPhoto) -> Self {
        self.photo = Some(value);
        self
    }

    /// Sets a new pinned message.
    ///
    /// # Arguments
    ///
    /// * `value` - Latest pinned message in the group.
    pub fn with_pinned_message(mut self, value: Message) -> Self {
        self.pinned_message = Some(Box::new(value));
        self
    }
}

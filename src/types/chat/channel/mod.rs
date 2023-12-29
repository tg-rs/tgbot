use serde::{Deserialize, Serialize};

use crate::types::{ChatPeerId, ChatPhoto, ChatUsername, Integer, Message, ReactionType};

#[cfg(test)]
mod tests;

/// Represents a channel chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChannelChat {
    /// Unique identifier of the channel.
    pub id: ChatPeerId,
    /// Title of the channel.
    pub title: String,
    /// List of all active channel usernames.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// List of available reactions allowed in the chat.
    ///
    /// If omitted, then all emoji reactions are allowed.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,
    /// Description of the channel.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates whether messages from the channel can't be forwarded to other chats.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Invite link for the channel.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// Unique identifier of a linked discussion group.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<Integer>,
    /// The time after which all messages sent to the channel
    /// will be automatically deleted; in seconds.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Photo associated with the channel.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Latest pinned message in the channel.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Username of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<ChatUsername>,
}

impl ChannelChat {
    /// Creates a new `ChannelChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the channel.
    /// * `title` - Title of the channel.
    pub fn new<A, B>(id: A, title: B) -> Self
    where
        A: Into<ChatPeerId>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            active_usernames: None,
            available_reactions: None,
            description: None,
            has_protected_content: None,
            invite_link: None,
            linked_chat_id: None,
            message_auto_delete_time: None,
            photo: None,
            pinned_message: None,
            username: None,
        }
    }

    /// Sets a new list of active usernames.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of all active channel usernames.
    pub fn with_active_usernames<A, B>(mut self, value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        self.active_usernames = Some(value.into_iter().map(Into::into).collect());
        self
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

    /// Sets a new description
    ///
    /// # Arguments
    ///
    /// * `value` - The description of the channel.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(value.into());
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
    /// * `value` - The invite link for the channel.
    pub fn with_invite_link<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.invite_link = Some(value.into());
        self
    }

    /// Sets a new linked chat ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the linked discussion group.
    pub fn with_linked_chat_id(mut self, value: Integer) -> Self {
        self.linked_chat_id = Some(value);
        self
    }

    /// Sets a new message auto-delete time.
    ///
    /// # Arguments
    ///
    /// * `value` - Value in seconds after which
    ///             all messages sent to the chat
    ///             will be automatically deleted.
    pub fn with_message_auto_delete_time(mut self, value: Integer) -> Self {
        self.message_auto_delete_time = Some(value);
        self
    }

    /// Sets a new photo.
    ///
    /// # Arguments
    ///
    /// * `value` - Photo associated with the channel.
    pub fn with_photo(mut self, value: ChatPhoto) -> Self {
        self.photo = Some(value);
        self
    }

    /// Sets a new pinned message.
    ///
    /// # Arguments
    ///
    /// * `value` - Latest pinned message in the channel.
    pub fn with_pinned_message(mut self, value: Message) -> Self {
        self.pinned_message = Some(Box::new(value));
        self
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username of the channel.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<ChatUsername>,
    {
        self.username = Some(value.into());
        self
    }
}

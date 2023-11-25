use serde::{Deserialize, Serialize};

use crate::types::{ChatPhoto, Integer, Message};

#[cfg(test)]
mod tests;

/// Represents a private chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrivateChat {
    /// Unique identifier of the chat.
    pub id: Integer,
    /// First name of the other party.
    pub first_name: String,
    /// Last name of the other party.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Username of the target chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Photo of the other party.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Bio of the other party.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Latest pinned message.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Indicates whether privacy settings of the other party
    /// in the private chat allows to use `tg://user?id=<user_id>`
    /// links only in chats with the user.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Indicates whether privacy settings of the other party
    /// restrict sending voice and video note messages.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// List of all active usernames of the other party.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// Custom emoji identifier of emoji status of the other party.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status of
    /// the other party in a private chat in Unix time, if any.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<Integer>,
}

impl PrivateChat {
    /// Creates a new `PrivateChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the target chat.
    /// * `first_name` - First name of the other party.
    pub fn new<T>(id: Integer, first_name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            id,
            first_name: first_name.into(),
            last_name: None,
            username: None,
            photo: None,
            bio: None,
            pinned_message: None,
            has_private_forwards: None,
            message_auto_delete_time: None,
            has_restricted_voice_and_video_messages: None,
            active_usernames: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
        }
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - Last name.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.username = Some(value.into());
        self
    }

    /// Sets a new photo.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat photo.
    pub fn with_photo(mut self, value: ChatPhoto) -> Self {
        self.photo = Some(value);
        self
    }

    /// Sets a new bio.
    ///
    /// # Arguments
    ///
    /// * `value` - Bio.
    pub fn with_bio<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.bio = Some(value.into());
        self
    }

    /// Sets a new pinned message.
    ///
    /// # Arguments
    ///
    /// * `value` - Pinned message.
    pub fn with_pinned_message(mut self, value: Message) -> Self {
        self.pinned_message = Some(Box::new(value));
        self
    }

    /// Sets a new value for a `has_private_forwards` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether privacy settings of the other party
    ///             in the private chat allows to use `tg://user?id=<user_id>`
    ///             links only in chats with the user.
    pub fn with_has_private_forwards(mut self, value: bool) -> Self {
        self.has_private_forwards = Some(value);
        self
    }

    /// Sets a new message auto-delete time.
    ///
    /// # Arguments
    ///
    /// * `value` - Time in seconds.
    pub fn with_message_auto_delete_time(mut self, value: Integer) -> Self {
        self.message_auto_delete_time = Some(value);
        self
    }

    /// Sets a new value for a `has_restricted_voice_and_video_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether privacy settings of the other party
    ///             restrict sending voice and video note messages.
    pub fn with_has_restricted_voice_and_video_messages(mut self, value: bool) -> Self {
        self.has_restricted_voice_and_video_messages = Some(value);
        self
    }

    /// Sets a new list of active usernames.
    ///
    /// # Arguments
    ///
    /// * `value` - Active usernames.
    pub fn with_active_usernames<A, B>(mut self, value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        self.active_usernames = Some(value.into_iter().map(Into::into).collect());
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
}

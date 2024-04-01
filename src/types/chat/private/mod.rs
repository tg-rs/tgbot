use serde::{Deserialize, Serialize};

use crate::types::{
    AccentColor,
    BusinessIntro,
    ChatPeerId,
    ChatPhoto,
    ChatUsername,
    Integer,
    Message,
    ProfileAccentColor,
};

#[cfg(test)]
mod tests;

/// Represents a private chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrivateChat {
    /// Unique identifier of the chat.
    pub id: ChatPeerId,
    /// First name of the other party.
    pub first_name: String,
    /// Identifier of the accent color for the chat name and
    /// backgrounds of the chat photo, reply header, and link preview.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(rename = "accent_color_id", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<AccentColor>,
    /// List of all active usernames of the other party.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// Custom emoji identifier of emoji chosen by the chat for the reply header and link preview background.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    /// Bio of the other party.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// For chats with business accounts, the intro of the business.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,
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
    /// Indicates whether privacy settings of the other party
    /// in the private chat allows to use `tg://user?id=<user_id>`
    /// links only in chats with the user.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// Indicates whether privacy settings of the other party
    /// restrict sending voice and video note messages.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// Last name of the other party.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The time after which all messages sent to the chat
    /// will be automatically deleted; in seconds.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Photo of the other party.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Latest pinned message.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Identifier of the accent color for the chat's profile background.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(rename = "profile_accent_color_id", skip_serializing_if = "Option::is_none")]
    pub profile_accent_color: Option<ProfileAccentColor>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,
    /// Username of the target chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<ChatUsername>,
}

impl PrivateChat {
    /// Creates a new `PrivateChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the target chat.
    /// * `first_name` - First name of the other party.
    pub fn new<A, B>(id: A, first_name: B) -> Self
    where
        A: Into<ChatPeerId>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            first_name: first_name.into(),
            accent_color: None,
            active_usernames: None,
            background_custom_emoji_id: None,
            bio: None,
            business_intro: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None,
            last_name: None,
            message_auto_delete_time: None,
            photo: None,
            pinned_message: None,
            profile_accent_color: None,
            profile_background_custom_emoji_id: None,
            username: None,
        }
    }

    /// Sets a new accent color.
    ///
    /// # Arguments
    ///
    /// * `value` - Accent color for the chat.
    pub fn with_accent_color(mut self, value: AccentColor) -> Self {
        self.accent_color = Some(value);
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

    /// Sets a new custom emoji identifier for the message background.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom emoji identifier of emoji chosen by the chat
    ///             for the reply header and link preview background.
    pub fn with_background_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.background_custom_emoji_id = Some(value.into());
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

    /// Sets a new business intro.
    ///
    /// # Arguments
    ///
    /// * `value` - Intro.
    pub fn with_business_intro(mut self, value: BusinessIntro) -> Self {
        self.business_intro = Some(value);
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

    /// Sets a new message auto-delete time.
    ///
    /// # Arguments
    ///
    /// * `value` - Time in seconds.
    pub fn with_message_auto_delete_time(mut self, value: Integer) -> Self {
        self.message_auto_delete_time = Some(value);
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

    /// Sets a new pinned message.
    ///
    /// # Arguments
    ///
    /// * `value` - Pinned message.
    pub fn with_pinned_message(mut self, value: Message) -> Self {
        self.pinned_message = Some(Box::new(value));
        self
    }

    /// Sets a new profile accent color.
    ///
    /// # Arguments
    ///
    /// * `value` - Accent color for the chat's profile background.
    pub fn with_profile_accent_color(mut self, value: ProfileAccentColor) -> Self {
        self.profile_accent_color = Some(value);
        self
    }

    /// Sets a new custom emoji identifer for the chat's profile background.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom emoji identifier of the emoji chosen by the chat for its profile background.
    pub fn with_profile_background_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.profile_background_custom_emoji_id = Some(value.into());
        self
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<ChatUsername>,
    {
        self.username = Some(value.into());
        self
    }
}

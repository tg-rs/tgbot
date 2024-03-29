use serde::{Deserialize, Serialize};

use crate::types::{
    AccentColor,
    ChatLocation,
    ChatPeerId,
    ChatPermissions,
    ChatPhoto,
    ChatUsername,
    Integer,
    Message,
    ProfileAccentColor,
    ReactionType,
};

#[cfg(test)]
mod tests;

/// Represents a supergroup chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SupergroupChat {
    /// Unique identifier of the supergroup.
    pub id: ChatPeerId,
    /// Title of the supergroup.
    pub title: String,
    /// Identifier of the accent color for the chat name and
    /// backgrounds of the chat photo, reply header, and link preview.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(rename = "accent_color_id", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<AccentColor>,
    /// List of all active supergroup usernames.
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
    /// Custom emoji identifier of emoji chosen by the chat for the reply header and link preview background.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    /// Indicates whether the bot can change the supergroup sticker set.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    /// The name of the group's custom emoji sticker set.
    /// Custom emoji from this set can be used by all users and bots in the group.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,
    /// Description of the supergroup.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    /// Indicates whether aggressive anti-spam checks are enabled in the supergroup.
    ///
    /// The field is only available to chat administrators.
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// Indicates whether non-administrators can only get the list of bots
    /// and administrators in the supergroup.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// Indicates whether messages from the supergroup can't be forwarded to other chats.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Whether new chat members will have access to old messages;
    /// available only to chat administrators.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    /// Invite link for the supergroup.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// Indicates whether the supergroup is a forum (has topics enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// Indicates whether all users directly joining the supergroup
    /// need to be approved by administrators.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    /// Indicates whether users need to join the supergroup before they can send messages.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    /// Unique identifier of the linked channel.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<Integer>,
    /// The location to which the supergroup is connected.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
    /// The time after which all messages sent to the supergroup
    /// will be automatically deleted; in seconds.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Default supergroup member permissions.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    /// Photo associated with the supergroup.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Latest pinned message in the supergroup.
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
    /// The minimum allowed delay between consecutive messages sent by each unprivileged user.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<Integer>,
    /// Name of supergroup sticker set.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// The minimum number of boosts that a non-administrator user needs to add
    /// in order to ignore slow mode and chat permissions.
    ///
    /// Returned only in [`crate::types::GetChat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<Integer>,
    /// Username of the supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<ChatUsername>,
}

impl SupergroupChat {
    /// Creates a new `SupergroupChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the supergroup.
    /// * `title` - Title of the supergroup.
    pub fn new<A, B>(id: A, title: B) -> Self
    where
        A: Into<ChatPeerId>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            accent_color: None,
            active_usernames: None,
            available_reactions: None,
            background_custom_emoji_id: None,
            can_set_sticker_set: None,
            custom_emoji_sticker_set_name: None,
            description: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
            has_aggressive_anti_spam_enabled: None,
            has_hidden_members: None,
            has_protected_content: None,
            has_visible_history: None,
            invite_link: None,
            is_forum: None,
            join_by_request: None,
            join_to_send_messages: None,
            linked_chat_id: None,
            location: None,
            message_auto_delete_time: None,
            permissions: None,
            photo: None,
            pinned_message: None,
            profile_accent_color: None,
            profile_background_custom_emoji_id: None,
            slow_mode_delay: None,
            sticker_set_name: None,
            unrestrict_boost_count: None,
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

    /// Sets a value for a `can_set_sticker_set` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the bot can change the supergroup sticker set.
    pub fn with_can_set_sticker_set(mut self, value: bool) -> Self {
        self.can_set_sticker_set = Some(value);
        self
    }

    /// Sets a new custom emoji sticker set name
    ///
    /// # Arguments
    ///
    /// `value` - Name of of the group's custom emoji sticker set.
    pub fn with_custom_emoji_sticker_set_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_emoji_sticker_set_name = Some(value.into());
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the supergroup.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(value.into());
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

    /// Sets a new value for a `has_aggressive_anti_spam_enabled` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether aggressive anti-spam checks are enabled in the supergroup.
    pub fn with_has_aggressive_anti_spam_enabled(mut self, value: bool) -> Self {
        self.has_aggressive_anti_spam_enabled = Some(value);
        self
    }

    /// Sets a new value for a `has_hidden_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether non-administrators can only get the list of bots
    ///             and administrators in the supergroup.
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

    /// Sets a new value for a `has_visible_history` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether new chat members will have access to old messages;
    ///             available only to chat administrators.
    pub fn with_has_visible_history(mut self, value: bool) -> Self {
        self.has_visible_history = Some(value);
        self
    }

    /// Sets a new invite link.
    ///
    /// # Arguments
    ///
    /// * `value` - Invite link for the supergroup.
    pub fn with_invite_link<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.invite_link = Some(value.into());
        self
    }

    /// Sets a new value for an `is_forum` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the supergroup is a forum (has topics enabled).
    pub fn with_is_forum(mut self, value: bool) -> Self {
        self.is_forum = Some(value);
        self
    }

    /// Sets a new value for a `join_by_request` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether all users directly joining the supergroup
    ///             need to be approved by administrators.
    pub fn with_join_by_request(mut self, value: bool) -> Self {
        self.join_by_request = Some(value);
        self
    }

    /// Sets a new value for a `join_to_send_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether users need to join
    ///             the supergroup before they can send messages.
    pub fn with_join_to_send_messages(mut self, value: bool) -> Self {
        self.join_to_send_messages = Some(value);
        self
    }

    /// Sets a new linked chat ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat ID.
    pub fn with_linked_chat_id(mut self, value: Integer) -> Self {
        self.linked_chat_id = Some(value);
        self
    }

    /// Sets a new supergroup location.
    ///
    /// # Arguments
    ///
    /// * `value` - Location.
    pub fn with_location(mut self, value: ChatLocation) -> Self {
        self.location = Some(value);
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
    /// * `value` - Photo associated with the supergroup.
    pub fn with_photo(mut self, value: ChatPhoto) -> Self {
        self.photo = Some(value);
        self
    }

    /// Sets a new latest pinned message.
    ///
    /// # Arguments
    ///
    /// * `value` - Latest pinned message in the supergroup.
    pub fn with_pinned_message(mut self, value: Message) -> Self {
        self.pinned_message = Some(Box::new(value));
        self
    }

    /// Sets a new permissions.
    ///
    /// # Arguments
    ///
    /// * `value` - Permissions.
    pub fn with_permissions(mut self, value: ChatPermissions) -> Self {
        self.permissions = Some(value);
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

    /// Sets a new slow mode delay.
    ///
    /// # Arguments
    ///
    /// * `value` - Delay in seconds.
    pub fn with_slow_mode_delay(mut self, value: Integer) -> Self {
        self.slow_mode_delay = Some(value);
        self
    }

    /// Sets a new sticker set name.
    ///
    /// # Arguments
    ///
    /// * `value` - Name of the sticker set.
    pub fn with_sticker_set_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.sticker_set_name = Some(value.into());
        self
    }

    /// Sets a new unrestrict boost count.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of boosts.
    pub fn with_unrestrict_boost_count(mut self, value: Integer) -> Self {
        self.unrestrict_boost_count = Some(value);
        self
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username of the supergroup.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<ChatUsername>,
    {
        self.username = Some(value.into());
        self
    }
}

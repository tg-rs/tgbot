use serde::{Deserialize, Serialize};

use crate::types::{
    AccentColor,
    Birthdate,
    BusinessIntro,
    BusinessLocation,
    BusinessOpeningHours,
    Chat,
    ChatLocation,
    ChatPermissions,
    ChatPhoto,
    Integer,
    Message,
    ProfileAccentColor,
    ReactionType,
};

#[cfg(test)]
mod tests;

/// Type of the chat.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatFullInfoType {
    /// Channel chat.
    Channel,
    /// Group chat.
    Group,
    /// Private chat.
    Private,
    /// Supergroup chat.
    Supergroup,
}

/// Contains full information about a chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatFullInfo {
    /// Unique identifier for this chat.
    pub id: Integer,
    #[serde(rename = "type")]
    /// Type of the chat.
    pub chat_type: ChatFullInfoType,

    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview.
    #[serde(rename = "accent_color_id")]
    pub accent_color: AccentColor,
    /// The maximum number of reactions that can be set on a message in the chat.
    pub max_reaction_count: Integer,

    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,
    /// Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    /// Whether non-administrators can only get the list of bots and administrators in the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// Whether messages from the chat can't be forwarded to other chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Whether new chat members will have access to old messages; available only to chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Chat photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<ChatPhoto>>,
    /// The most recent pinned message (by sending date).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Message>,
    /// Identifier of the accent color for the chat's profile background.
    #[serde(rename = "profile_accent_color_id", skip_serializing_if = "Option::is_none")]
    pub profile_accent_color: Option<ProfileAccentColor>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,

    /// The list of all active chat usernames; for private chats, supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// Username, for private chats, supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Description, for supergroups, channels and group chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Primary invite link, for supergroups, channels and group chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    /// Title, for supergroups, channels and group chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Whether paid media messages can be sent or forwarded to the channel chat.
    ///
    /// The field is available only for channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,

    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa;
    /// for supergroups and channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<Integer>,

    /// Bio of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// For private chats, the date of birth of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Birthdate>,
    /// For private chats with business accounts, the intro of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,
    /// For private chats with business accounts, the location of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_location: Option<BusinessLocation>,
    /// For private chats with business accounts, the opening hours of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<BusinessOpeningHours>,
    /// Custom emoji identifier of the emoji status of the chat or the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<Integer>,
    /// First name of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Whether privacy settings of the other party in the private chat
    /// allows to use `tg://user?id=<user_id>` links only in chats with the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// Whether the privacy settings of the other party restrict
    /// sending voice and video note messages in the private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// Last name of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// For private chats, the personal channel of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Chat>,

    /// Whether the bot can change the group sticker set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    /// For supergroups, the name of the group's custom emoji sticker set.
    /// Custom emoji from this set can be used by all users and bots in the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,
    /// Whether aggressive anti-spam checks are enabled in the supergroup.
    /// The field is only available to chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// Whether the supergroup chat is a forum (has topics enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// Whether users need to join the supergroup before they can send messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    /// Whether all users directly joining the supergroup need to be approved by supergroup administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    /// For supergroups, the location to which the supergroup is connected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
    /// For supergroups, the minimum allowed delay between
    /// consecutive messages sent by each unprivileged user; in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<Integer>,
    /// For supergroups, name of the group sticker set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// For supergroups, the minimum number of boosts that a non-administrator
    /// user needs to add in order to ignore slow mode and chat permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<Integer>,

    /// Default chat member permissions, for groups and supergroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    /// Undocumented.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_gift: Option<bool>,
}

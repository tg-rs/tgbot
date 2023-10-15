use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::ChatId,
};

#[cfg(test)]
mod tests;

/// Represents the rights of an administrator in a chat
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatAdministratorRights {
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can
    /// access the chat event log, boost list in channels,
    /// see channel members, report spam messages,
    /// see anonymous administrators in supergroups and ignore slow mode
    ///
    /// Implied by any other administrator privilege.
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members,
    /// or access supergroup statistics
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges
    /// or demote administrators that they have promoted,
    /// directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can post messages in the channel,
    /// or access channel statistics; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// True, if the administrator can edit messages of other users
    /// and can pin messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// True, if the user is allowed to pin messages; groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// True, if the administrator can post stories in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    /// True, if the administrator can edit stories posted by other users; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    /// True, if the administrator can delete stories posted by other users; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    /// True, if the user is allowed to create, rename,
    /// close, and reopen forum topics; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

impl ChatAdministratorRights {
    /// Returns a new ChatAdministratorRights with all flags set to True
    pub fn all() -> Self {
        Self {
            is_anonymous: true,
            can_manage_chat: true,
            can_delete_messages: true,
            can_manage_video_chats: true,
            can_restrict_members: true,
            can_promote_members: true,
            can_change_info: true,
            can_invite_users: true,
            can_post_messages: Some(true),
            can_edit_messages: Some(true),
            can_pin_messages: Some(true),
            can_post_stories: Some(true),
            can_edit_stories: Some(true),
            can_delete_stories: Some(true),
            can_manage_topics: Some(true),
        }
    }

    /// True, if the administrator can post messages in the channel,
    /// or access channel statistics; channels only
    pub fn with_can_post_messages(mut self, value: bool) -> Self {
        self.can_post_messages = Some(value);
        self
    }

    /// True, if the administrator can edit messages of other users
    /// and can pin messages; channels only
    pub fn with_can_edit_messages(mut self, value: bool) -> Self {
        self.can_edit_messages = Some(value);
        self
    }

    /// True, if the user is allowed to pin messages; groups and supergroups only
    pub fn with_can_pin_messages(mut self, value: bool) -> Self {
        self.can_pin_messages = Some(value);
        self
    }

    /// True, if the administrator can post stories in the channel; channels only
    pub fn with_can_post_stories(mut self, value: bool) -> Self {
        self.can_post_stories = Some(value);
        self
    }

    /// True, if the administrator can edit stories posted by other users; channels only
    pub fn with_can_edit_stories(mut self, value: bool) -> Self {
        self.can_edit_stories = Some(value);
        self
    }

    /// True, if the administrator can delete stories posted by other users; channels only
    pub fn with_can_delete_stories(mut self, value: bool) -> Self {
        self.can_delete_stories = Some(value);
        self
    }

    /// True, if the user is allowed to create, rename,
    /// close, and reopen forum topics; supergroups only
    pub fn with_can_manage_topics(mut self, value: bool) -> Self {
        self.can_manage_topics = Some(value);
        self
    }
}

/// Describes actions that a non-administrator user is allowed to take in a chat
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatPermissions {
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// True, if the user is allowed to send audios, documents,
    /// photos, videos, video notes and voice notes, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,
    /// True, if the user is allowed to send polls, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// True, if the user is allowed to send animations, games, stickers
    /// and use inline bots, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// True, if the user is allowed to add web page previews to their messages,
    /// implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// True, if the user is allowed to change the chat title, photo and other settings
    ///
    /// Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// True, if the user is allowed to invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// True, if the user is allowed to pin messages
    ///
    /// Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

impl ChatPermissions {
    /// Restrict everything
    pub fn restricted() -> Self {
        Self {
            can_send_messages: Some(false),
            can_send_media_messages: Some(false),
            can_send_polls: Some(false),
            can_send_other_messages: Some(false),
            can_add_web_page_previews: Some(false),
            can_change_info: Some(false),
            can_invite_users: Some(false),
            can_pin_messages: Some(false),
        }
    }

    /// Allow everything
    pub fn allowed() -> Self {
        Self {
            can_send_messages: Some(true),
            can_send_media_messages: Some(true),
            can_send_polls: Some(true),
            can_send_other_messages: Some(true),
            can_add_web_page_previews: Some(true),
            can_change_info: Some(true),
            can_invite_users: Some(true),
            can_pin_messages: Some(true),
        }
    }

    /// Permission to send text messages, contacts, locations and venues
    pub fn with_send_messages(mut self, flag: bool) -> Self {
        self.can_send_messages = Some(flag);
        self
    }

    /// Permission to send audios, documents, photos, videos, video notes and voice notes
    pub fn with_send_media_messages(mut self, flag: bool) -> Self {
        self.can_send_media_messages = Some(flag);
        self
    }

    /// Permission to send polls
    pub fn with_send_polls(mut self, flag: bool) -> Self {
        self.can_send_polls = Some(flag);
        self
    }

    /// Permission to send animations, games, stickers and use inline bots
    pub fn with_send_other_messages(mut self, flag: bool) -> Self {
        self.can_send_other_messages = Some(flag);
        self
    }

    /// Permission add web page previews to messages
    pub fn with_add_web_page_previews(mut self, flag: bool) -> Self {
        self.can_add_web_page_previews = Some(flag);
        self
    }

    /// Permission to change the chat title, photo and other settings
    pub fn with_change_info(mut self, flag: bool) -> Self {
        self.can_change_info = Some(flag);
        self
    }

    /// Permission to invite new users to the chat
    pub fn with_invite_users(mut self, flag: bool) -> Self {
        self.can_invite_users = Some(flag);
        self
    }

    /// Permission to pin messages
    pub fn with_pin_messages(mut self, flag: bool) -> Self {
        self.can_pin_messages = Some(flag);
        self
    }
}

/// Set default chat permissions for all members
///
/// The bot must be an administrator in the group or a supergroup
/// for this to work and must have the can_restrict_members admin rights
///
/// Returns True on success
#[derive(Clone, Debug, Serialize)]
pub struct SetChatPermissions {
    chat_id: ChatId,
    permissions: ChatPermissions,
}

impl SetChatPermissions {
    /// Creates a new SetChatPermissions
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * permissions - New permissions
    pub fn new<C: Into<ChatId>>(chat_id: C, permissions: ChatPermissions) -> Self {
        SetChatPermissions {
            chat_id: chat_id.into(),
            permissions,
        }
    }
}

impl Method for SetChatPermissions {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatPermissions", self)
    }
}

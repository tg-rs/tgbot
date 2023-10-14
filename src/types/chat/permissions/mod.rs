use serde::{Deserialize, Serialize};

use crate::{method::Method, request::Request, types::ChatId};

#[cfg(test)]
mod tests;

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

    fn into_request(self) -> Request {
        Request::json("setChatPermissions", self)
    }
}

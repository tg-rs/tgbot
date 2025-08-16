use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::ChatId,
};

/// Represents the rights of an administrator in a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatAdministratorRights {
    /// Indicates whether the user is allowed to change the chat title, photo and other settings.
    pub can_change_info: bool,
    /// Indicates whether the administrator can delete messages of other users.
    pub can_delete_messages: bool,
    /// Indicates whether the administrator can delete stories posted by other users.
    pub can_delete_stories: Option<bool>,
    /// Indicates whether the administrator can edit messages of other users
    /// and can pin messages; channels only.
    pub can_edit_messages: Option<bool>,
    /// Indicates whether the administrator can edit stories posted by other users.
    pub can_edit_stories: Option<bool>,
    /// Indicates whether the user is allowed to invite new users to the chat.
    pub can_invite_users: bool,
    /// Indicates whether the administrator can
    /// access the chat event log, boost list in channels,
    /// see channel members, report spam messages,
    /// see anonymous administrators in supergroups and ignore slow mode.
    ///
    /// Implied by any other administrator privilege.
    pub can_manage_chat: bool,
    /// Whether the administrator can manage direct messages
    /// of the channel and decline suggested posts; for channels only.
    pub can_manage_direct_messages: Option<bool>,
    /// Indicates whether the user is allowed to create, rename,
    /// close, and reopen forum topics; supergroups only.
    pub can_manage_topics: Option<bool>,
    /// Indicates whether the administrator can manage video chats.
    pub can_manage_video_chats: bool,
    /// Indicates whether the user is allowed to pin messages; groups and supergroups only.
    pub can_pin_messages: Option<bool>,
    /// Indicates whether the administrator can post messages in the channel,
    /// or access channel statistics; channels only.
    pub can_post_messages: Option<bool>,
    /// Indicates whether the administrator can post stories in the chat.
    pub can_post_stories: Option<bool>,
    /// Indicates whether the administrator can
    /// add new administrators with a subset of their own privileges
    /// or demote administrators that they have promoted,
    /// directly or indirectly (promoted by administrators that were appointed by the user).
    pub can_promote_members: bool,
    /// Indicates whether the administrator can restrict, ban or unban chat members,
    /// or access supergroup statistics.
    pub can_restrict_members: bool,
    /// Indicates whether the user's presence in the chat is hidden.
    pub is_anonymous: bool,
}

impl ChatAdministratorRights {
    /// Creates a new `ChatAdministratorRights` with all flags set to `true`.
    ///
    /// To create an object with all flags set to `false` use [`Self::default`] method.
    pub fn all() -> Self {
        Self {
            can_change_info: true,
            can_delete_messages: true,
            can_delete_stories: Some(true),
            can_edit_messages: Some(true),
            can_edit_stories: Some(true),
            can_invite_users: true,
            can_manage_chat: true,
            can_manage_direct_messages: Some(true),
            can_manage_topics: Some(true),
            can_manage_video_chats: true,
            can_pin_messages: Some(true),
            can_post_messages: Some(true),
            can_post_stories: Some(true),
            can_promote_members: true,
            can_restrict_members: true,
            is_anonymous: true,
        }
    }

    /// Sets a new value for the `can_change_info` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed
    ///   to change the chat title,
    ///   photo and other settings.
    pub fn with_can_change_info(mut self, value: bool) -> Self {
        self.can_change_info = value;
        self
    }

    /// Sets a new value for the `can_delete_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   delete messages of other users.
    pub fn with_can_delete_messages(mut self, value: bool) -> Self {
        self.can_delete_messages = value;
        self
    }

    /// Sets a new value for the `can_delete_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   delete stories posted by other users;
    ///   channels only.
    pub fn with_can_delete_stories(mut self, value: bool) -> Self {
        self.can_delete_stories = Some(value);
        self
    }

    /// Sets a new value for the `can_edit_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   edit messages of other users and can pin messages;
    ///   channels only.
    pub fn with_can_edit_messages(mut self, value: bool) -> Self {
        self.can_edit_messages = Some(value);
        self
    }

    /// Sets a new value for the `can_edit_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   edit stories posted by other users;
    ///   channels only.
    pub fn with_can_edit_stories(mut self, value: bool) -> Self {
        self.can_edit_stories = Some(value);
        self
    }

    /// Sets a new value for the `can_invite_users` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to
    ///   invite new users to the chat.
    pub fn with_can_invite_users(mut self, value: bool) -> Self {
        self.can_invite_users = value;
        self
    }

    /// Sets a new value for the `can_manage_chat` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   access the chat event log, boost list in channels,
    ///   see channel members, report spam messages,
    ///   see anonymous administrators in supergroups and ignore slow mode.
    pub fn with_can_manage_chat(mut self, value: bool) -> Self {
        self.can_manage_chat = value;
        self
    }

    /// Sets a new value for the `can_manage_direct_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the administrator can manage direct messages
    ///   of the channel and decline suggested posts; for channels only.
    pub fn with_can_manage_direct_messages(mut self, value: bool) -> Self {
        self.can_manage_direct_messages = Some(value);
        self
    }

    /// Sets a new value for the `can_manage_topics` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to
    ///   create, rename, close, and reopen forum topics;
    ///   supergroups only.
    pub fn with_can_manage_topics(mut self, value: bool) -> Self {
        self.can_manage_topics = Some(value);
        self
    }

    /// Sets a new value for the `can_manage_video_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   manage video chats.
    pub fn with_can_manage_video_chats(mut self, value: bool) -> Self {
        self.can_manage_video_chats = value;
        self
    }

    /// Sets a new value for the `can_pin_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to
    ///   pin messages;
    ///   groups and supergroups only.
    pub fn with_can_pin_messages(mut self, value: bool) -> Self {
        self.can_pin_messages = Some(value);
        self
    }

    /// Sets a new value for the `can_post_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   post messages in the channel,
    ///   or access channel statistics;
    ///   channels only.
    pub fn with_can_post_messages(mut self, value: bool) -> Self {
        self.can_post_messages = Some(value);
        self
    }

    /// Sets a new value for the `can_post_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   post stories in the channel;
    ///   channels only.
    pub fn with_can_post_stories(mut self, value: bool) -> Self {
        self.can_post_stories = Some(value);
        self
    }

    /// Sets a new value for the `can_promote_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   add new administrators with a subset of their own privileges
    ///   or demote administrators that they have promoted,
    ///   directly or indirectly (promoted by administrators that
    ///   were appointed by the user).
    pub fn with_can_promote_members(mut self, value: bool) -> Self {
        self.can_promote_members = value;
        self
    }

    /// Sets a new value for the `can_restrict_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///   restrict, ban or unban chat members,
    ///   or access supergroup statistics.
    pub fn with_can_restrict_members(mut self, value: bool) -> Self {
        self.can_restrict_members = value;
        self
    }

    /// Sets a new value for the `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user's presence in the chat is hidden.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.is_anonymous = value;
        self
    }
}

/// Represents actions that a non-administrator user is allowed to take in a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatPermissions {
    /// Indicates whether the user is allowed to add web page previews to their messages.
    pub can_add_web_page_previews: Option<bool>,
    /// Indicates whether the user is allowed to change the chat title, photo and other settings.
    ///
    /// Ignored in public supergroups.
    pub can_change_info: Option<bool>,
    /// Indicates whether the user is allowed to invite new users to the chat.
    pub can_invite_users: Option<bool>,
    /// Indicates whether the user is allowed to create forum topics.
    ///
    /// If omitted defaults to the value of `can_pin_messages`.
    pub can_manage_topics: Option<bool>,
    /// Indicates whether the user is allowed to pin messages.
    ///
    /// Ignored in public supergroups.
    pub can_pin_messages: Option<bool>,
    /// Indicates whether the user is allowed to send audios.
    pub can_send_audios: Option<bool>,
    /// Indicates whether the user is allowed to send documents.
    pub can_send_documents: Option<bool>,
    /// Indicates whether the user is allowed to send text messages, contacts, locations and venues.
    pub can_send_messages: Option<bool>,
    /// Indicates whether the user is allowed to send animations,
    /// games, stickers and use inline bots.
    pub can_send_other_messages: Option<bool>,
    /// Indicates whether the user is allowed to send photos.
    pub can_send_photos: Option<bool>,
    /// Indicates whether the user is allowed to send polls, implies `can_send_messages`.
    pub can_send_polls: Option<bool>,
    /// Indicates whether the user is allowed to send video notes.
    pub can_send_video_notes: Option<bool>,
    /// Indicates whether the user is allowed to send videos.
    pub can_send_videos: Option<bool>,
    /// Indicates whether the user is allowed to send voice notes.
    pub can_send_voice_notes: Option<bool>,
}

impl ChatPermissions {
    /// Restrict everything.
    pub fn restricted() -> Self {
        Self {
            can_change_info: Some(false),
            can_add_web_page_previews: Some(false),
            can_invite_users: Some(false),
            can_manage_topics: Some(false),
            can_pin_messages: Some(false),
            can_send_audios: Some(false),
            can_send_documents: Some(false),
            can_send_messages: Some(false),
            can_send_other_messages: Some(false),
            can_send_photos: Some(false),
            can_send_polls: Some(false),
            can_send_video_notes: Some(false),
            can_send_videos: Some(false),
            can_send_voice_notes: Some(false),
        }
    }

    /// Allow everything.
    pub fn allowed() -> Self {
        Self {
            can_add_web_page_previews: Some(true),
            can_change_info: Some(true),
            can_invite_users: Some(true),
            can_manage_topics: Some(true),
            can_pin_messages: Some(true),
            can_send_audios: Some(true),
            can_send_documents: Some(true),
            can_send_messages: Some(true),
            can_send_other_messages: Some(true),
            can_send_photos: Some(true),
            can_send_polls: Some(true),
            can_send_video_notes: Some(true),
            can_send_videos: Some(true),
            can_send_voice_notes: Some(true),
        }
    }

    /// Sets a new value for the `can_add_web_page_previews` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission add web page previews to messages.
    pub fn with_can_add_web_page_previews(mut self, flag: bool) -> Self {
        self.can_add_web_page_previews = Some(flag);
        self
    }

    /// Sets a new value for the `can_change_info` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to change the chat title, photo and other settings.
    pub fn with_can_change_info(mut self, flag: bool) -> Self {
        self.can_change_info = Some(flag);
        self
    }

    /// Sets a new value for the `can_invite_users` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to invite new users to the chat.
    pub fn with_can_invite_users(mut self, flag: bool) -> Self {
        self.can_invite_users = Some(flag);
        self
    }

    /// Sets a new value for the `can_manage_topics` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to manage topics.
    pub fn with_can_manage_topics(mut self, flag: bool) -> Self {
        self.can_manage_topics = Some(flag);
        self
    }

    /// Sets a new value for the `can_pin_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to pin messages.
    pub fn with_can_pin_messages(mut self, flag: bool) -> Self {
        self.can_pin_messages = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_audios` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send audios.
    pub fn with_can_send_audios(mut self, flag: bool) -> Self {
        self.can_send_audios = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_documents` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send documents.
    pub fn with_can_send_documents(mut self, flag: bool) -> Self {
        self.can_send_documents = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send text messages, contacts, locations and venues.
    pub fn with_can_send_messages(mut self, flag: bool) -> Self {
        self.can_send_messages = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_other_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send animations, games, stickers and use inline bots.
    pub fn with_can_send_other_messages(mut self, flag: bool) -> Self {
        self.can_send_other_messages = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_photos` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send photos.
    pub fn with_can_send_photos(mut self, flag: bool) -> Self {
        self.can_send_photos = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_polls` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send polls.
    pub fn with_can_send_polls(mut self, flag: bool) -> Self {
        self.can_send_polls = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_video_notes` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send video notes.
    pub fn with_can_send_video_notes(mut self, flag: bool) -> Self {
        self.can_send_video_notes = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_videos` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send videos.
    pub fn with_can_send_videos(mut self, flag: bool) -> Self {
        self.can_send_videos = Some(flag);
        self
    }

    /// Sets a new value for the `can_send_voice_notes` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Permission to send voice notes.
    pub fn with_can_send_voice_notes(mut self, flag: bool) -> Self {
        self.can_send_voice_notes = Some(flag);
        self
    }
}

/// Sets default chat permissions for all members.
///
/// The bot must be an administrator in the group or a supergroup
/// for this to work and must have the `can_restrict_members` admin rights.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetChatPermissions {
    chat_id: ChatId,
    permissions: ChatPermissions,
    use_independent_chat_permissions: Option<bool>,
}

impl SetChatPermissions {
    /// Creates a new `SetChatPermissions`
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `permissions` - New permissions.
    pub fn new<T>(chat_id: T, permissions: ChatPermissions) -> Self
    where
        T: Into<ChatId>,
    {
        SetChatPermissions {
            chat_id: chat_id.into(),
            permissions,
            use_independent_chat_permissions: None,
        }
    }

    /// Sets a new value for the `use_independent_chat_permissions` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the chat permissions are set independently.
    ///
    /// If `false`, the `can_send_other_messages` and `can_add_web_page_previews` permissions
    /// will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`,
    /// `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions;
    /// the `can_send_polls` permission will imply the `can_send_messages` permission.
    pub fn with_use_independent_chat_permissions(mut self, value: bool) -> Self {
        self.use_independent_chat_permissions = Some(value);
        self
    }
}

impl Method for SetChatPermissions {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatPermissions", self)
    }
}

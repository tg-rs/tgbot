use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    api::{Method, Payload},
    types::{Chat, ChatId, ChatInviteLink, ChatPermissions, Integer, User},
};

#[cfg(test)]
mod tests;

/// Represents a member of a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "status")]
pub enum ChatMember {
    /// Represents a chat administrator.
    Administrator(ChatMemberAdministrator),
    /// Represents a chat creator.
    Creator(ChatMemberCreator),
    /// Represents a kicked user.
    Kicked(ChatMemberKicked),
    /// Represents a user who left the chat.
    #[serde(deserialize_with = "ChatMemberUser::deserialize_value")]
    #[serde(serialize_with = "ChatMemberUser::serialize_value")]
    Left(User),
    /// Represents a regular chat member.
    Member {
        /// Information about the user
        user: User,
        /// Date when the user's subscription will expire; Unix time
        until_date: Option<Integer>,
    },
    /// Represents a restricted user.
    Restricted(ChatMemberRestricted),
}

impl ChatMember {
    /// Returns the user object associated with the chat member.
    pub fn get_user(&self) -> &User {
        use self::ChatMember::*;
        match self {
            Administrator(ref admin) => &admin.user,
            Creator(ref creator) => &creator.user,
            Kicked(ref kicked) => &kicked.user,
            Left(ref user) => user,
            Member { ref user, .. } => user,
            Restricted(ref restricted) => &restricted.user,
        }
    }

    /// Checks if a user is a member of the chat.
    pub fn is_member(&self) -> bool {
        use self::ChatMember::*;
        match self {
            Administrator(_) | Creator(_) | Member { .. } => true,
            Kicked(_) | Left(_) => false,
            Restricted(ref restricted) => restricted.is_member,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ChatMemberUser {
    user: User,
}

impl ChatMemberUser {
    fn deserialize_value<'de, T>(deserializer: T) -> Result<User, T::Error>
    where
        T: Deserializer<'de>,
    {
        ChatMemberUser::deserialize(deserializer).map(|x| x.user)
    }

    fn serialize_value<T>(value: &User, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let value = ChatMemberUser { user: value.clone() };
        value.serialize(serializer)
    }
}

/// Represents a chat administrator.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatMemberAdministrator {
    /// Information about the user.
    pub user: User,
    /// Indicates whether a bot is allowed to edit administrator privileges of that user.
    pub can_be_edited: bool,
    /// Indicates whether the administrator can change the chat title, photo and other settings.
    pub can_change_info: bool,
    /// Indicates whether the administrator can delete messages of other users.
    pub can_delete_messages: bool,
    /// Indicates whether the administrator can delete stories posted by other users;
    /// channels only.
    pub can_delete_stories: Option<bool>,
    /// Indicates whether the administrator can edit messages
    /// of other users and can pin messages; channels only.
    pub can_edit_messages: Option<bool>,
    /// Indicates whether the administrator can edit stories posted by other users; channels only.
    pub can_edit_stories: Option<bool>,
    /// Indicates whether the administrator can invite new users to the chat.
    pub can_invite_users: bool,
    /// Indicates whether the administrator can access the chat event log,
    /// chat statistics, message statistics in channels, see channel members,
    /// see anonymous administrators in supergroups and ignore slow mode;
    /// implied by any other administrator privilege.
    pub can_manage_chat: bool,
    /// Indicates whether the administrator is allowed to
    /// create, rename, close, and reopen forum topics; supergroups only.
    pub can_manage_topics: Option<bool>,
    /// Indicates whether the administrator can manage video chats.
    pub can_manage_video_chats: bool,
    /// Indicates whether the administrator can pin messages; groups and supergroups only.
    pub can_pin_messages: Option<bool>,
    /// Indicates whether the administrator can post in the channel; channels only.
    pub can_post_messages: Option<bool>,
    /// Indicates whether the administrator can post stories in the channel; channels only.
    pub can_post_stories: Option<bool>,
    /// Indicates whether the administrator can add new administrators with a subset
    /// of his own privileges or demote administrators that he has promoted,
    /// directly or indirectly (promoted by administrators that were appointed by the user).
    pub can_promote_members: bool,
    /// Indicates whether the administrator can restrict, ban or unban chat members.
    pub can_restrict_members: bool,
    /// Custom title for the administrator.
    pub custom_title: Option<String>,
    /// Indicates whether the administrator's presence in the chat is hidden.
    pub is_anonymous: bool,
}

impl ChatMemberAdministrator {
    /// Creates a new `ChatMemberAdministrator`
    ///
    /// # Arguments
    ///
    /// * `user` - Information about the user.
    pub fn new(user: User) -> Self {
        Self {
            user,
            can_be_edited: false,
            can_change_info: false,
            can_delete_messages: false,
            can_delete_stories: None,
            can_edit_messages: None,
            can_edit_stories: None,
            can_invite_users: false,
            can_manage_chat: false,
            can_manage_topics: None,
            can_manage_video_chats: false,
            can_pin_messages: None,
            can_post_messages: None,
            can_post_stories: None,
            can_promote_members: false,
            can_restrict_members: false,
            is_anonymous: false,
            custom_title: None,
        }
    }

    /// Sets a new value for a `can_be_edited` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a bot is allowed to edit privileges of that administrator.
    pub fn with_can_be_edited(mut self, value: bool) -> Self {
        self.can_be_edited = value;
        self
    }

    /// Sets a new value for a `can_change_info` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can change the chat title,
    ///             photo and other settings.
    pub fn with_can_change_info(mut self, value: bool) -> Self {
        self.can_change_info = value;
        self
    }

    /// Sets a new value for a `can_delete_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can delete messages of other users.
    pub fn with_can_delete_messages(mut self, value: bool) -> Self {
        self.can_delete_messages = value;
        self
    }

    /// Sets a new value for a `can_delete_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator
    ///             can delete stories posted by other users; channels only.
    pub fn with_can_delete_stories(mut self, value: bool) -> Self {
        self.can_delete_stories = Some(value);
        self
    }

    /// Sets a new value for a `can_edit_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can edit messages
    ///             of other users and can pin messages; channels only.
    pub fn with_can_edit_messages(mut self, value: bool) -> Self {
        self.can_edit_messages = Some(value);
        self
    }

    /// Sets a new value for a `can_edit_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///             edit stories posted by other users; channels only.
    pub fn with_can_edit_stories(mut self, value: bool) -> Self {
        self.can_edit_stories = Some(value);
        self
    }

    /// Sets a new value for a `can_invite_users` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can invite new users to the chat.
    pub fn with_can_invite_users(mut self, value: bool) -> Self {
        self.can_invite_users = value;
        self
    }

    /// Sets a new value for a `can_manage_chat` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can access the chat event log,
    ///             chat statistics, message statistics in channels, see channel members,
    ///             see anonymous administrators in supergroups and ignore slow mode;
    ///             implied by any other administrator privilege.
    pub fn with_can_manage_chat(mut self, value: bool) -> Self {
        self.can_manage_chat = value;
        self
    }

    /// Sets a new value for a `can_manage_topics` flag.
    ///
    /// # Arguments
    ///
    /// * value -  Indicates whether the administrator is allowed to
    ///            create, rename, close, and reopen forum topics; supergroups only.
    pub fn with_can_manage_topics(mut self, value: bool) -> Self {
        self.can_manage_topics = Some(value);
        self
    }

    /// Sets a new value for a `can_manage_video_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can manage video chats.
    pub fn with_can_manage_video_chats(mut self, value: bool) -> Self {
        self.can_manage_video_chats = value;
        self
    }

    /// Sets a new value for a `can_pin_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can pin messages;
    ///             groups and supergroups only.
    pub fn with_can_pin_messages(mut self, value: bool) -> Self {
        self.can_pin_messages = Some(value);
        self
    }

    /// Sets a new value for a `can_post_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can post in the channel; channels only.
    pub fn with_can_post_messages(mut self, value: bool) -> Self {
        self.can_post_messages = Some(value);
        self
    }

    /// Sets a new value for a `can_post_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///             post stories in the channel; channels only.
    pub fn with_can_post_stories(mut self, value: bool) -> Self {
        self.can_post_stories = Some(value);
        self
    }

    /// Sets a new value for a `can_promote_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can add new administrators with a subset
    ///             of his own privileges or demote administrators that he has promoted,
    ///             directly or indirectly (promoted by administrators
    ///             that were appointed by the user).
    pub fn with_can_promote_members(mut self, value: bool) -> Self {
        self.can_promote_members = value;
        self
    }

    /// Sets a new value for a `can_restrict_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can restrict, ban or unban chat members.
    pub fn with_can_restrict_members(mut self, value: bool) -> Self {
        self.can_restrict_members = value;
        self
    }

    /// Sets a new custom title.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom title for the administrator.
    pub fn with_custom_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_title = Some(value.into());
        self
    }

    /// Sets a new value for an `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator's presence in the chat is hidden.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.is_anonymous = value;
        self
    }
}

/// Represents a chat member that owns the chat and has all administrator privileges.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatMemberCreator {
    /// Indicates whether the creator's presence in the chat is hidden.
    pub is_anonymous: bool,
    /// Information about the user.
    pub user: User,
    /// Custom title for the creator.
    pub custom_title: Option<String>,
}

impl ChatMemberCreator {
    /// Creates a new `ChatMemberCreator`.
    ///
    /// # Arguments
    ///
    /// * `user` - Information about the user.
    pub fn new(user: User) -> Self {
        Self {
            is_anonymous: false,
            user,
            custom_title: None,
        }
    }

    /// Sets a new custom title.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom title for the creator.
    pub fn with_custom_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_title = Some(value.into());
        self
    }

    /// Sets a new value of an `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the creator's presence in the chat is hidden.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.is_anonymous = value;
        self
    }
}

/// Represents a kicked chat member.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatMemberKicked {
    /// Date when restrictions will be lifted for this user; unix time.
    pub until_date: Integer,
    /// Information about the user.
    pub user: User,
}

impl ChatMemberKicked {
    /// Creates a new `ChatMemberKicked`.
    ///
    /// # Arguments
    ///
    /// * `until_date` - Date when restrictions will be lifted for this user; unix time.
    /// * `user` - Information about the user.
    pub fn new(until_date: Integer, user: User) -> Self {
        Self { user, until_date }
    }
}

/// Represents a restricted user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatMemberRestricted {
    /// Information about the user.
    pub user: User,
    /// Indicates whether the user may add web page previews to his messages.
    pub can_add_web_page_previews: bool,
    /// Indicates whether the user allowed to change the chat title, photo and other settings.
    pub can_change_info: bool,
    /// Indicates whether the user allowed to invite new users to the chat.
    pub can_invite_users: bool,
    /// Indicates whether the user is allowed to create forum topics.
    pub can_manage_topics: bool,
    /// Indicates whether the user allowed to pin messages; groups and supergroups only.
    pub can_pin_messages: Option<bool>,
    /// Indicates whether the user is allowed to send audios.
    pub can_send_audios: Option<bool>,
    /// Indicates whether the user is allowed to send documents.
    pub can_send_documents: Option<bool>,
    /// Indicates whether the user can send text messages, contacts, locations and venues.
    pub can_send_messages: bool,
    /// Indicates whether the user can send animations, games, stickers and use inline bots.
    pub can_send_other_messages: bool,
    /// Indicates whether the user is allowed to send photos.
    pub can_send_photos: Option<bool>,
    /// Indicates whether the user is allowed to send polls.
    pub can_send_polls: bool,
    /// Indicates whether the user is allowed to send video notes.
    pub can_send_video_notes: Option<bool>,
    /// Indicates whether the user is allowed to send videos.
    pub can_send_videos: Option<bool>,
    /// Indicates whether the user is allowed to send voice notes.
    pub can_send_voice_notes: Option<bool>,
    /// Indicates whether the user is a member of the chat at the moment of the request.
    pub is_member: bool,
    /// Date when restrictions will be lifted for this user; unix time.
    pub until_date: Integer,
}

impl ChatMemberRestricted {
    /// Creates a new `ChatMemberRestricted`
    ///
    /// # Arguments
    ///
    /// * `user` - Information about the user.
    /// * `until_date` - Date when restrictions will be lifted for this user; unix time.
    pub fn new(user: User, until_date: Integer) -> Self {
        Self {
            user,
            can_add_web_page_previews: false,
            can_change_info: false,
            can_invite_users: false,
            can_manage_topics: false,
            can_pin_messages: None,
            can_send_audios: None,
            can_send_documents: None,
            can_send_messages: false,
            can_send_other_messages: false,
            can_send_photos: None,
            can_send_polls: false,
            can_send_video_notes: None,
            can_send_videos: None,
            can_send_voice_notes: None,
            is_member: false,
            until_date,
        }
    }

    /// Sets a new value for a `can_add_web_page_previews` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user may add web page previews to his messages.
    pub fn with_can_add_web_page_previews(mut self, value: bool) -> Self {
        self.can_add_web_page_previews = value;
        self
    }

    /// Sets a new value for a `can_change_info` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user allowed to change the chat title,
    ///             photo and other settings.
    pub fn with_can_change_info(mut self, value: bool) -> Self {
        self.can_change_info = value;
        self
    }

    /// Sets a new value for a `can_invite_users` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user allowed to invite new users to the chat.
    pub fn with_can_invite_users(mut self, value: bool) -> Self {
        self.can_invite_users = value;
        self
    }

    /// Sets a new value for a `can_manage_topics` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to create forum topics.
    pub fn with_can_manage_topics(mut self, value: bool) -> Self {
        self.can_manage_topics = value;
        self
    }

    /// Sets a new value for a `can_pin_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user allowed to pin messages;
    ///             groups and supergroups only.
    pub fn with_can_pin_messages(mut self, value: bool) -> Self {
        self.can_pin_messages = Some(value);
        self
    }

    /// Sets a new value for a `can_send_audios` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to send audios.
    pub fn with_can_send_audios(mut self, value: bool) -> Self {
        self.can_send_audios = Some(value);
        self
    }

    /// Sets a new value for a `can_send_documents` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to send documents.
    pub fn with_can_send_documents(mut self, value: bool) -> Self {
        self.can_send_documents = Some(value);
        self
    }

    /// Sets a new value for a `can_send_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user can send text messages,
    ///             contacts, locations and venues.
    pub fn with_can_send_messages(mut self, value: bool) -> Self {
        self.can_send_messages = value;
        self
    }

    /// Sets a new value for a `can_send_other_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user can send animations,
    ///             games, stickers and use inline bots.
    pub fn with_can_send_other_messages(mut self, value: bool) -> Self {
        self.can_send_other_messages = value;
        self
    }

    /// Sets a new value for a `can_send_photos` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to send photos.
    pub fn with_can_send_photos(mut self, value: bool) -> Self {
        self.can_send_photos = Some(value);
        self
    }

    /// Sets a new value for a `can_send_polls` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to send polls.
    pub fn with_can_send_polls(mut self, value: bool) -> Self {
        self.can_send_polls = value;
        self
    }

    /// Sets a new value for a `can_send_video_notes` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to send video notes.
    pub fn with_can_send_video_notes(mut self, value: bool) -> Self {
        self.can_send_video_notes = Some(value);
        self
    }

    /// Sets a new value for a `can_send_videos` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to send videos.
    pub fn with_can_send_videos(mut self, value: bool) -> Self {
        self.can_send_videos = Some(value);
        self
    }

    /// Sets a new value for a `can_send_voice_notes` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is allowed to send voice notes.
    pub fn with_can_send_voice_notes(mut self, value: bool) -> Self {
        self.can_send_voice_notes = Some(value);
        self
    }

    /// Sets a new value for an `is_member` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is a member of the chat at the moment of the request.
    pub fn with_is_member(mut self, value: bool) -> Self {
        self.is_member = value;
        self
    }
}

/// Represents changes in a status of a chat member.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to.
    pub chat: Chat,
    /// Date the change was done in Unix time.
    pub date: Integer,
    /// Performer of the action, which resulted in the change.
    pub from: User,
    /// New information about the chat member.
    pub new_chat_member: ChatMember,
    /// Previous information about the chat member.
    pub old_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat;
    /// for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
    /// Indicates whether the user joined the chat via a chat folder invite link.
    pub via_chat_folder_invite_link: Option<bool>,
    /// Indicates whether the user joined the chat after sending a direct join request
    /// and being approved by an administrator.
    pub via_join_request: Option<bool>,
}

impl ChatMemberUpdated {
    /// Creates a new `ChatMemberUpdated`.
    ///
    /// # Arguments
    ///
    /// * `chat` - Chat the user belongs to.
    /// * `date` - Date the change was done in Unix time.
    /// * `from` - Performer of the action, which resulted in the change.
    /// * `new_chat_member` - New information about the chat member.
    /// * `old_chat_member` - Previous information about the chat member.
    pub fn new(
        chat: Chat,
        date: Integer,
        from: User,
        new_chat_member: ChatMember,
        old_chat_member: ChatMember,
    ) -> Self {
        Self {
            chat,
            date,
            from,
            new_chat_member,
            old_chat_member,
            invite_link: None,
            via_chat_folder_invite_link: None,
            via_join_request: None,
        }
    }

    /// Sets a new invite link
    ///
    /// # Arguments
    ///
    /// * `value` - The invite link, which was used by the user to join the chat.
    pub fn with_invite_link(mut self, value: ChatInviteLink) -> Self {
        self.invite_link = Some(value);
        self
    }

    /// Sets a new value for a `via_chat_folder_invite_link` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user joined the chat via a chat folder invite link.
    pub fn with_via_chat_folder_invite_link(mut self, value: bool) -> Self {
        self.via_chat_folder_invite_link = Some(value);
        self
    }

    /// Sets a new value for a `via_join_request` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user joined the chat after sending a direct join request
    ///             and being approved by an administrator.
    pub fn with_via_join_request(mut self, value: bool) -> Self {
        self.via_join_request = Some(value);
        self
    }
}

/// Bans a user in a chat.
///
/// In the case of supergroups and channels,
/// the user will not be able to return to the chat on their own using invite links,
/// etc., unless unbanned first.
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct BanChatMember {
    chat_id: ChatId,
    user_id: Integer,
    revoke_messages: Option<bool>,
    until_date: Option<Integer>,
}

impl BanChatMember {
    /// Creates a new `BanChatMember`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `user_id` - Unique identifier of the target user.
    pub fn new<T>(chat_id: T, user_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }

    /// Sets a new value for a `revoke_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Delete all messages from the chat for the user that is being removed;
    ///             if `false`, the user will be able to see messages in the group that were
    ///             sent before the user was removed;
    ///             always `true` for supergroups and channels.
    pub fn with_revoke_messages(mut self, value: bool) -> Self {
        self.revoke_messages = Some(value);
        self
    }

    /// Sets a new until date.
    ///
    /// # Arguments
    ///
    /// * `value` - The date when the user will be unbanned, unix time.
    ///
    /// If user is banned for more than 366 days or less than 30 seconds
    /// from the current time they are considered to be banned forever.
    pub fn with_until_date(mut self, value: Integer) -> Self {
        self.until_date = Some(value);
        self
    }
}

impl Method for BanChatMember {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("banChatMember", self)
    }
}

/// Returns a list of administrators in a chat.
///
/// On success, returns an array of [`ChatMember`] objects that contains
/// information about all chat administrators except other bots.
///
/// If the chat is a group or a supergroup and no administrators
/// were appointed, only the creator will be returned.
#[derive(Clone, Debug, Serialize)]
pub struct GetChatAdministrators {
    chat_id: ChatId,
}

impl GetChatAdministrators {
    /// Creates a new `GetChatAdministrators`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChatAdministrators {
    type Response = Vec<ChatMember>;

    fn into_payload(self) -> Payload {
        Payload::json("getChatAdministrators", self)
    }
}

/// Returns a member of a chat.
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMember {
    chat_id: ChatId,
    user_id: Integer,
}

impl GetChatMember {
    /// Creates a new `GetChatMember`
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `user_id` - Unique identifier of the target user.
    pub fn new<T>(chat_id: T, user_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl Method for GetChatMember {
    type Response = ChatMember;

    fn into_payload(self) -> Payload {
        Payload::json("getChatMember", self)
    }
}

/// Returns a number of members in a chat.
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMemberCount {
    chat_id: ChatId,
}

impl GetChatMemberCount {
    /// Creates a new `GetChatMemberCount`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChatMemberCount {
    type Response = Integer;

    fn into_payload(self) -> Payload {
        Payload::json("getChatMemberCount", self)
    }
}

/// Promotes or demotes a user in a chat.
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct PromoteChatMember {
    chat_id: ChatId,
    user_id: Integer,
    can_change_info: Option<bool>,
    can_delete_messages: Option<bool>,
    can_delete_stories: Option<bool>,
    can_edit_messages: Option<bool>,
    can_edit_stories: Option<bool>,
    can_invite_users: Option<bool>,
    can_manage_chat: Option<bool>,
    can_manage_topics: Option<bool>,
    can_manage_video_chats: Option<bool>,
    can_pin_messages: Option<bool>,
    can_post_messages: Option<bool>,
    can_post_stories: Option<bool>,
    can_promote_members: Option<bool>,
    can_restrict_members: Option<bool>,
    is_anonymous: Option<bool>,
}

impl PromoteChatMember {
    /// Creates a new `PromoteChatMember`
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `user_id` - Unique identifier of the target user.
    pub fn new<T>(chat_id: T, user_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            user_id,
            can_change_info: None,
            can_delete_messages: None,
            can_delete_stories: None,
            can_edit_messages: None,
            can_edit_stories: None,
            can_invite_users: None,
            can_manage_chat: None,
            can_manage_topics: None,
            can_manage_video_chats: None,
            can_pin_messages: None,
            can_post_messages: None,
            can_post_stories: None,
            can_promote_members: None,
            can_restrict_members: None,
            is_anonymous: None,
        }
    }

    /// Promotes all privileges.
    pub fn promote_all(mut self) -> Self {
        self.is_anonymous = Some(true);
        self.can_change_info = Some(true);
        self.can_delete_messages = Some(true);
        self.can_delete_stories = Some(true);
        self.can_edit_messages = Some(true);
        self.can_edit_stories = Some(true);
        self.can_invite_users = Some(true);
        self.can_manage_chat = Some(true);
        self.can_manage_topics = Some(true);
        self.can_manage_video_chats = Some(true);
        self.can_pin_messages = Some(true);
        self.can_post_messages = Some(true);
        self.can_post_stories = Some(true);
        self.can_promote_members = Some(true);
        self.can_restrict_members = Some(true);
        self
    }

    /// Demotes all privileges.
    pub fn demote_all(mut self) -> Self {
        self.is_anonymous = Some(false);
        self.can_change_info = Some(false);
        self.can_delete_messages = Some(false);
        self.can_delete_stories = Some(false);
        self.can_edit_messages = Some(false);
        self.can_edit_stories = Some(false);
        self.can_invite_users = Some(false);
        self.can_manage_chat = Some(false);
        self.can_manage_topics = Some(false);
        self.can_manage_video_chats = Some(false);
        self.can_pin_messages = Some(false);
        self.can_post_messages = Some(false);
        self.can_post_stories = Some(false);
        self.can_promote_members = Some(false);
        self.can_restrict_members = Some(false);
        self
    }

    /// Sets a new value for a `can_change_info` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can change chat title,
    ///             photo and other settings.
    pub fn with_can_change_info(mut self, value: bool) -> Self {
        self.can_change_info = Some(value);
        self
    }

    /// Sets a new value for a `can_delete_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can delete messages of other users.
    pub fn with_can_delete_messages(mut self, value: bool) -> Self {
        self.can_delete_messages = Some(value);
        self
    }

    /// Sets a new value for a `can_delete_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can delete stories posted by other users;
    ///             channels only.
    pub fn with_can_delete_stories(mut self, value: bool) -> Self {
        self.can_delete_stories = Some(value);
        self
    }

    /// Sets a new value for a `can_edit_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can edit messages
    ///             of other users and can pin messages;
    ///             channels only.
    pub fn with_can_edit_messages(mut self, value: bool) -> Self {
        self.can_edit_messages = Some(value);
        self
    }

    /// Sets a new value for a `can_edit_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can
    ///             edit stories posted by other users; channels only.
    pub fn with_can_edit_stories(mut self, value: bool) -> Self {
        self.can_edit_stories = Some(value);
        self
    }

    /// Sets a new value for a `can_invite_users` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can invite new users to the chat.
    pub fn with_can_invite_users(mut self, value: bool) -> Self {
        self.can_invite_users = Some(value);
        self
    }

    /// Sets a new value for a `can_manage_chat` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can access the chat event log,
    ///             chat statistics, message statistics in channels, see channel members,
    ///             see anonymous administrators in supergroups and ignore slow mode;
    ///             implied by any other administrator privilege.
    pub fn with_can_manage_chat(mut self, value: bool) -> Self {
        self.can_manage_chat = Some(value);
        self
    }

    /// Sets a new value for a `can_manage_topics` flag.
    ///
    /// # Arguments
    ///
    /// * value - User is allowed to create, rename, close, and reopen forum topics;
    ///           supergroups only.
    pub fn with_can_manage_topics(mut self, value: bool) -> Self {
        self.can_manage_topics = Some(value);
        self
    }

    /// Sets a new value for a `can_manage_video_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can manage video chats;
    ///             supergroups only.
    pub fn with_can_manage_video_chats(mut self, value: bool) -> Self {
        self.can_manage_video_chats = Some(value);
        self
    }

    /// Sets a new value for a `can_pin_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can pin messages;
    ///             supergroups only.
    pub fn with_can_pin_messages(mut self, value: bool) -> Self {
        self.can_pin_messages = Some(value);
        self
    }

    /// Sets a new value for a `can_post_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can create channel posts;
    ///             channels only.
    pub fn with_can_post_messages(mut self, value: bool) -> Self {
        self.can_post_messages = Some(value);
        self
    }

    /// Sets a new value for a `can_post_stories` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can post stories in the channel;
    ///             channels only.
    pub fn with_can_post_stories(mut self, value: bool) -> Self {
        self.can_post_stories = Some(value);
        self
    }

    /// Sets a new value for a `can_promote_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can add new administrators with a subset
    ///             of his own privileges or demote administrators that he has promoted,
    ///             directly or indirectly (promoted by administrators that were appointed by him).
    pub fn with_can_promote_members(mut self, value: bool) -> Self {
        self.can_promote_members = Some(value);
        self
    }

    /// Sets a new value for a `can_restrict_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the administrator can restrict, ban or unban chat members.
    pub fn with_can_restrict_members(mut self, value: bool) -> Self {
        self.can_restrict_members = Some(value);
        self
    }

    /// Sets a new value for an `is_anonymous` flag.
    ///
    /// # Arguments
    ///
    /// * value - Indicates whether the administrator's presence in the chat is hidden.
    pub fn with_is_anonymous(mut self, value: bool) -> Self {
        self.is_anonymous = Some(value);
        self
    }
}

impl Method for PromoteChatMember {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("promoteChatMember", self)
    }
}

/// Restricts a user in a supergroup.
///
/// The bot must be an administrator in the supergroup
/// for this to work and must have the appropriate admin rights.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct RestrictChatMember {
    chat_id: ChatId,
    permissions: ChatPermissions,
    user_id: Integer,
    until_date: Option<Integer>,
    use_independent_chat_permissions: Option<bool>,
}

impl RestrictChatMember {
    /// Creates a new `RestrictChatMember`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `user_id` - Unique identifier of the target user.
    pub fn new<T>(chat_id: T, user_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        RestrictChatMember {
            chat_id: chat_id.into(),
            permissions: ChatPermissions::default(),
            user_id,
            until_date: None,
            use_independent_chat_permissions: None,
        }
    }

    /// Allows everything.
    pub fn allow_all(mut self) -> Self {
        self.permissions = ChatPermissions::allowed();
        self
    }

    /// Restricts everything.
    pub fn restrict_all(mut self) -> Self {
        self.permissions = ChatPermissions::restricted();
        self
    }

    /// Replaces current permissions with the new one.
    ///
    /// # Arguments
    ///
    /// * value - The new permissions.
    pub fn with_permissions(mut self, value: ChatPermissions) -> Self {
        self.permissions = value;
        self
    }

    /// Sets a new until date.
    ///
    /// # Arguments
    ///
    /// * value - The date when restrictions will be lifted for the user, unix time.
    ///
    /// If user is restricted for more than 366 days or less than 30 seconds
    /// from the current time, they are considered to be restricted forever.
    pub fn with_until_date(mut self, value: Integer) -> Self {
        self.until_date = Some(value);
        self
    }

    /// Sets a new value for a `use_independent_chat_permissions` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the chat permissions are set independently.
    ///
    /// If `false`, the `can_send_other_messages` and `can_add_web_page_previews` permissions
    /// will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`,
    /// `can_send_photos`, `can_send_videos`, `can_send_video_notes`,
    /// and `can_send_voice_notes` permissions; the `can_send_polls` permission
    /// will imply the `can_send_messages` permission.
    pub fn with_use_independent_chat_permissions(mut self, value: bool) -> Self {
        self.use_independent_chat_permissions = Some(value);
        self
    }
}

impl Method for RestrictChatMember {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("restrictChatMember", self)
    }
}

/// Sets a custom title for an administrator in a supergroup promoted by the bot.
#[derive(Clone, Debug, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    chat_id: ChatId,
    user_id: Integer,
    custom_title: String,
}

impl SetChatAdministratorCustomTitle {
    /// Creates a new `SetChatAdministratorCustomTitle`.
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier of the target chat.
    /// * custom_title - New custom title for the administrator;
    ///                  0-16 characters; emoji are not allowed.
    /// * user_id - Unique identifier of the target user.
    pub fn new<A, B>(chat_id: A, custom_title: B, user_id: Integer) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            custom_title: custom_title.into(),
            user_id,
        }
    }
}

impl Method for SetChatAdministratorCustomTitle {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatAdministratorCustomTitle", self)
    }
}

/// Unbans a previously kicked user in a supergroup or channel.
///
/// The user will not return to the group or channel
/// automatically, but will be able to join via link, etc.
///
/// The bot must be an administrator for this to work
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct UnbanChatMember {
    chat_id: ChatId,
    user_id: Integer,
    only_if_banned: Option<bool>,
}

impl UnbanChatMember {
    /// Creates a new `UnbanChatMember`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `user_id` - Unique identifier of the target user.
    pub fn new<T>(chat_id: T, user_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        UnbanChatMember {
            chat_id: chat_id.into(),
            user_id,
            only_if_banned: None,
        }
    }

    /// Sets a new value for an `only_if_banned` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - If `true` - do nothing if the user is not banned.
    pub fn with_only_if_banned(mut self, only_if_banned: bool) -> Self {
        self.only_if_banned = Some(only_if_banned);
        self
    }
}

impl Method for UnbanChatMember {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("unbanChatMember", self)
    }
}

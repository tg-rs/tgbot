use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    api::{Method, Payload},
    types::{Chat, ChatId, ChatInviteLink, ChatPermissions, Integer, User},
};

#[cfg(test)]
mod tests;

/// Information about one member of a chat
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "status")]
pub enum ChatMember {
    /// Chat admin
    Administrator(ChatMemberAdministrator),
    /// Chat creator
    Creator(ChatMemberCreator),
    /// Kicked user
    Kicked(ChatMemberKicked),
    /// Left user
    #[serde(deserialize_with = "ChatMemberUser::deserialize_value")]
    #[serde(serialize_with = "ChatMemberUser::serialize_value")]
    Left(User),
    /// Chat member
    #[serde(deserialize_with = "ChatMemberUser::deserialize_value")]
    #[serde(serialize_with = "ChatMemberUser::serialize_value")]
    Member(User),
    /// Restricted user
    Restricted(ChatMemberRestricted),
}

impl ChatMember {
    /// Returns a user object
    pub fn get_user(&self) -> &User {
        use self::ChatMember::*;
        match self {
            Administrator(ref admin) => &admin.user,
            Creator(ref creator) => &creator.user,
            Kicked(ref kicked) => &kicked.user,
            Left(ref user) => user,
            Member(ref user) => user,
            Restricted(ref restricted) => &restricted.user,
        }
    }

    /// Whether a user is a member of the chat
    pub fn is_member(&self) -> bool {
        use self::ChatMember::*;
        match self {
            Administrator(_) | Creator(_) | Member(_) => true,
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
    fn deserialize_value<'de, D>(deserializer: D) -> Result<User, D::Error>
    where
        D: Deserializer<'de>,
    {
        ChatMemberUser::deserialize(deserializer).map(|x| x.user)
    }

    fn serialize_value<S>(value: &User, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = ChatMemberUser { user: value.clone() };
        value.serialize(serializer)
    }
}

/// Chat admin
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatMemberAdministrator {
    /// Information about the user
    pub user: User,
    /// True, if the bot is allowed
    /// to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can access the chat event log,
    /// chat statistics, message statistics in channels, see channel members,
    /// see anonymous administrators in supergroups and ignore slow mode;
    /// implied by any other administrator privilege
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// True, if the administrator can
    /// add new administrators with a subset
    /// of his own privileges or
    /// demote administrators that he has promoted,
    /// directly or indirectly
    /// (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the administrator can change
    /// the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the administrator can invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can post
    /// in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// True, if the administrator can edit messages
    /// of other users and can pin messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// True, if the administrator can pin messages; groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// True, if the user is allowed to
    /// create, rename, close, and reopen forum topics; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Custom title for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

/// Represents a chat member that owns the chat and has all administrator privileges.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatMemberCreator {
    /// Information about the user
    pub user: User,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Custom title for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

/// Kicked user
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatMemberKicked {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user, unix time
    pub until_date: Integer,
}

/// Restricted user
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatMemberRestricted {
    /// Information about the user
    pub user: User,
    /// True, if the user is a member
    /// of the chat at the moment of the request
    pub is_member: bool,
    /// True, if the user allowed to change
    /// the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the user allowed to pin messages; groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// True, if the user can send
    /// text messages, contacts, locations and venues
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    /// True, if the user is allowed to send documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    /// True, if the user is allowed to send photos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    /// True, if the user is allowed to send videos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    /// True, if the user is allowed to send video notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    /// True, if the user is allowed to send voice notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    /// True, if the user is allowed to send polls
    pub can_send_polls: bool,
    /// True, if the user can send
    /// animations, games, stickers
    /// and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: bool,
    /// True, if user may add web page previews
    /// to his messages, implies can_send_media_messages
    pub can_add_web_page_previews: bool,
    /// True, if the user is allowed to create forum topics
    pub can_manage_topics: bool,
    /// Date when restrictions will be lifted for this user, unix time
    pub until_date: Integer,
}

/// Represents changes in the status of a chat member
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: Integer,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat;
    /// for joining by invite link events only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    /// True, if the user joined the chat via a chat folder invite link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}

/// Ban a user in a group, a supergroup or a channel
///
/// In the case of supergroups and channels,
/// the user will not be able to return to the chat on their own using invite links,
/// etc., unless unbanned first. The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights
#[derive(Clone, Debug, Serialize)]
pub struct BanChatMember {
    chat_id: ChatId,
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoke_messages: Option<bool>,
}

impl BanChatMember {
    /// Creates a new BanChatMember
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        BanChatMember {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }

    /// Date when the user will be unbanned, unix time
    ///
    /// If user is banned for more than 366 days or less than 30 seconds
    /// from the current time they are considered to be banned forever
    pub fn until_date(mut self, until_date: Integer) -> Self {
        self.until_date = Some(until_date);
        self
    }

    /// Delete all messages from the chat for the user that is being removed
    ///
    /// If False, the user will be able to see messages in the group that were
    /// sent before the user was removed.
    /// Always True for supergroups and channels.
    pub fn revoke_messages(mut self, revoke_messages: bool) -> Self {
        self.revoke_messages = Some(revoke_messages);
        self
    }
}

impl Method for BanChatMember {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("banChatMember", self)
    }
}

/// Get a list of administrators in a chat
///
/// On success, returns an Array of ChatMember objects that contains
/// information about all chat administrators except other bots
/// If the chat is a group or a supergroup and no administrators
/// were appointed, only the creator will be returned
#[derive(Clone, Debug, Serialize)]
pub struct GetChatAdministrators {
    chat_id: ChatId,
}

impl GetChatAdministrators {
    /// Creates a new GetChatAdministrators
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChatAdministrators {
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

/// Get information about a member of a chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMember {
    chat_id: ChatId,
    user_id: Integer,
}

impl GetChatMember {
    /// Creates a new GetChatMember
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        GetChatMember {
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

/// Get the number of members in a chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMemberCount {
    chat_id: ChatId,
}

impl GetChatMemberCount {
    /// Creates a new GetChatMemberCount
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChatMemberCount {
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

/// Promote or demote a user in a supergroup or a channel
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights
/// Pass False for all boolean parameters to demote a user
#[derive(Clone, Debug, Serialize)]
pub struct PromoteChatMember {
    chat_id: ChatId,
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_video_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_topics: Option<bool>,
}

impl PromoteChatMember {
    /// Creates a new PromoteChatMember with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        PromoteChatMember {
            chat_id: chat_id.into(),
            user_id,
            is_anonymous: None,
            can_change_info: None,
            can_delete_messages: None,
            can_edit_messages: None,
            can_invite_users: None,
            can_manage_chat: None,
            can_manage_video_chats: None,
            can_pin_messages: None,
            can_post_messages: None,
            can_promote_members: None,
            can_restrict_members: None,
            can_manage_topics: None,
        }
    }

    /// Promote all privileges
    pub fn promote_all(mut self) -> Self {
        self.is_anonymous = Some(true);
        self.can_change_info = Some(true);
        self.can_delete_messages = Some(true);
        self.can_edit_messages = Some(true);
        self.can_invite_users = Some(true);
        self.can_manage_chat = Some(true);
        self.can_manage_video_chats = Some(true);
        self.can_pin_messages = Some(true);
        self.can_post_messages = Some(true);
        self.can_promote_members = Some(true);
        self.can_restrict_members = Some(true);
        self.can_manage_topics = Some(true);
        self
    }

    /// Demote all privileges
    pub fn demote_all(mut self) -> Self {
        self.is_anonymous = Some(false);
        self.can_change_info = Some(false);
        self.can_delete_messages = Some(false);
        self.can_edit_messages = Some(false);
        self.can_invite_users = Some(false);
        self.can_manage_chat = Some(false);
        self.can_manage_video_chats = Some(false);
        self.can_pin_messages = Some(false);
        self.can_post_messages = Some(false);
        self.can_promote_members = Some(false);
        self.can_restrict_members = Some(false);
        self.can_manage_topics = Some(false);
        self
    }

    /// Administrator's presence in the chat is hidden if true
    #[allow(clippy::wrong_self_convention)]
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }

    /// Administrator can change chat title, photo and other settings
    pub fn can_change_info(mut self, can_change_info: bool) -> Self {
        self.can_change_info = Some(can_change_info);
        self
    }

    /// Administrator can delete messages of other users
    pub fn can_delete_messages(mut self, can_delete_messages: bool) -> Self {
        self.can_delete_messages = Some(can_delete_messages);
        self
    }

    /// Administrator can edit messages of other users and can pin messages, channels only
    pub fn can_edit_messages(mut self, can_edit_messages: bool) -> Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }

    /// Administrator can invite new users to the chat
    pub fn can_invite_users(mut self, can_invite_users: bool) -> Self {
        self.can_invite_users = Some(can_invite_users);
        self
    }

    /// Administrator can access the chat event log, chat statistics,
    /// message statistics in channels, see channel members,
    /// see anonymous administrators in supergroups and ignore slow mode
    ///
    /// Implied by any other administrator privilege
    pub fn can_manage_chat(mut self, can_manage_chat: bool) -> Self {
        self.can_manage_chat = Some(can_manage_chat);
        self
    }

    /// Administrator can manage video chats, supergroups only
    pub fn can_manage_video_chats(mut self, can_manage_video_chats: bool) -> Self {
        self.can_manage_video_chats = Some(can_manage_video_chats);
        self
    }

    /// Administrator can pin messages, supergroups only
    pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
    }

    /// Administrator can create channel posts, channels only
    pub fn can_post_messages(mut self, can_post_messages: bool) -> Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }

    /// Administrator can add new administrators with a subset of his own privileges or demote administrators
    /// that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    pub fn can_promote_members(mut self, can_promote_members: bool) -> Self {
        self.can_promote_members = Some(can_promote_members);
        self
    }

    /// Administrator can restrict, ban or unban chat members
    pub fn can_restrict_members(mut self, can_restrict_members: bool) -> Self {
        self.can_restrict_members = Some(can_restrict_members);
        self
    }

    /// User is allowed to create, rename, close, and reopen forum topics, supergroups only
    pub fn can_manage_topics(mut self, can_manage_topics: bool) -> Self {
        self.can_manage_topics = Some(can_manage_topics);
        self
    }
}

impl Method for PromoteChatMember {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("promoteChatMember", self)
    }
}

/// Restrict a user in a supergroup
///
/// The bot must be an administrator in the supergroup
/// for this to work and must have the appropriate admin rights.
///
/// Pass True for all boolean parameters to lift restrictions from a user
#[derive(Clone, Debug, Serialize)]
pub struct RestrictChatMember {
    chat_id: ChatId,
    user_id: Integer,
    permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
}

impl RestrictChatMember {
    /// Creates a new RestrictChatMember with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        RestrictChatMember {
            chat_id: chat_id.into(),
            user_id,
            permissions: ChatPermissions::default(),
            until_date: None,
            use_independent_chat_permissions: None,
        }
    }

    /// Replace current permissions with the new one
    pub fn with_permissions(mut self, permissions: ChatPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Restrict everything
    pub fn restrict_all(mut self) -> Self {
        self.permissions = ChatPermissions::restricted();
        self
    }

    /// Allow everything
    pub fn allow_all(mut self) -> Self {
        self.permissions = ChatPermissions::allowed();
        self
    }

    /// Date when restrictions will be lifted for the user, unix time
    ///
    /// If user is restricted for more than 366 days or less than 30 seconds
    /// from the current time, they are considered to be restricted forever
    pub fn until_date(mut self, until_date: Integer) -> Self {
        self.until_date = Some(until_date);
        self
    }

    /// Pass True if chat permissions are set independently.
    ///
    /// Otherwise, the can_send_other_messages and can_add_web_page_previews permissions
    /// will imply the can_send_messages, can_send_audios, can_send_documents, can_send_photos,
    /// can_send_videos, can_send_video_notes, and can_send_voice_notes permissions;
    /// the can_send_polls permission will imply the can_send_messages permission.
    pub fn use_independent_chat_permissions(mut self, value: bool) -> Self {
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

/// Set a custom title for an administrator in a supergroup promoted by the bot
///
/// Returns True on success
#[derive(Clone, Debug, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    chat_id: ChatId,
    user_id: Integer,
    custom_title: String,
}

impl SetChatAdministratorCustomTitle {
    /// Creates a new SetChatAdministratorCustomTitle
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    /// * custom_title - New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub fn new<C, T>(chat_id: C, user_id: Integer, custom_title: T) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            user_id,
            custom_title: custom_title.into(),
        }
    }
}

impl Method for SetChatAdministratorCustomTitle {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatAdministratorCustomTitle", self)
    }
}

/// Unban a previously kicked user in a supergroup or channel
///
/// The user will not return to the group or channel
/// automatically, but will be able to join via link, etc.
///
/// The bot must be an administrator for this to work
#[derive(Clone, Debug, Serialize)]
pub struct UnbanChatMember {
    chat_id: ChatId,
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_if_banned: Option<bool>,
}

impl UnbanChatMember {
    /// Creates a new UnbanChatMember
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        UnbanChatMember {
            chat_id: chat_id.into(),
            user_id,
            only_if_banned: None,
        }
    }

    /// If true - do nothing if the user is not banned
    pub fn only_if_banned(mut self, only_if_banned: bool) -> Self {
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

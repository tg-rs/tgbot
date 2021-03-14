use crate::types::{
    chat::{Chat, ChatInviteLink},
    primitive::Integer,
    user::User,
};
use serde::{Deserialize, Deserializer};

/// Information about one member of a chat
#[derive(Clone, Debug, Deserialize)]
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
    #[serde(deserialize_with = "deserialize_chat_user")]
    Left(User),
    /// Chat member
    #[serde(deserialize_with = "deserialize_chat_user")]
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

fn deserialize_chat_user<'de, D>(deserializer: D) -> Result<User, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Inner {
        user: User,
    }

    Inner::deserialize(deserializer).map(|x| x.user)
}

/// Chat admin
#[derive(Clone, Debug, Deserialize)]
pub struct ChatMemberAdministrator {
    /// Information about the user
    pub user: User,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the bot is allowed
    /// to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// True, if the administrator can change
    /// the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can edit messages
    /// of other users and can pin messages; channels only
    pub can_edit_messages: Option<bool>,
    /// True, if the administrator can invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can access the chat event log,
    /// chat statistics, message statistics in channels, see channel members,
    /// see anonymous administrators in supergroups and ignore slow mode;
    /// implied by any other administrator privilege
    pub can_manage_chat: bool,
    /// True, if the administrator can manage voice chats
    pub can_manage_voice_chats: bool,
    /// True, if the administrator can pin messages; groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// True, if the administrator can post
    /// in the channel; channels only
    pub can_post_messages: Option<bool>,
    /// True, if the administrator can
    /// add new administrators with a subset
    /// of his own privileges or
    /// demote administrators that he has promoted,
    /// directly or indirectly
    /// (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// Custom title for this user
    pub custom_title: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChatMemberCreator {
    /// Information about the user
    pub user: User,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Custom title for this user
    pub custom_title: Option<String>,
}

/// Kicked user
#[derive(Clone, Debug, Deserialize)]
pub struct ChatMemberKicked {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user, unix time
    pub until_date: Integer,
}

/// Restricted user
#[derive(Clone, Debug, Deserialize)]
pub struct ChatMemberRestricted {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user, unix time
    pub until_date: Integer,
    /// True, if user may add web page previews
    /// to his messages, implies can_send_media_messages
    pub can_add_web_page_previews: bool,
    /// True, if the user allowed to change
    /// the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the user allowed to pin messages; groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// True, if the user can send
    /// audios, documents, photos, videos,
    /// video notes and voice notes, implies can_send_messages
    pub can_send_media_messages: bool,
    /// True, if the user can send
    /// text messages, contacts, locations and venues
    pub can_send_messages: bool,
    /// True, if the user can send
    /// animations, games, stickers
    /// and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: bool,
    /// True, if the user is allowed to send polls
    pub can_send_polls: bool,
    /// True, if the user is a member
    /// of the chat at the moment of the request
    pub is_member: bool,
}

/// Represents changes in the status of a chat member
#[derive(Clone, Debug, Deserialize)]
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
    pub invite_link: Option<ChatInviteLink>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_chat_member_admin_full() {
        let mut admin: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "administrator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname",
                "last_name": "lastname",
                "username": "username",
                "language_code": "RU"
            },
            "custom_title": "god",
            "is_anonymous": false,
            "can_be_edited": true,
            "can_change_info": false,
            "can_post_messages": true,
            "can_edit_messages": false,
            "can_delete_messages": true,
            "can_invite_users": false,
            "can_restrict_members": true,
            "can_pin_messages": false,
            "can_promote_members": true,
            "can_manage_voice_chats": false,
            "can_manage_chat": true
        }))
        .unwrap();
        assert!(admin.is_member());
        assert_eq!(admin.get_user().id, 1);
        if let ChatMember::Administrator(ref mut admin) = admin {
            assert!(!admin.is_anonymous);
            assert_eq!(admin.user.id, 1);
            assert!(!admin.user.is_bot);
            assert_eq!(admin.user.first_name, "firstname");
            assert_eq!(admin.user.last_name.take().unwrap(), "lastname");
            assert_eq!(admin.user.username.take().unwrap(), "username");
            assert_eq!(admin.user.language_code.take().unwrap(), "RU");
            assert_eq!(admin.custom_title.take().unwrap(), "god");
            assert!(admin.can_be_edited);
            assert!(!admin.can_change_info);
            assert_eq!(admin.can_post_messages, Some(true));
            assert_eq!(admin.can_edit_messages, Some(false));
            assert!(admin.can_delete_messages);
            assert!(!admin.can_invite_users);
            assert!(admin.can_restrict_members);
            assert_eq!(admin.can_pin_messages, Some(false));
            assert!(admin.can_promote_members);
            assert!(!admin.can_manage_voice_chats);
            assert!(admin.can_manage_chat);
        } else {
            panic!("Unexpected chat member: {:?}", admin);
        }
    }

    #[test]
    fn deserialize_chat_member_admin_partial() {
        let mut admin: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "administrator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname",
                "last_name": "lastname",
                "username": "username",
                "language_code": "RU"
            },
            "is_anonymous": false,
            "can_be_edited": true,
            "can_change_info": false,
            "can_delete_messages": true,
            "can_invite_users": false,
            "can_restrict_members": true,
            "can_promote_members": true,
            "can_manage_voice_chats": false,
            "can_manage_chat": true
        }))
        .unwrap();
        assert!(admin.is_member());
        assert_eq!(admin.get_user().id, 1);
        if let ChatMember::Administrator(ref mut admin) = admin {
            assert!(!admin.is_anonymous);
            assert_eq!(admin.user.id, 1);
            assert!(!admin.user.is_bot);
            assert_eq!(admin.user.first_name, "firstname");
            assert_eq!(admin.user.last_name.take().unwrap(), "lastname");
            assert_eq!(admin.user.username.take().unwrap(), "username");
            assert_eq!(admin.user.language_code.take().unwrap(), "RU");
            assert!(admin.custom_title.is_none());
            assert!(admin.can_be_edited);
            assert!(!admin.can_change_info);
            assert!(admin.can_delete_messages);
            assert!(admin.can_edit_messages.is_none());
            assert!(!admin.can_invite_users);
            assert!(admin.can_manage_chat);
            assert!(!admin.can_manage_voice_chats);
            assert!(admin.can_pin_messages.is_none());
            assert!(admin.can_post_messages.is_none());
            assert!(admin.can_promote_members);
            assert!(admin.can_restrict_members);
        } else {
            panic!("Unexpected chat member: {:?}", admin);
        }
    }

    #[test]
    fn deserialize_chat_member_creator_partial() {
        let mut creator: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "creator",
            "is_anonymous": false,
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            }
        }))
        .unwrap();
        assert!(creator.is_member());
        assert_eq!(creator.get_user().id, 1);
        if let ChatMember::Creator(ref mut creator) = creator {
            assert!(!creator.is_anonymous);
            assert_eq!(creator.user.id, 1);
            assert!(creator.custom_title.is_none());
        } else {
            panic!("Unexpected chat member: {:?}", creator);
        }
    }

    #[test]
    fn deserialize_chat_member_creator_full() {
        let mut creator: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "creator",
            "is_anonymous": false,
            "custom_title": "creator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            }
        }))
        .unwrap();
        assert!(creator.is_member());
        assert_eq!(creator.get_user().id, 1);
        if let ChatMember::Creator(ref mut creator) = creator {
            assert!(!creator.is_anonymous);
            assert_eq!(creator.user.id, 1);
            assert!(!creator.user.is_bot);
            assert_eq!(creator.user.first_name, String::from("firstname"));
            assert_eq!(creator.user.last_name, None);
            assert_eq!(creator.user.username, None);
            assert_eq!(creator.user.language_code, None);
            assert_eq!(creator.custom_title.take().unwrap(), "creator");
        } else {
            panic!("Unexpected chat member: {:?}", creator);
        }
    }

    #[test]
    fn deserialize_chat_member_kicked() {
        let mut kicked: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "kicked",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname",
                "last_name": "lastname",
                "username": "username"
            },
            "until_date": 0
        }))
        .unwrap();
        assert!(!kicked.is_member());
        assert_eq!(kicked.get_user().id, 1);
        if let ChatMember::Kicked(ref mut kicked) = kicked {
            assert_eq!(kicked.user.id, 1);
            assert!(kicked.user.is_bot);
            assert_eq!(kicked.user.first_name, "firstname");
            assert_eq!(kicked.user.last_name.take().unwrap(), "lastname");
            assert_eq!(kicked.user.username.take().unwrap(), "username");
            assert!(kicked.user.language_code.is_none());
            assert_eq!(kicked.until_date, 0);
        } else {
            panic!("Unexpected chat member: {:?}", kicked);
        }
    }

    #[test]
    fn deserialize_chat_member_left() {
        let left: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "left",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            }
        }))
        .unwrap();
        assert!(!left.is_member());
        assert_eq!(left.get_user().id, 1);
        if let ChatMember::Left(ref left) = left {
            assert_eq!(left.id, 1);
            assert!(left.is_bot);
            assert_eq!(left.first_name, "firstname");
            assert!(left.last_name.is_none());
            assert!(left.username.is_none());
            assert!(left.language_code.is_none());
        } else {
            panic!("Unexpected chat member: {:?}", left);
        }
    }

    #[test]
    fn deserialize_chat_member() {
        let member: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "member",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            }
        }))
        .unwrap();
        assert!(member.is_member());
        assert_eq!(member.get_user().id, 1);
        if let ChatMember::Member(ref member) = member {
            assert_eq!(member.id, 1);
            assert!(!member.is_bot);
            assert_eq!(member.first_name, "firstname");
            assert!(member.last_name.is_none());
            assert!(member.username.is_none());
            assert!(member.language_code.is_none());
        } else {
            panic!("Unexpected chat member: {:?}", member);
        }
    }

    #[test]
    fn deserialize_chat_member_restricted_full() {
        let restricted: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "restricted",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            },
            "until_date": 0,
            "can_change_info": true,
            "can_invite_users": false,
            "can_send_polls": true,
            "can_pin_messages": false,
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_other_messages": true,
            "can_add_web_page_previews": false,
            "is_member": true
        }))
        .unwrap();
        assert_eq!(restricted.get_user().id, 1);
        assert!(restricted.is_member());
        if let ChatMember::Restricted(ref restricted) = restricted {
            assert_eq!(restricted.user.id, 1);
            assert!(restricted.user.is_bot);
            assert_eq!(restricted.user.first_name, "firstname");
            assert!(restricted.user.last_name.is_none());
            assert!(restricted.user.username.is_none());
            assert!(restricted.user.language_code.is_none());
            assert_eq!(restricted.until_date, 0);
            assert!(!restricted.can_add_web_page_previews);
            assert!(restricted.can_change_info);
            assert!(!restricted.can_invite_users);
            assert!(!restricted.can_pin_messages.unwrap());
            assert!(!restricted.can_send_media_messages);
            assert!(restricted.can_send_messages);
            assert!(restricted.can_send_other_messages);
            assert!(restricted.can_send_polls);
            assert!(restricted.is_member);
        } else {
            panic!("Unexpected chat member: {:?}", restricted);
        }
    }

    #[test]
    fn deserialize_chat_member_restricted_partial() {
        let restricted: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "restricted",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            },
            "until_date": 0,
            "can_change_info": true,
            "can_invite_users": false,
            "can_send_polls": true,
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_other_messages": true,
            "can_add_web_page_previews": false,
            "is_member": true
        }))
        .unwrap();
        assert_eq!(restricted.get_user().id, 1);
        assert!(restricted.is_member());
        if let ChatMember::Restricted(ref restricted) = restricted {
            assert_eq!(restricted.user.id, 1);
            assert_eq!(restricted.until_date, 0);
            assert!(!restricted.can_add_web_page_previews);
            assert!(restricted.can_change_info);
            assert!(!restricted.can_invite_users);
            assert!(restricted.can_pin_messages.is_none());
            assert!(!restricted.can_send_media_messages);
            assert!(restricted.can_send_messages);
            assert!(restricted.can_send_other_messages);
            assert!(restricted.can_send_polls);
            assert!(restricted.is_member);
        } else {
            panic!("Unexpected chat member: {:?}", restricted);
        }
    }

    #[test]
    fn deserialize_chat_member_updated_full() {
        let data: ChatMemberUpdated = serde_json::from_value(serde_json::json!({
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle"
            },
            "from": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            },
            "date": 0,
            "old_chat_member": {
                "status": "member",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "firstname"
                }
            },
            "new_chat_member": {
                "status": "kicked",
                "user": {
                    "id": 2,
                    "is_bot": true,
                    "first_name": "firstname",
                },
                "until_date": 0
            },
            "invite_link": {
                "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
                "creator": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "firstname"
                },
                "is_primary": true,
                "is_revoked": false
            }
        }))
        .unwrap();
        if let Chat::Group(ref chat) = data.chat {
            assert_eq!(chat.id, 1);
        } else {
            panic!("Unknown chat type: {:?}", data.chat)
        }
        assert_eq!(data.from.id, 1);
        assert_eq!(data.date, 0);
        assert_eq!(data.old_chat_member.get_user().id, 2);
        assert!(data.old_chat_member.is_member());
        assert_eq!(data.new_chat_member.get_user().id, 2);
        assert!(!data.new_chat_member.is_member());
        assert_eq!(
            data.invite_link.unwrap().invite_link,
            "https://t.me/joinchat/o8oIBrbCI3U2OGJi"
        );
    }

    #[test]
    fn deserialize_chat_member_updated_partial() {
        let data: ChatMemberUpdated = serde_json::from_value(serde_json::json!({
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle"
            },
            "from": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            },
            "date": 0,
            "old_chat_member": {
                "status": "member",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "firstname"
                }
            },
            "new_chat_member": {
                "status": "kicked",
                "user": {
                    "id": 2,
                    "is_bot": true,
                    "first_name": "firstname",
                },
                "until_date": 0
            },
        }))
        .unwrap();
        if let Chat::Group(ref chat) = data.chat {
            assert_eq!(chat.id, 1);
        } else {
            panic!("Unknown chat type: {:?}", data.chat)
        }
        assert_eq!(data.from.id, 1);
        assert_eq!(data.date, 0);
        assert_eq!(data.old_chat_member.get_user().id, 2);
        assert!(data.old_chat_member.is_member());
        assert_eq!(data.new_chat_member.get_user().id, 2);
        assert!(!data.new_chat_member.is_member());
        assert!(data.invite_link.is_none());
    }
}

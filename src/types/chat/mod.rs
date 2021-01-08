use crate::types::{message::Message, primitive::Integer};
use serde::{Deserialize, Serialize};
use std::fmt;

mod location;
mod member;
mod permissions;
mod photo;

pub use self::{
    location::ChatLocation,
    member::{ChatMember, ChatMemberAdministrator, ChatMemberKicked, ChatMemberRestricted},
    permissions::ChatPermissions,
    photo::ChatPhoto,
};

/// Chat
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Chat {
    /// Channel
    Channel(ChannelChat),
    /// Group
    Group(GroupChat),
    /// Private chat
    Private(PrivateChat),
    /// Supergroup
    Supergroup(SupergroupChat),
}

/// Channel chat
#[derive(Clone, Debug, Deserialize)]
pub struct ChannelChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a channel
    pub username: Option<String>,
    /// Chat photo
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Description of a channel
    ///
    /// Returned only in getChat
    pub description: Option<String>,
    /// Invite link
    ///
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// Unique identifier for the linked discussion group
    ///
    /// Returned only in getChat
    pub linked_chat_id: Option<Integer>,
}

/// Group chat
#[derive(Clone, Debug, Deserialize)]
pub struct GroupChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Chat photo
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Invite link
    ///
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions, for groups and supergroups
    ///
    /// Returned only in getChat
    pub permissions: Option<ChatPermissions>,
}

/// Private chat
#[derive(Clone, Debug, Deserialize)]
pub struct PrivateChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// First name of the other party
    pub first_name: String,
    /// Last name of the other party
    pub last_name: Option<String>,
    /// Username of a chat
    pub username: Option<String>,
    /// Chat photo
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Bio of the other party
    ///
    /// Returned only in getChat
    pub bio: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
}

/// Supergroup chat
#[derive(Clone, Debug, Deserialize)]
pub struct SupergroupChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a supergroup
    pub username: Option<String>,
    /// Photo of a supergroup
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Description of a supergroup
    ///
    /// Returned only in getChat
    pub description: Option<String>,
    /// Invite link
    ///
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Latest pinned message
    ///
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// For supergroups, name of group sticker set
    ///
    /// Returned only in getChat
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set
    ///
    /// Returned only in getChat
    pub can_set_sticker_set: Option<bool>,
    /// Default chat member permissions, for groups and supergroups
    ///
    /// Returned only in getChat
    pub permissions: Option<ChatPermissions>,
    /// The minimum allowed delay between consecutive messages sent by each unpriviledged user
    ///
    /// Returned only in getChat
    pub slow_mode_delay: Option<Integer>,
    /// Unique identifier for the linked channel
    ///
    /// Returned only in getChat
    pub linked_chat_id: Option<Integer>,
    /// The location to which the supergroup is connected
    ///
    /// Returned only in getChat
    pub location: Option<ChatLocation>,
}

/// Chat ID or username
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum ChatId {
    /// @username of a chat
    Username(String),
    /// ID of a chat
    Id(Integer),
}

impl fmt::Display for ChatId {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChatId::Username(username) => write!(out, "{}", username),
            ChatId::Id(chat_id) => write!(out, "{}", chat_id),
        }
    }
}

impl From<&str> for ChatId {
    fn from(username: &str) -> ChatId {
        ChatId::Username(String::from(username))
    }
}

impl From<String> for ChatId {
    fn from(username: String) -> ChatId {
        ChatId::Username(username)
    }
}

impl From<Integer> for ChatId {
    fn from(id: Integer) -> ChatId {
        ChatId::Id(id)
    }
}

/// Type of action to tell the user that some is happening on the bot's side
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    /// For location data
    FindLocation,
    /// For audio files
    RecordAudio,
    /// For videos
    RecordVideo,
    /// For video notes
    RecordVideoNote,
    /// For text messages
    Typing,
    /// For audio files
    UploadAudio,
    /// For general files
    UploadDocument,
    /// For photos
    UploadPhoto,
    /// For videos
    UploadVideo,
    /// For video notes
    UploadVideoNote,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn deserialize_channel_full() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "channeltitle",
            "username": "channelusername",
            "photo": {
                "small_file_id": "smallfileid",
                "small_file_unique_id": "smallfileuniqueid",
                "big_file_id": "bigfileid",
                "big_file_unique_id": "bigfileuniqueid",
            },
            "description": "channeldescription",
            "invite_link": "channelinvitelink",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "channeltitle"
                },
                "text": "test"
            },
            "linked_chat_id": 2
        }))
        .unwrap();
        if let Chat::Channel(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "channeltitle");
            assert_eq!(chat.username.unwrap(), "channelusername");
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
            assert_eq!(chat.description.unwrap(), "channeldescription");
            assert_eq!(chat.invite_link.unwrap(), "channelinvitelink");
            assert!(chat.pinned_message.is_some());
            assert_eq!(chat.linked_chat_id.unwrap(), 2);
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
    }

    #[test]
    fn deserialize_channel_partial() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "channeltitle"
        }))
        .unwrap();
        if let Chat::Channel(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "channeltitle");
            assert!(chat.username.is_none());
            assert!(chat.photo.is_none());
            assert!(chat.description.is_none());
            assert!(chat.invite_link.is_none());
            assert!(chat.pinned_message.is_none());
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
    }

    #[test]
    fn deserialize_group_full() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "grouptitle",
            "photo": {
                "small_file_id": "smallfileid",
                "small_file_unique_id": "smallfileuniqueid",
                "big_file_id": "bigfileid",
                "big_file_unique_id": "bigfileuniqueid",
            },
            "invite_link": "groupinvitelink",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "grouptitle"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "user"
                },
                "text": "test"
            },
            "permissions": {"can_send_messages": true}
        }))
        .unwrap();
        if let Chat::Group(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "grouptitle");
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
            assert_eq!(chat.invite_link.unwrap(), "groupinvitelink");
            let permissions = chat.permissions.unwrap();
            assert!(permissions.can_send_messages.unwrap());
            assert!(chat.pinned_message.is_some());
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
    }

    #[test]
    fn deserialize_group_partial() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "grouptitle"
        }))
        .unwrap();
        if let Chat::Group(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "grouptitle");
            assert!(chat.photo.is_none());
            assert!(chat.invite_link.is_none());
            assert!(chat.pinned_message.is_none());
            assert!(chat.permissions.is_none());
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
    }

    #[test]
    fn deserialize_private_full() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "private",
            "username": "testusername",
            "first_name": "testfirstname",
            "last_name": "testlastname",
            "photo": {
                "small_file_id": "smallfileid",
                "small_file_unique_id": "smallfileuniqueid",
                "big_file_id": "bigfileid",
                "big_file_unique_id": "bigfileuniqueid",
            },
            "bio": "testbio",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 2,
                    "type": "private",
                    "first_name": "test"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "user"
                },
                "text": "test"
            }
        }))
        .unwrap();
        if let Chat::Private(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.username.unwrap(), "testusername");
            assert_eq!(chat.first_name, "testfirstname");
            assert_eq!(chat.last_name.unwrap(), "testlastname");
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
            assert_eq!(chat.bio.unwrap(), "testbio");
            assert_eq!(chat.pinned_message.unwrap().id, 1);
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }

    #[test]
    fn deserialize_private_partial() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "private",
            "first_name": "testfirstname"
        }))
        .unwrap();
        if let Chat::Private(chat) = chat {
            assert_eq!(chat.id, 1);
            assert!(chat.username.is_none());
            assert_eq!(chat.first_name, "testfirstname");
            assert!(chat.last_name.is_none());
            assert!(chat.photo.is_none());
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }

    #[test]
    fn deserialize_supergroup_full() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "supergrouptitle",
            "username": "supergroupusername",
            "photo": {
                "small_file_id": "smallfileid",
                "small_file_unique_id": "smallfileuniqueid",
                "big_file_id": "bigfileid",
                "big_file_unique_id": "bigfileuniqueid",
            },
            "description": "supergroupdescription",
            "invite_link": "supergroupinvitelink",
            "sticker_set_name": "supergroupstickersetname",
            "can_set_sticker_set": true,
            "slow_mode_delay": 10,
            "permissions": {
                "can_send_messages": true
            },
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "supergroup",
                    "title": "supergrouptitle",
                    "username": "supergroupusername"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "user"
                },
                "text": "test"
            },
            "linked_chat_id": 2,
            "location": {
                "location": {
                    "longitude": 0,
                    "latitude": 1
                },
                "address": "test location"
            }
        }))
        .unwrap();
        if let Chat::Supergroup(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "supergrouptitle");
            assert_eq!(chat.username.unwrap(), "supergroupusername");
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
            assert_eq!(chat.description.unwrap(), "supergroupdescription");
            assert_eq!(chat.invite_link.unwrap(), "supergroupinvitelink");
            assert_eq!(chat.sticker_set_name.unwrap(), "supergroupstickersetname");
            assert_eq!(chat.slow_mode_delay.unwrap(), 10);
            assert!(chat.can_set_sticker_set.unwrap());
            assert!(chat.pinned_message.is_some());
            let permissions = chat.permissions.unwrap();
            assert!(permissions.can_send_messages.unwrap());
            assert_eq!(chat.linked_chat_id.unwrap(), 2);
            assert_eq!(chat.location.unwrap().address, "test location");
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }

    #[test]
    fn deserialize_supergroup_partial() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "supergrouptitle",
            "username": "supergroupusername"
        }))
        .unwrap();
        if let Chat::Supergroup(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "supergrouptitle");
            assert_eq!(chat.username.unwrap(), "supergroupusername");
            assert!(chat.photo.is_none());
            assert!(chat.description.is_none());
            assert!(chat.invite_link.is_none());
            assert!(chat.sticker_set_name.is_none());
            assert!(chat.can_set_sticker_set.is_none());
            assert!(chat.pinned_message.is_none());
            assert!(chat.permissions.is_none());
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }

    #[test]
    fn chat_id() {
        let chat_id = ChatId::from(1);
        if let ChatId::Id(chat_id) = chat_id {
            assert_eq!(chat_id, 1);
        } else {
            panic!("Unexpected chat id: {:?}", chat_id);
        }
        assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#"1"#);
        assert_eq!(chat_id.to_string(), "1");

        let chat_id = ChatId::from("username");
        if let ChatId::Username(ref username) = chat_id {
            assert_eq!(username, "username");
        } else {
            panic!("Unexpected chat id: {:?}", chat_id);
        }
        assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
        assert_eq!(chat_id.to_string(), "username");

        let chat_id = ChatId::from(String::from("username"));
        if let ChatId::Username(ref username) = chat_id {
            assert_eq!(username, "username");
        } else {
            panic!("Unexpected chat id: {:?}", chat_id);
        }
        assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
        assert_eq!(chat_id.to_string(), "username");

        let mut map = HashMap::new();
        let chat_id_1 = ChatId::from(1);
        let chat_id_2 = ChatId::from("username");
        map.insert(chat_id_1.clone(), "1".to_string());
        map.insert(chat_id_2.clone(), "2".to_string());
        assert_eq!(map.get(&chat_id_1).unwrap(), "1");
        assert_eq!(map.get(&chat_id_2).unwrap(), "2");
    }

    #[test]
    fn chat_action() {
        assert_eq!(
            serde_json::to_string(&ChatAction::FindLocation).unwrap(),
            r#""find_location""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::RecordAudio).unwrap(),
            r#""record_audio""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::RecordVideo).unwrap(),
            r#""record_video""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::RecordVideoNote).unwrap(),
            r#""record_video_note""#
        );
        assert_eq!(serde_json::to_string(&ChatAction::Typing).unwrap(), r#""typing""#);
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadAudio).unwrap(),
            r#""upload_audio""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadDocument).unwrap(),
            r#""upload_document""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadPhoto).unwrap(),
            r#""upload_photo""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadVideo).unwrap(),
            r#""upload_video""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadVideoNote).unwrap(),
            r#""upload_video_note""#
        );
    }
}

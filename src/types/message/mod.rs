use crate::types::{chat::Chat, primitive::Integer, reply_markup::InlineKeyboardMarkup, text::Text, user::User};
use serde::Deserialize;

mod data;
mod edit_result;
mod forward;
mod sender;

pub use self::{data::*, edit_result::*, forward::*, sender::*};

/// This object represents a message
#[derive(Clone, Debug, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: Integer,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<Integer>,
    /// Sender of the message
    #[serde(flatten)]
    pub sender: MessageSender,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Author signature
    pub author_signature: Option<String>,
    /// True, if the message can't be forwarded
    #[serde(default)]
    pub has_protected_content: bool,
    /// Forwarded data
    #[serde(flatten)]
    pub forward: Option<Forward>,
    /// True, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(default)]
    pub is_automatic_forward: bool,
    /// For replies, the original message
    /// Note that the Message object in this field will not contain further
    /// reply_to fields even if it itself is a reply
    #[serde(rename = "reply_to_message")]
    pub reply_to: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Contains message data
    #[serde(flatten)]
    pub data: MessageData,
}

impl Message {
    /// Returns true if message has edited and false otherwise
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns text of the message (includes caption)
    pub fn get_text(&self) -> Option<&Text> {
        match self.data {
            MessageData::Text(ref text)
            | MessageData::Audio {
                caption: Some(ref text),
                ..
            }
            | MessageData::Document {
                caption: Some(ref text),
                ..
            }
            | MessageData::Photo {
                caption: Some(ref text),
                ..
            }
            | MessageData::Video {
                caption: Some(ref text),
                ..
            }
            | MessageData::Voice {
                caption: Some(ref text),
                ..
            } => Some(text),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reply_to() {
        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "reply_to_message": {
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                "text": "test"
            }
        }))
        .unwrap();
        if let Some(msg) = msg.reply_to {
            assert_eq!(msg.id, 1);
        } else {
            panic!("Unexpected reply_to data: {:?}", msg.reply_to);
        }
    }

    #[test]
    fn reply_to_with_empty_data() {
        let data: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "reply_to_message": {
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            }
        }))
        .unwrap();
        assert!(data.reply_to.is_some());
    }

    #[test]
    fn via_bot() {
        let data: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "via_bot": {
                "id": 3, "is_bot": true,
                "first_name": "example",
                "last_name": "bot",
                "username": "examplebot"
            }
        }))
        .unwrap();
        let bot = data.via_bot.expect("via_bot is empty");
        assert_eq!(bot.id, 3);
        assert_eq!(bot.first_name, "example");
        assert_eq!(bot.last_name.unwrap(), "bot");
        assert_eq!(bot.username.unwrap(), "examplebot");
    }

    #[test]
    fn is_edited() {
        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "edit_date": 1
        }))
        .unwrap();
        assert!(msg.is_edited());
    }

    #[test]
    fn get_chat_and_user_data() {
        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "has_protected_content": true,
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.chat.get_id(), 1);
        assert!(msg.chat.get_username().is_none());
        assert!(msg.sender.get_user().is_some());
        assert_eq!(msg.sender.get_user_id(), Some(1));
        assert!(msg.sender.get_user_username().is_none());
        assert!(msg.has_protected_content);

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 2, "type": "supergroup", "title": "supergrouptitle", "username": "supergroupusername"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.chat.get_id(), 2);
        assert_eq!(msg.chat.get_username().unwrap(), "supergroupusername");
        assert!(msg.sender.get_user().is_some());
        assert_eq!(msg.sender.get_user_id(), Some(1));
        assert!(msg.sender.get_user_username().is_none());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 3, "type": "private", "first_name": "firstname"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.chat.get_id(), 3);
        assert!(msg.chat.get_username().is_none());
        assert!(msg.sender.get_user().is_some());
        assert_eq!(msg.sender.get_user_id(), Some(1));
        assert!(msg.sender.get_user_username().is_none());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 4, "type": "private", "first_name": "firstname", "username": "username"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.chat.get_id(), 4);
        assert_eq!(msg.chat.get_username().unwrap(), "username");
        assert!(msg.sender.get_user().is_some());
        assert_eq!(msg.sender.get_user_id(), Some(1));
        assert!(msg.sender.get_user_username().is_none());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false, "username": "test"},
            "chat": {"id": 5, "type": "group", "title": "grouptitle"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.chat.get_id(), 5);
        assert!(msg.chat.get_username().is_none());
        assert!(msg.sender.get_user().is_some());
        assert_eq!(msg.sender.get_user_id(), Some(1));
        assert_eq!(msg.sender.get_user_username(), Some("test"));

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1111,
                "date": 0,
                "author_signature": "test",
                "chat": {
                    "id": 6,
                    "type": "channel",
                    "title": "channeltitle",
                    "username": "channelusername"
                },
                "sender_chat": {
                    "id": 6,
                    "type": "channel",
                    "title": "channeltitle",
                    "username": "channelusername"
                },
                "is_automatic_forward": true,
                "text": "test message from channel"
        }))
        .unwrap();
        assert_eq!(msg.chat.get_id(), 6);
        assert_eq!(msg.chat.get_username().unwrap(), "channelusername");
        assert!(msg.sender.get_user().is_none());
        assert!(msg.sender.get_user_id().is_none());
        assert!(msg.sender.get_user_username().is_none());
        assert!(msg.is_automatic_forward);
        if let Chat::Channel(chat) = msg.sender.get_chat().as_ref().unwrap() {
            assert_eq!(chat.id, 6);
        } else {
            panic!("Unexpected sender: {:?}", msg.sender);
        }

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1111,
                "date": 0,
                "author_signature": "test",
                "chat": {
                    "id": 7,
                    "type": "channel",
                    "title": "channeltitle"
                },
                "text": "test message from channel"
        }))
        .unwrap();
        assert_eq!(msg.chat.get_id(), 7);
        assert!(msg.chat.get_username().is_none());
        assert!(msg.sender.get_user().is_none());
        assert!(msg.sender.get_user_id().is_none());
        assert!(msg.sender.get_user_username().is_none());
    }

    #[test]
    fn get_text() {
        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test audio caption",
            "audio": {
                "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
                "file_unique_id": "unique-id",
                "duration": 243
            }
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test audio caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test document caption",
            "document": {
                "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA",
                "file_unique_id": "unique-id",
            }
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test document caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test photo caption",
            "photo": [{
                "file_id": "photo-id",
                "file_unique_id": "unique-id",
                "width": 200,
                "height": 200
            }]
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test photo caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "text"
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "text");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test video caption",
            "video": {
                "file_id": "video-id",
                "file_unique_id": "unique-id",
                "width": 1,
                "height": 2,
                "duration": 3
            }
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test video caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test voice caption",
            "voice": {
                "file_id": "voice-id",
                "file_unique_id": "unique-id",
                "duration": 123
            }
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test voice caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "group_chat_created": true
        }))
        .unwrap();
        assert!(msg.get_text().is_none());
    }

    #[test]
    fn edit_message_result() {
        let data: EditMessageResult = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "text"
        }))
        .unwrap();
        if let EditMessageResult::Message(msg) = data {
            assert_eq!(msg.id, 1);
        } else {
            panic!("Unexpected message result: {:?}", data);
        }

        let data: EditMessageResult = serde_json::from_value(serde_json::json!(false)).unwrap();
        if let EditMessageResult::Bool(flag) = data {
            assert!(!flag);
        } else {
            panic!("Unexpected message result: {:?}", data);
        }
    }
}

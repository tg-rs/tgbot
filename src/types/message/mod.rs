use crate::types::{
    chat::Chat, message::raw::RawMessage, primitive::Integer, reply_markup::InlineKeyboardMarkup, text::Text,
    user::User,
};
use serde::{de::Error, Deserialize, Deserializer};

mod data;
mod forward;
mod kind;
mod raw;

pub use self::{data::*, forward::*, kind::*};

/// This object represents a message
#[derive(Clone, Debug)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub id: Integer,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Contains chat-specific data
    pub kind: MessageKind,
    /// Forwarded data
    pub forward: Option<Forward>,
    /// For replies, the original message
    /// Note that the Message object in this field will not contain further
    /// reply_to fields even if it itself is a reply
    pub reply_to: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<Integer>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Contains message data
    pub data: MessageData,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Sender of the message, sent on behalf of a chat
    ///
    /// The channel itself for channel messages
    /// The supergroup itself for messages from anonymous group administrators
    /// The linked channel for messages automatically forwarded to the discussion group
    pub sender_chat: Option<Chat>,
}

impl Message {
    /// Returns true if message has edited and false otherwise
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns ID of the chat
    pub fn get_chat_id(&self) -> Integer {
        match self.kind {
            MessageKind::Private { ref chat, .. } => chat.id,
            MessageKind::Channel { ref chat, .. } => chat.id,
            MessageKind::Group { ref chat, .. } => chat.id,
            MessageKind::Supergroup { ref chat, .. } => chat.id,
        }
    }

    /// Returns username of the chat
    pub fn get_chat_username(&self) -> Option<&str> {
        if let Some(ref username) = match self.kind {
            MessageKind::Private { ref chat, .. } => &chat.username,
            MessageKind::Channel { ref chat, .. } => &chat.username,
            MessageKind::Supergroup { ref chat, .. } => &chat.username,
            _ => &None,
        } {
            Some(username.as_str())
        } else {
            None
        }
    }

    /// Returns author of the message
    pub fn get_user(&self) -> Option<&User> {
        match self.kind {
            MessageKind::Channel { .. } => None,
            MessageKind::Private { ref from, .. }
            | MessageKind::Group { ref from, .. }
            | MessageKind::Supergroup { ref from, .. } => Some(from),
        }
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

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawMessage::deserialize(deserializer)?;

        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        }

        let message_kind = match raw.chat {
            Chat::Channel(chat) => MessageKind::Channel {
                chat,
                author_signature: raw.author_signature,
            },
            Chat::Group(chat) => MessageKind::Group {
                chat,
                from: required!(from),
                author_signature: raw.author_signature,
            },
            Chat::Private(chat) => MessageKind::Private {
                chat,
                from: required!(from),
            },
            Chat::Supergroup(chat) => MessageKind::Supergroup {
                chat,
                from: required!(from),
                author_signature: raw.author_signature,
            },
        };

        Ok(Message {
            id: raw.message_id,
            date: raw.date,
            kind: message_kind,
            forward: raw.forward,
            reply_to: raw.reply_to_message.map(Box::new),
            via_bot: raw.via_bot,
            edit_date: raw.edit_date,
            media_group_id: raw.media_group_id,
            data: raw.data,
            reply_markup: raw.reply_markup,
            sender_chat: raw.sender_chat,
        })
    }
}

/// Result of editMessage* requests
#[derive(Clone, Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot
    Message(Message),
    /// Returned if edited message is NOT sent by the bot
    Bool(bool),
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
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 1);
        assert!(msg.get_chat_username().is_none());
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 2, "type": "supergroup", "title": "supergrouptitle", "username": "supergroupusername"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 2);
        assert_eq!(msg.get_chat_username().unwrap(), "supergroupusername");
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 3, "type": "private", "first_name": "firstname"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 3);
        assert!(msg.get_chat_username().is_none());
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 4, "type": "private", "first_name": "firstname", "username": "username"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 4);
        assert_eq!(msg.get_chat_username().unwrap(), "username");
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 5, "type": "group", "title": "grouptitle"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 5);
        assert!(msg.get_chat_username().is_none());
        assert!(msg.get_user().is_some());

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
                "text": "test message from channel"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 6);
        assert_eq!(msg.get_chat_username().unwrap(), "channelusername");
        assert!(msg.get_user().is_none());
        if let Chat::Channel(chat) = msg.sender_chat.as_ref().unwrap() {
            assert_eq!(chat.id, 6);
        } else {
            panic!("Unexpected sender chat: {:?}", msg.sender_chat);
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
        assert_eq!(msg.get_chat_id(), 7);
        assert!(msg.get_chat_username().is_none());
        assert!(msg.get_user().is_none());
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

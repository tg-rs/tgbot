use crate::types::{chat::Chat, primitive::Integer, user::User};
use serde::Deserialize;

/// Sender of the message
#[derive(Clone, Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged, from = "RawMessageSender")]
pub enum MessageSender {
    /// For messages sent by chat
    ///
    /// For example, the channel itself for channel posts, the supergroup itself for messages
    /// from anonymous group administrators, the linked channel
    /// for messages automatically forwarded to the discussion group.
    Chat(Chat),
    /// For messages sent by user
    User(User),
    /// For messages without sender chat and user
    Unknown,
}

impl MessageSender {
    /// Returns a sender user
    pub fn get_user(&self) -> Option<&User> {
        match self {
            MessageSender::User(ref user) => Some(user),
            _ => None,
        }
    }

    /// Returns ID of sender user
    pub fn get_user_id(&self) -> Option<Integer> {
        self.get_user().map(|user| user.id)
    }

    /// Returns username of sender user
    pub fn get_user_username(&self) -> Option<&str> {
        self.get_user()
            .and_then(|user| user.username.as_ref())
            .map(String::as_str)
    }

    /// Returns a sender chat
    pub fn get_chat(&self) -> Option<&Chat> {
        match self {
            MessageSender::Chat(ref chat) => Some(chat),
            _ => None,
        }
    }

    /// Returns ID of sender chat
    pub fn get_chat_id(&self) -> Option<Integer> {
        self.get_chat().map(|chat| chat.get_id())
    }

    /// Returns username of sender chat
    pub fn get_chat_username(&self) -> Option<&str> {
        self.get_chat().and_then(|chat| chat.get_username())
    }
}

#[derive(Clone, Debug, Deserialize)]
struct RawMessageSender {
    sender_chat: Option<Chat>,
    from: Option<User>,
}

impl From<RawMessageSender> for MessageSender {
    fn from(raw: RawMessageSender) -> Self {
        match (raw.sender_chat, raw.from) {
            (Some(chat), None) => MessageSender::Chat(chat),
            (None, Some(user)) => MessageSender::User(user),
            _ => MessageSender::Unknown,
        }
    }
}
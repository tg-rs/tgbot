use serde::{Deserialize, Serialize};

use crate::types::{Chat, ChatPeerId, ChatUsername, User, UserPeerId, UserUsername};

#[cfg(test)]
mod tests;

/// Represents a sender of a message.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged, from = "RawMessageSender", into = "RawMessageSender")]
pub enum MessageSender {
    /// For messages sent by a chat.
    ///
    /// For example, the channel itself for channel posts, the supergroup itself for messages
    /// from anonymous group administrators, the linked channel
    /// for messages automatically forwarded to the discussion group.
    Chat(Chat),
    /// For messages sent by a user.
    User(User),
    /// For messages without a sender chat and a user.
    Unknown,
}

impl From<Chat> for MessageSender {
    fn from(value: Chat) -> Self {
        Self::Chat(value)
    }
}

impl From<User> for MessageSender {
    fn from(value: User) -> Self {
        Self::User(value)
    }
}

impl MessageSender {
    /// Returns the sender user.
    pub fn get_user(&self) -> Option<&User> {
        match self {
            MessageSender::User(ref user) => Some(user),
            _ => None,
        }
    }

    /// Returns the ID of the sender user.
    pub fn get_user_id(&self) -> Option<UserPeerId> {
        self.get_user().map(|user| user.id)
    }

    /// Returns the username of the sender user.
    pub fn get_user_username(&self) -> Option<&UserUsername> {
        self.get_user().and_then(|user| user.username.as_ref())
    }

    /// Returns the sender chat.
    pub fn get_chat(&self) -> Option<&Chat> {
        match self {
            MessageSender::Chat(ref chat) => Some(chat),
            _ => None,
        }
    }

    /// Returns the ID of the sender chat.
    pub fn get_chat_id(&self) -> Option<ChatPeerId> {
        self.get_chat().map(|chat| chat.get_id())
    }

    /// Returns the username of the sender chat.
    pub fn get_chat_username(&self) -> Option<&ChatUsername> {
        self.get_chat().and_then(|chat| chat.get_username())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RawMessageSender {
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl From<MessageSender> for RawMessageSender {
    fn from(value: MessageSender) -> Self {
        match value {
            MessageSender::Chat(chat) => Self {
                sender_chat: Some(chat),
                from: None,
            },
            MessageSender::User(user) => Self {
                sender_chat: None,
                from: Some(user),
            },
            MessageSender::Unknown => Self {
                sender_chat: None,
                from: None,
            },
        }
    }
}

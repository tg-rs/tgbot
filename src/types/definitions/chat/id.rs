use std::fmt;

use serde::{Deserialize, Serialize};

use crate::types::Integer;

/// ID of a chat.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(from = "Integer", into = "Integer")]
pub struct ChatPeerId(Integer);

impl From<Integer> for ChatPeerId {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl From<ChatPeerId> for Integer {
    fn from(value: ChatPeerId) -> Self {
        value.0
    }
}

impl fmt::Display for ChatPeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<Integer> for ChatPeerId {
    fn eq(&self, other: &Integer) -> bool {
        self.0.eq(other)
    }
}

/// Username of a chat in the format `@username`.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(from = "String", into = "String")]
pub struct ChatUsername(String);

impl From<&str> for ChatUsername {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for ChatUsername {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ChatUsername> for String {
    fn from(value: ChatUsername) -> Self {
        value.0
    }
}

impl fmt::Display for ChatUsername {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<String> for ChatUsername {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<str> for ChatUsername {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

/// Represents an ID or username of a chat.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum ChatId {
    /// ID of a chat.
    Id(ChatPeerId),
    /// Username of a chat in the format `@username`.
    Username(ChatUsername),
}

impl fmt::Display for ChatId {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChatId::Id(chat_id) => write!(out, "{}", chat_id.0),
            ChatId::Username(username) => write!(out, "{}", username.0),
        }
    }
}

impl From<ChatPeerId> for ChatId {
    fn from(value: ChatPeerId) -> Self {
        ChatId::Id(value)
    }
}

impl From<ChatUsername> for ChatId {
    fn from(value: ChatUsername) -> Self {
        ChatId::Username(value)
    }
}

impl From<&str> for ChatId {
    fn from(username: &str) -> ChatId {
        ChatId::Username(String::from(username).into())
    }
}

impl From<String> for ChatId {
    fn from(username: String) -> ChatId {
        ChatId::Username(username.into())
    }
}

impl From<Integer> for ChatId {
    fn from(id: Integer) -> ChatId {
        ChatId::Id(id.into())
    }
}

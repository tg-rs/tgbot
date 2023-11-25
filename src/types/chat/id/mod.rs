use std::fmt;

use serde::{Deserialize, Serialize};

use crate::types::Integer;

#[cfg(test)]
mod tests;

/// Represents an ID or username of a chat.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum ChatId {
    /// ID of a chat.
    Id(Integer),
    /// Username of a chat in the format `@username`.
    Username(String),
}

impl fmt::Display for ChatId {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChatId::Id(chat_id) => write!(out, "{}", chat_id),
            ChatId::Username(username) => write!(out, "{}", username),
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

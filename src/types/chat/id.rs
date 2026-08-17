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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn chat_peer_id() {
        let a = ChatPeerId::from(1);
        let b = ChatPeerId::from(1);
        assert_eq!(a, b);
        assert_eq!(Integer::from(a), 1);
        assert_eq!(b.to_string(), "1");
    }

    #[test]
    fn chat_username() {
        let a = ChatUsername::from("@test");
        let b = ChatUsername::from(String::from("@test"));
        assert_eq!(a, b);
        assert_eq!(a, *"@test");
        assert_eq!(a, String::from("@test"));
        assert_eq!(a.to_string(), "@test");
    }

    #[test]
    fn chat_id() {
        let chat_id = ChatId::from(1);
        if let ChatId::Id(chat_id) = chat_id {
            assert_eq!(chat_id, 1);
        } else {
            panic!("Unexpected chat id: {chat_id:?}");
        }
        assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#"1"#);
        assert_eq!(chat_id.to_string(), "1");

        let chat_id = ChatId::from("username");
        if let ChatId::Username(ref username) = chat_id {
            assert_eq!(username, "username");
        } else {
            panic!("Unexpected chat id: {chat_id:?}");
        }
        assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
        assert_eq!(chat_id.to_string(), "username");

        let chat_id = ChatId::from(String::from("username"));
        if let ChatId::Username(ref username) = chat_id {
            assert_eq!(username, "username");
        } else {
            panic!("Unexpected chat id: {chat_id:?}");
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

        let chat_id = ChatId::from(ChatPeerId::from(1));
        assert!(matches!(chat_id, ChatId::Id(_)));

        let chat_id = ChatId::from(ChatUsername::from("@test"));
        assert!(matches!(chat_id, ChatId::Username(_)));
    }
}

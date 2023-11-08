use serde::Serialize;

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer},
};

#[cfg(test)]
mod tests;

/// Ban a channel chat in a supergroup or a channel
///
/// Until the chat is unbanned, the owner of the banned chat won't be able to send messages
/// on behalf of any of their channels. The bot must be an administrator in the supergroup or
/// channel for this to work and must have the appropriate administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct BanChatSenderChat {
    chat_id: ChatId,
    sender_chat_id: Integer,
}

impl BanChatSenderChat {
    /// Creates a new BanChatSenderChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier of the target chat
    /// * sender_chat_id - Unique identifier of the target sender chat
    pub fn new<T>(chat_id: T, sender_chat_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            sender_chat_id,
        }
    }
}

impl Method for BanChatSenderChat {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("banChatSenderChat", self)
    }
}

/// Unban a previously banned channel chat in a supergroup or channel
///
/// The bot must be an administrator for this to work and must have
/// the appropriate administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct UnbanChatSenderChat {
    chat_id: ChatId,
    sender_chat_id: Integer,
}

impl UnbanChatSenderChat {
    /// Creates a new UnbanChatSenderChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier of the target chat
    /// * sender_chat_id - Unique identifier of the target sender chat
    pub fn new<T>(chat_id: T, sender_chat_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            sender_chat_id,
        }
    }
}

impl Method for UnbanChatSenderChat {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("unbanChatSenderChat", self)
    }
}

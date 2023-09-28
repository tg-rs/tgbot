use serde::Serialize;

use crate::{
    method::Method,
    request::Request,
    types::{ChatId, Integer},
};

#[cfg(test)]
mod tests;

/// Add a message to the list of pinned messages in a chat
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the 'can_pin_messages'
/// admin right in a supergroup or 'can_edit_messages' admin right in a channel.
#[derive(Clone, Debug, Serialize)]
pub struct PinChatMessage {
    chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl PinChatMessage {
    /// Creates a new PinChatMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of a message to pin
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        PinChatMessage {
            chat_id: chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message
    ///
    /// Notifications are always disabled in channels
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
}

impl Method for PinChatMessage {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("pinChatMessage", self)
    }
}

/// Remove a message from the list of pinned messages in a chat
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the 'can_pin_messages'
/// admin right in a supergroup or 'can_edit_messages' admin right in a channel
#[derive(Clone, Debug, Serialize)]
pub struct UnpinChatMessage {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
}

impl UnpinChatMessage {
    /// Creates a new UnpinChatMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        UnpinChatMessage {
            chat_id: chat_id.into(),
            message_id: None,
        }
    }

    /// Identifier of a message to unpin
    ///
    /// If not specified, the most recent pinned message (by sending date) will be unpinned
    pub fn message_id(mut self, message_id: Integer) -> Self {
        self.message_id = Some(message_id);
        self
    }
}

impl Method for UnpinChatMessage {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("unpinChatMessage", self)
    }
}

/// Clear the list of pinned messages in a chat
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the 'can_pin_messages'
/// admin right in a supergroup or 'can_edit_messages' admin right in a channel
#[derive(Clone, Debug, Serialize)]
pub struct UnpinAllChatMessages {
    chat_id: ChatId,
}

impl UnpinAllChatMessages {
    /// Creates a new UnpinAllChatMessages
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        UnpinAllChatMessages {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for UnpinAllChatMessages {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("unpinAllChatMessages", self)
    }
}

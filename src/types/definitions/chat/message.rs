use serde::Serialize;

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer},
};

/// Adds a message to a list of pinned messages in a chat.
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the `can_pin_messages`
/// admin right in a supergroup or `can_edit_messages` admin right in a channel.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct PinChatMessage {
    chat_id: ChatId,
    message_id: Integer,
    business_connection_id: Option<String>,
    disable_notification: Option<bool>,
}

impl PinChatMessage {
    /// Creates a new `PinChatMessage`
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of a message to pin.
    pub fn new<T>(chat_id: T, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        PinChatMessage {
            chat_id: chat_id.into(),
            message_id,
            business_connection_id: None,
            disable_notification: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection
    ///   on behalf of which the message will be pinned.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to notify all chat members about the new pinned message;
    ///   notifications are always disabled in channels and private chats.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }
}

impl Method for PinChatMessage {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("pinChatMessage", self)
    }
}

/// Removes a message from a list of pinned messages in a chat.
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the `can_pin_messages`
/// admin right in a supergroup or `can_edit_messages` admin right in a channel.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct UnpinChatMessage {
    chat_id: ChatId,
    business_connection_id: Option<String>,
    message_id: Option<Integer>,
}

impl UnpinChatMessage {
    /// Creates a new `UnpinChatMessage`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        UnpinChatMessage {
            chat_id: chat_id.into(),
            business_connection_id: None,
            message_id: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection
    ///   on behalf of which the message will be unpinned.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new message ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of a message to unpin.
    ///
    /// If not specified, the most recent pinned message (by sending date) will be unpinned.
    pub fn with_message_id(mut self, value: Integer) -> Self {
        self.message_id = Some(value);
        self
    }
}

impl Method for UnpinChatMessage {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("unpinChatMessage", self)
    }
}

/// Clears a list of pinned messages in a chat.
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the `can_pin_messages`
/// admin right in a supergroup or `can_edit_messages` admin right in a channel.
#[derive(Clone, Debug, Serialize)]
pub struct UnpinAllChatMessages {
    chat_id: ChatId,
}

impl UnpinAllChatMessages {
    /// Creates a new `UnpinAllChatMessages`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        UnpinAllChatMessages {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for UnpinAllChatMessages {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("unpinAllChatMessages", self)
    }
}

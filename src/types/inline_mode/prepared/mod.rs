use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{InlineQueryResult, Integer},
};

#[cfg(test)]
mod tests;

/// Describes an inline message to be sent by a user of a Mini App.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message.
    pub id: String,
    /// Expiration date of the prepared message, in Unix time.
    /// Expired prepared messages can no longer be used.
    pub expiration_date: Integer,
}

impl PreparedInlineMessage {
    /// Creates a new `PreparedInlineMessage`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the prepared message.
    /// * `expiration_date` - Expiration date of the prepared message, in Unix time.
    pub fn new<T>(id: T, expiration_date: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            id: id.into(),
            expiration_date,
        }
    }
}

/// Stores a message that can be sent by a user of a Mini App.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SavePreparedInlineMessage {
    result: InlineQueryResult,
    user_id: Integer,
    allow_bot_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_user_chats: Option<bool>,
}

impl SavePreparedInlineMessage {
    /// Creates a new `SavePreparedInlineMessage`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user that can use the prepared message.
    /// * `result` - An object describing the message to be sent
    pub fn new<T>(user_id: Integer, result: T) -> Self
    where
        T: Into<InlineQueryResult>,
    {
        Self {
            user_id,
            result: result.into(),
            allow_bot_chats: None,
            allow_channel_chats: None,
            allow_group_chats: None,
            allow_user_chats: None,
        }
    }

    /// Sets a new value for the `allow_bot_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to private chats with bots.
    pub fn with_allow_bot_chats(mut self, value: bool) -> Self {
        self.allow_bot_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_channel_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to channel chats.
    pub fn with_allow_channel_chats(mut self, value: bool) -> Self {
        self.allow_channel_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_group_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to group and supergroup chats.
    pub fn with_allow_group_chats(mut self, value: bool) -> Self {
        self.allow_group_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_user_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to private chats with users.
    pub fn with_allow_user_chats(mut self, value: bool) -> Self {
        self.allow_user_chats = Some(value);
        self
    }
}

impl Method for SavePreparedInlineMessage {
    type Response = PreparedInlineMessage;

    fn into_payload(self) -> Payload {
        Payload::json("savePreparedInlineMessage", self)
    }
}

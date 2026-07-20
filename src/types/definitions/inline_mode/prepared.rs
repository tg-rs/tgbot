use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError},
    types::{InlineQueryResult, Integer, SerializeError},
};

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
#[derive(Debug)]
pub struct SavePreparedInlineMessage {
    form: Form,
}

impl SavePreparedInlineMessage {
    /// Creates a new `SavePreparedInlineMessage`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user that can use the prepared message.
    /// * `result` - An object describing the message to be sent
    pub fn new<T>(user_id: Integer, result: T) -> Result<Self, SerializeError>
    where
        T: Into<InlineQueryResult>,
    {
        let (form, data) = result.into().into_parts(&[0]);
        let mut form = form.unwrap_or_default();
        form.insert_field("user_id", user_id);
        form.insert_field("result", data.serialize()?);
        Ok(Self { form })
    }

    /// Sets a new value for the `allow_bot_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to private chats with bots.
    pub fn with_allow_bot_chats(mut self, value: bool) -> Self {
        self.form.insert_field("allow_bot_chats", value);
        self
    }

    /// Sets a new value for the `allow_channel_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to channel chats.
    pub fn with_allow_channel_chats(mut self, value: bool) -> Self {
        self.form.insert_field("allow_channel_chats", value);
        self
    }

    /// Sets a new value for the `allow_group_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to group and supergroup chats.
    pub fn with_allow_group_chats(mut self, value: bool) -> Self {
        self.form.insert_field("allow_group_chats", value);
        self
    }

    /// Sets a new value for the `allow_user_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to private chats with users.
    pub fn with_allow_user_chats(mut self, value: bool) -> Self {
        self.form.insert_field("allow_user_chats", value);
        self
    }
}

impl Method for SavePreparedInlineMessage {
    type Response = PreparedInlineMessage;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::form("savePreparedInlineMessage", self.form)
    }
}

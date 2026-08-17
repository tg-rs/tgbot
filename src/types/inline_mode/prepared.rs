use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{InlineQueryResult, InlineQueryResultData, Integer},
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

/// Stores a message that can be sent by a user of a Mini App.
#[derive(Debug)]
pub struct SavePreparedInlineMessage {
    user_id: Integer,
    result: InlineQueryResult,
    parameters: SavePreparedInlineMessageParameters,
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
            parameters: Default::default(),
        }
    }

    /// Sets a new value for the `allow_bot_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to private chats with bots.
    pub fn with_allow_bot_chats(mut self, value: bool) -> Self {
        self.parameters.allow_bot_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_channel_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to channel chats.
    pub fn with_allow_channel_chats(mut self, value: bool) -> Self {
        self.parameters.allow_channel_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_group_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to group and supergroup chats.
    pub fn with_allow_group_chats(mut self, value: bool) -> Self {
        self.parameters.allow_group_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_user_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message can be sent to private chats with users.
    pub fn with_allow_user_chats(mut self, value: bool) -> Self {
        self.parameters.allow_user_chats = Some(value);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
struct SavePreparedInlineMessageParameters {
    allow_bot_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_user_chats: Option<bool>,
    result: Option<InlineQueryResultData>,
    user_id: Option<Integer>,
}

impl Method for SavePreparedInlineMessage {
    type Response = PreparedInlineMessage;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self {
            user_id,
            result,
            mut parameters,
        } = self;
        let mut form = Form::default();
        parameters.user_id = Some(user_id);
        parameters.result = Some(result.write(&mut form));
        parameters.serialize(&mut form)?;
        Payload::form("savePreparedInlineMessage", form)
    }
}

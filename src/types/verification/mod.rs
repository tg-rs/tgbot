use serde::Serialize;

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer},
};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Serialize)]
/// Removes verification from a chat that is currently verified on behalf of the organization represented by the bot.
pub struct RemoveChatVerification {
    chat_id: ChatId,
}

impl RemoveChatVerification {
    /// Creates a new `RemoveChatVerification`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat
    ///   or username of the target channel (in the format @channelusername).
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for RemoveChatVerification {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("removeChatVerification", self)
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
/// Removes verification from a user who is currently verified on behalf of the organization represented by the bot.
pub struct RemoveUserVerification {
    user_id: Integer,
}

impl RemoveUserVerification {
    /// Creates a new `RemoveUserVerification`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user.
    pub fn new(user_id: Integer) -> Self {
        Self { user_id }
    }
}

impl Method for RemoveUserVerification {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("removeUserVerification", self)
    }
}

/// Verifies a chat on behalf of the organization which is represented by the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct VerifyChat {
    chat_id: ChatId,
    custom_description: Option<String>,
}

impl VerifyChat {
    /// Creates a new `VerifyChat`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat
    ///   or username of the target channel (in the format @channelusername).
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            custom_description: None,
        }
    }

    /// Sets a new custom description.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom description for the verification; 0-70 characters.
    ///   Must be empty if the organization isn't allowed to provide a custom verification description.
    pub fn with_custom_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_description = Some(value.into());
        self
    }
}

impl Method for VerifyChat {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("verifyChat", self)
    }
}

/// Verifies a user on behalf of the organization which is represented by the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct VerifyUser {
    user_id: Integer,
    custom_description: Option<String>,
}

impl VerifyUser {
    /// Creates a new `VerifyUser`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user.
    pub fn new(user_id: Integer) -> Self {
        Self {
            user_id,
            custom_description: None,
        }
    }

    /// Sets a new custom description.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom description for the verification; 0-70 characters.
    ///   Must be empty if the organization isn't allowed to provide a custom verification description.
    pub fn with_custom_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_description = Some(value.into());
        self
    }
}

impl Method for VerifyUser {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("verifyUser", self)
    }
}

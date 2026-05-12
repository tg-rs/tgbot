use serde::{Deserialize, Serialize};

use crate::types::{Chat, User};

/// Information for a message sent by a guest bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageGuestBot {
    /// The chat whose original message triggered the bot's response.
    #[serde(rename = "guest_bot_caller_chat")]
    caller_chat: Option<Chat>,
    /// The user whose original message triggered the bot's response.
    #[serde(rename = "guest_bot_caller_user")]
    caller_user: Option<User>,
}

impl MessageGuestBot {
    /// Sets a new caller chat.
    ///
    /// # Arguments
    ///
    /// * `value` - The chat whose original message triggered the bot's response.
    pub fn with_caller_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.caller_chat = Some(value.into());
        self
    }

    /// Sets a new caller user.
    ///
    /// # Arguments
    ///
    /// * `value` - The user whose original message triggered the bot's response.
    pub fn with_caller_user(mut self, value: User) -> Self {
        self.caller_user = Some(value);
        self
    }
}

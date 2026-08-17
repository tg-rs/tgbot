use serde::{Deserialize, Serialize};

use crate::types::{Chat, User};

/// Information for a message sent by a guest bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageGuestBot {
    /// The chat whose original message triggered the bot's response.
    #[serde(rename = "guest_bot_caller_chat")]
    pub caller_chat: Option<Chat>,
    /// The user whose original message triggered the bot's response.
    #[serde(rename = "guest_bot_caller_user")]
    pub caller_user: Option<User>,
}

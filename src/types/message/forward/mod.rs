use serde::Deserialize;

use crate::types::{ChannelChat, Integer, User};

#[cfg(test)]
mod tests;

/// Contains information about original message
#[derive(Clone, Debug, Deserialize)]
pub struct Forward {
    /// Sender of the original message
    #[serde(flatten)]
    pub from: ForwardFrom,
    /// Date the original message was sent in Unix time
    #[serde(rename = "forward_date")]
    pub date: Integer,
}

/// Sender of the original message
#[derive(Clone, Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(from = "RawForwardFrom")]
pub enum ForwardFrom {
    /// Information about user
    User(User),
    /// Name of user who has hidden link to account
    HiddenUser(String),
    /// Information about channel
    Channel {
        /// Information about the original chat
        chat: ChannelChat,
        /// Identifier of the original message in the channel
        message_id: Integer,
        /// Signature of the post author if present
        signature: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum RawForwardFrom {
    User {
        forward_from: User,
    },
    HiddenUser {
        forward_sender_name: String,
    },
    Channel {
        forward_from_chat: ChannelChat,
        forward_from_message_id: Integer,
        forward_signature: Option<String>,
    },
}

impl From<RawForwardFrom> for ForwardFrom {
    fn from(raw: RawForwardFrom) -> Self {
        match raw {
            RawForwardFrom::User { forward_from } => ForwardFrom::User(forward_from),
            RawForwardFrom::HiddenUser { forward_sender_name } => ForwardFrom::HiddenUser(forward_sender_name),
            RawForwardFrom::Channel {
                forward_from_chat,
                forward_from_message_id,
                forward_signature,
            } => ForwardFrom::Channel {
                chat: forward_from_chat,
                message_id: forward_from_message_id,
                signature: forward_signature,
            },
        }
    }
}

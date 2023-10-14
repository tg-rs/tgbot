use serde::{Deserialize, Serialize};

use crate::types::{ChannelChat, Integer, User};

#[cfg(test)]
mod tests;

/// Contains information about original message
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Forward {
    /// Sender of the original message
    #[serde(flatten)]
    pub from: ForwardFrom,
    /// Date the original message was sent in Unix time
    #[serde(rename = "forward_date")]
    pub date: Integer,
}

/// Sender of the original message
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(from = "RawForwardFrom")]
#[serde(into = "RawForwardFrom")]
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
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
        #[serde(skip_serializing_if = "Option::is_none")]
        forward_signature: Option<String>,
    },
}

impl From<RawForwardFrom> for ForwardFrom {
    fn from(raw: RawForwardFrom) -> Self {
        match raw {
            RawForwardFrom::Channel {
                forward_from_chat,
                forward_from_message_id,
                forward_signature,
            } => ForwardFrom::Channel {
                chat: forward_from_chat,
                message_id: forward_from_message_id,
                signature: forward_signature,
            },
            RawForwardFrom::HiddenUser { forward_sender_name } => ForwardFrom::HiddenUser(forward_sender_name),
            RawForwardFrom::User { forward_from } => ForwardFrom::User(forward_from),
        }
    }
}

impl Into<RawForwardFrom> for ForwardFrom {
    fn into(self) -> RawForwardFrom {
        match self {
            Self::Channel {
                chat: forward_from_chat,
                message_id: forward_from_message_id,
                signature: forward_signature,
            } => RawForwardFrom::Channel {
                forward_from_chat,
                forward_from_message_id,
                forward_signature,
            },
            Self::HiddenUser(forward_sender_name) => RawForwardFrom::HiddenUser { forward_sender_name },
            Self::User(forward_from) => RawForwardFrom::User { forward_from },
        }
    }
}

use crate::types::message::Message;
use serde::Deserialize;

/// Result of editMessage* requests
#[derive(Clone, Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot
    Message(Message),
    /// Returned if edited message is NOT sent by the bot
    Bool(bool),
}

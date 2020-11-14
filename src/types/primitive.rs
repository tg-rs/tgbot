use serde::Deserialize;

/// Telegram Integer type
pub type Integer = i64;

/// Telegram Float type
pub type Float = f32;

/// Represents a unique message identifier
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: Integer,
}

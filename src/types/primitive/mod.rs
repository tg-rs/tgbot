use serde::{Deserialize, Serialize};

pub(crate) use self::boolean::{False, True};
pub use self::parse_mode::*;

mod boolean;
mod parse_mode;

/// Telegram Integer type
pub type Integer = i64;

/// Telegram Float type
pub type Float = f32;

/// Represents a unique message identifier
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: Integer,
}

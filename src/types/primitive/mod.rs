pub(crate) use self::boolean::{False, True};
pub use self::parse_mode::*;

mod boolean;
mod parse_mode;

/// Represents a Telegram Integer type
pub type Integer = i64;

/// Represents a Telegram Float type
pub type Float = f32;

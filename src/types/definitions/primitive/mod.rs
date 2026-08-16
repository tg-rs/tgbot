use serde::{Deserialize, Serialize};

pub(crate) use self::boolean::True;
pub use self::parse_mode::*;

mod boolean;
mod parse_mode;

/// Represents a Telegram Integer type
pub type Integer = i64;

/// Represents a Telegram Float type
pub type Float = f32;

/// Describes an amount of Telegram Stars.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative.
    pub amount: Integer,
    /// The number of 1/1000000000 shares of Telegram Stars;
    /// from -999999999 to 999999999;
    /// can be negative if and only if amount is non-positive
    pub nanostar_amount: Option<Integer>,
}

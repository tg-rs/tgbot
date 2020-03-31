use crate::types::primitive::Integer;
use serde::Deserialize;

/// Represents a dice with random value from 1 to 6
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Dice {
    /// Value of the dice, 1-6
    pub value: Integer,
}

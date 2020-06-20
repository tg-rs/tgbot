use crate::types::primitive::Integer;
use serde::{
    de::{Deserializer, Error as _},
    Deserialize,
};
use std::{error::Error, fmt};

/// Represents a dice with random value from 1 to 6
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Dice {
    kind: DiceKind,
    value: Integer,
}

impl Dice {
    fn from_raw(raw: RawDice) -> Result<Self, DiceError> {
        Ok(Self {
            kind: DiceKind::from_raw(raw.emoji)?,
            value: raw.value,
        })
    }

    /// Kind of the dice (bones, darts)
    pub fn kind(&self) -> DiceKind {
        self.kind
    }

    /// Value of the dice, 1-6
    pub fn value(&self) -> Integer {
        self.value
    }
}

/// Kind of the dice
#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DiceKind {
    /// Bones
    Bones,
    /// Darts
    Darts,
    /// Basketball
    Basketball,
}

impl DiceKind {
    fn from_raw(raw: String) -> Result<Self, DiceError> {
        Ok(match raw.as_str() {
            "🎲" => Self::Bones,
            "🎯" => Self::Darts,
            "🏀" => Self::Basketball,
            _ => return Err(DiceError::UnexpectedEmoji(raw)),
        })
    }

    pub(crate) fn into_raw(self) -> &'static str {
        match self {
            Self::Bones => "🎲",
            Self::Darts => "🎯",
            Self::Basketball => "🏀",
        }
    }
}

#[derive(Debug)]
enum DiceError {
    UnexpectedEmoji(String),
}

impl fmt::Display for DiceError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiceError::UnexpectedEmoji(emoji) => write!(out, "unexpected emoji: {}", emoji),
        }
    }
}

impl Error for DiceError {}

impl<'de> Deserialize<'de> for Dice {
    fn deserialize<D>(deserializer: D) -> Result<Dice, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawDice = Deserialize::deserialize(deserializer)?;
        Dice::from_raw(raw).map_err(D::Error::custom)
    }
}

#[derive(Deserialize)]
struct RawDice {
    emoji: String,
    value: Integer,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize() {
        let dice: Dice = serde_json::from_value(serde_json::json!({
            "emoji": "🎯",
            "value": 1
        }))
        .unwrap();
        assert_eq!(dice.value(), 1);
        assert_eq!(dice.kind(), DiceKind::Darts);

        let dice: Dice = serde_json::from_value(serde_json::json!({
            "emoji": "🎲",
            "value": 5
        }))
        .unwrap();
        assert_eq!(dice.value(), 5);
        assert_eq!(dice.kind(), DiceKind::Bones);

        let dice: Dice = serde_json::from_value(serde_json::json!({
            "emoji": "🏀",
            "value": 3
        }))
        .unwrap();
        assert_eq!(dice.value(), 3);
        assert_eq!(dice.kind(), DiceKind::Basketball);
    }
}

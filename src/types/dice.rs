use crate::types::primitive::Integer;
use serde::{
    de::{Deserializer, Error as _},
    Deserialize,
};
use std::{error::Error, fmt};

/// Represents a dice with random value
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

    /// Kind of the dice
    pub fn kind(&self) -> DiceKind {
        self.kind
    }

    /// Value of the dice
    pub fn value(&self) -> Integer {
        self.value
    }
}

/// Kind of the dice
#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DiceKind {
    /// Basketball
    ///
    /// Value of the dice: 1-5
    Basketball,
    /// Bones
    ///
    /// Value of the dice: 1-6
    Bones,
    /// Darts
    ///
    /// Value of the dice: 1-6
    Darts,
    /// Football
    ///
    /// Value of the dice: 1-5
    Football,
    /// Slot machine
    ///
    /// Value of the dice: 1-64
    SlotMachine,
}

impl DiceKind {
    fn from_raw(raw: String) -> Result<Self, DiceError> {
        use self::DiceKind::*;
        Ok(match raw.as_str() {
            "🏀" => Basketball,
            "🎲" => Bones,
            "🎯" => Darts,
            "⚽" => Football,
            "🎰" => SlotMachine,
            _ => return Err(DiceError::UnexpectedEmoji(raw)),
        })
    }

    pub(crate) fn into_raw(self) -> &'static str {
        use self::DiceKind::*;
        match self {
            Basketball => "🏀",
            Bones => "🎲",
            Darts => "🎯",
            Football => "⚽",
            SlotMachine => "🎰",
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
            "emoji": "🏀",
            "value": 3
        }))
        .unwrap();
        assert_eq!(dice.value(), 3);
        assert_eq!(dice.kind(), DiceKind::Basketball);

        let dice: Dice = serde_json::from_value(serde_json::json!({
            "emoji": "🎲",
            "value": 5
        }))
        .unwrap();
        assert_eq!(dice.value(), 5);
        assert_eq!(dice.kind(), DiceKind::Bones);

        let dice: Dice = serde_json::from_value(serde_json::json!({
            "emoji": "🎯",
            "value": 1
        }))
        .unwrap();
        assert_eq!(dice.value(), 1);
        assert_eq!(dice.kind(), DiceKind::Darts);

        let dice: Dice = serde_json::from_value(serde_json::json!({
            "emoji": "⚽",
            "value": 3
        }))
        .unwrap();
        assert_eq!(dice.value(), 3);
        assert_eq!(dice.kind(), DiceKind::Football);

        let dice: Dice = serde_json::from_value(serde_json::json!({
            "emoji": "🎰",
            "value": 64
        }))
        .unwrap();
        assert_eq!(dice.value(), 64);
        assert_eq!(dice.kind(), DiceKind::SlotMachine);
    }
}

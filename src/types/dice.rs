use crate::types::primitive::Integer;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a dice with random value
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Dice {
    #[serde(rename = "emoji")]
    kind: DiceKind,
    value: Integer,
}

impl Dice {
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
#[derive(Debug, Copy, Clone, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
pub enum DiceKind {
    /// Basketball
    ///
    /// Value of the dice: 1-5
    #[serde(rename = "🏀")]
    Basketball,
    /// Bones
    ///
    /// Value of the dice: 1-6
    #[serde(rename = "🎲")]
    Bones,
    /// Bowling
    ///
    /// Value of the dice: 1-6
    #[serde(rename = "🎳")]
    Bowling,
    /// Darts
    ///
    /// Value of the dice: 1-6
    #[serde(rename = "🎯")]
    Darts,
    /// Football
    ///
    /// Value of the dice: 1-5
    #[serde(rename = "⚽")]
    Football,
    /// Slot machine
    ///
    /// Value of the dice: 1-64
    #[serde(rename = "🎰")]
    SlotMachine,
}

impl DiceKind {
    fn as_char(self) -> char {
        use super::DiceKind::*;
        match self {
            Basketball => '🏀',
            Bones => '🎲',
            Bowling => '🎳',
            Darts => '🎯',
            Football => '⚽',
            SlotMachine => '🎰',
        }
    }
}

impl From<DiceKind> for char {
    fn from(kind: DiceKind) -> Self {
        kind.as_char()
    }
}

impl fmt::Display for DiceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.as_char(), f)
    }
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
            "emoji": "🎳",
            "value": 5
        }))
        .unwrap();
        assert_eq!(dice.value(), 5);
        assert_eq!(dice.kind(), DiceKind::Bowling);

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

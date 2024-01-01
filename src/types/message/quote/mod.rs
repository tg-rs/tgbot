use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{Integer, Text, TextEntities};

#[cfg(test)]
mod tests;

/// Contains information about the quoted part of a message that is replied to by the given message.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct TextQuote {
    /// Approximate quote position in the original message in UTF-16 code units as specified by the sender.
    pub position: Integer,
    /// Text of the quoted part of a message that is replied to by the given message.
    #[serde(
        deserialize_with = "RawText::deserialize_value",
        flatten,
        serialize_with = "RawText::serialize_value"
    )]
    pub text: Text,
    /// Whether the quote was chosen manually by the message sender or was added automatically by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}

impl TextQuote {
    /// Creates a new `TextQuote`.
    ///
    /// # Arguments
    ///
    /// * `position` - Approximate quote position in the original message in UTF-16 code units.
    /// * `text` - Text of the quoted part of a message that is replied to by the given message.
    pub fn new<T>(position: Integer, text: T) -> Self
    where
        T: Into<Text>,
    {
        Self {
            position,
            text: text.into(),
            is_manual: None,
        }
    }

    /// Sets a new value for an `is_manual` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the quote was chosen manually by the message sender
    ///             or was added automatically by the server.
    pub fn with_is_manual(mut self, value: bool) -> Self {
        self.is_manual = Some(value);
        self
    }
}

#[derive(Deserialize, Serialize)]
struct RawText {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
}

impl RawText {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<Text, D::Error>
    where
        D: Deserializer<'de>,
    {
        RawText::deserialize(deserializer).map(|x| Text {
            data: x.text,
            entities: x.entities,
        })
    }

    fn serialize_value<S>(value: &Text, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawText {
            text: value.data.clone(),
            entities: value.entities.clone(),
        }
        .serialize(serializer)
    }
}

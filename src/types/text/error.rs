use std::{error::Error as StdError, fmt};

use serde_json::Error as JsonError;

/// An error when parsing/serializing entities
#[derive(Debug)]
pub enum TextEntityError {
    /// URL is required for text_link entity
    NoUrl,
    /// User is required for text_mention entity
    NoUser,
    /// Failed to serialize entities
    Serialize(JsonError),
}

impl StdError for TextEntityError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Serialize(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for TextEntityError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::TextEntityError::*;
        match self {
            NoUrl => write!(out, "URL is required for text_link entity"),
            NoUser => write!(out, "user is required for text_mention entity"),
            Serialize(err) => write!(out, "failed to serialize text entities: {}", err),
        }
    }
}

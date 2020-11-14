use crate::types::primitive::Integer;
use serde_json::Error as JsonError;
use std::{error::Error as StdError, fmt};

/// An error when parsing/serializing entities
#[derive(Debug)]
pub enum TextEntityError {
    /// Offset is out of text bounds
    BadOffset(Integer),
    /// Length is out of text bounds
    BadLength(Integer),
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
            BadOffset(offset) => write!(out, "offset \"{}\" is out of text bounds", offset),
            BadLength(length) => write!(out, "length \"{}\" is out of text bounds", length),
            NoUrl => write!(out, "URL is required for text_link entity"),
            NoUser => write!(out, "user is required for text_mention entity"),
            Serialize(err) => write!(out, "failed to serialize text entities: {}", err),
        }
    }
}

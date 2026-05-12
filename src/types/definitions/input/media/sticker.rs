use serde::{Deserialize, Serialize};

/// Represents a sticker file to be sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaSticker {
    emoji: Option<String>,
}

impl InputMediaSticker {
    /// Sets a new emoji.
    ///
    /// # Arguments
    ///
    /// * `value` - Emoji associated with the sticker; only for just uploaded stickers
    pub fn with_emoji<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.emoji = Some(value.into());
        self
    }
}

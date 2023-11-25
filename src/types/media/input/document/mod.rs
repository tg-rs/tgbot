use serde::{Deserialize, Serialize};

use crate::types::{ParseMode, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// Represents a general file to be sent.
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_content_type_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
}

impl InputMediaDocument {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new value for a `disable_content_type_detection` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to disable automatic server-side content type detection
    ///             for files uploaded using `multipart/form-data`.
    ///
    /// Always [`true`], if the document is sent as part of an album.
    pub fn with_disable_content_type_detection(mut self, value: bool) -> Self {
        self.disable_content_type_detection = Some(value);
        self
    }
}

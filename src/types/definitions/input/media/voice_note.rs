use serde::{Deserialize, Serialize};

use crate::types::{Integer, ParseMode, TextEntities, TextEntity};

/// Represents a voice message to be sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaVoiceNote {
    caption: Option<String>,
    caption_entities: Option<TextEntities>,
    #[serde(rename = "parse_mode")]
    caption_parse_mode: Option<ParseMode>,
    duration: Option<Integer>,
}

impl InputMediaVoiceNote {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters after entities parsing.
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
        self.caption_parse_mode = None;
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
        self.caption_parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration of the voice message in seconds.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.duration = Some(value);
        self
    }
}

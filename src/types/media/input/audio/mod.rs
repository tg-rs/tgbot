use serde::{Deserialize, Serialize};

use crate::types::{Integer, ParseMode, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// Represents an audio file to be treated as music to be sent.
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaAudio {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl InputMediaAudio {
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

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets a new performer.
    ///
    /// # Arguments
    ///
    /// * `value` - Performer of the audio.
    pub fn with_performer<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.performer = Some(value.into());
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title of the audio.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }
}

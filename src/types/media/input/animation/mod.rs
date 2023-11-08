use serde::{Deserialize, Serialize};

use crate::types::{Integer, ParseMode, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaAnimation {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
}

impl InputMediaAnimation {
    /// Sets a new caption
    ///
    /// # Arguments
    ///
    /// * value - Caption of the animation to be sent; 0-1024 characters
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new caption entities
    ///
    /// # Arguments
    ///
    /// * value - List of special entities that appear in the caption
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode
    ///
    /// # Arguments
    ///
    /// * value - Parse mode
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new duration
    ///
    /// # Arguments
    ///
    /// * value - Duration
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether photo needs to be covered with a spoiler animation
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.has_spoiler = Some(value);
        self
    }

    /// Sets a new height
    ///
    /// # Arguments
    ///
    /// * value - Height
    pub fn with_height(mut self, value: Integer) -> Self {
        self.height = Some(value);
        self
    }

    /// Sets a new width
    ///
    /// # Arguments
    ///
    /// * value - Width
    pub fn with_width(mut self, value: Integer) -> Self {
        self.width = Some(value);
        self
    }
}

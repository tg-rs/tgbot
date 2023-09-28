use serde::Serialize;

use crate::types::{Integer, ParseMode, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// Audio file to be treated as music to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaAudio {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl InputMediaAudio {
    /// Caption of the audio to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities<T>(mut self, caption_entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(caption_entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.caption_entities = None;
        self
    }

    /// Set duration
    pub fn duration(mut self, duration: Integer) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Performer of the audio
    pub fn performer<S: Into<String>>(mut self, performer: S) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Title of the audio
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }
}

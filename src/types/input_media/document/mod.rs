use serde::Serialize;

use crate::types::{ParseMode, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// General file to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_content_type_detection: Option<bool>,
}

impl InputMediaDocument {
    /// Caption of the document to be sent, 0-1024 characters
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

    /// Disables automatic server-side content type detection for
    /// files uploaded using multipart/form-data
    ///
    /// Always true, if the document is sent as part of an album
    pub fn disable_content_type_detection(mut self, value: bool) -> Self {
        self.disable_content_type_detection = Some(value);
        self
    }
}

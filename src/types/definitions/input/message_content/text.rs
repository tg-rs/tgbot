use serde::{Deserialize, Serialize};

use crate::types::{LinkPreviewOptions, ParseMode, Text, TextEntities, TextEntity};

/// Represents a text message to be sent as the result of an inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentText {
    message_text: String,
    entities: Option<TextEntities>,
    link_preview_options: Option<LinkPreviewOptions>,
    parse_mode: Option<ParseMode>,
}

impl InputMessageContentText {
    /// Creates a new `InputMessageContentText`.
    ///
    /// # Arguments
    ///
    /// * `value` - Text; 1-4096 characters.
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            message_text: value.into(),
            entities: None,
            link_preview_options: None,
            parse_mode: None,
        }
    }

    /// Sets a new list of entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the text.
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new link preview options.
    ///
    /// # Arguments
    ///
    /// * `value` - Link preview generation options for the message.
    pub fn with_link_preview_options(mut self, value: LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(value);
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.entities = None;
        self
    }
}

impl<T> From<T> for InputMessageContentText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl From<Text> for InputMessageContentText {
    fn from(value: Text) -> Self {
        let mut result = Self::new(value.data);
        if let Some(entities) = value.entities {
            result = result.with_entities(entities);
        }
        result
    }
}

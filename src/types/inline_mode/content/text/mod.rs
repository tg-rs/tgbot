use serde::{Deserialize, Serialize};

use crate::types::{ParseMode, Text, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// Represents a text message to be sent as the result of an inline query.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentText {
    message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            parse_mode: None,
            disable_web_page_preview: None,
        }
    }

    /// Sets a new value for a `disable_web_page_preview` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to disable link previews for links in the sent message.
    pub fn with_disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    /// Sets a new list of entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the text.
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_entities<T>(mut self, entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
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

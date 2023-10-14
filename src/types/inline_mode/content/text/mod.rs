use serde::{Deserialize, Serialize};

use crate::types::{ParseMode, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// Text message to be sent as the result of an inline query
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentText {
    message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

impl InputMessageContentText {
    /// Creates a new InputMessageContentText with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * message_text - Text of the message to be sent, 1-4096 characters
    pub fn new<S: Into<String>>(message_text: S) -> Self {
        InputMessageContentText {
            message_text: message_text.into(),
            entities: None,
            parse_mode: None,
            disable_web_page_preview: None,
        }
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn entities<T>(mut self, entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.entities = None;
        self
    }

    /// Disables link previews for links in the sent message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }
}

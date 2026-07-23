use serde::{Deserialize, Serialize};

use crate::types::{ParseMode, TextEntities, TextEntity};

/// Represents an input text with formatting options.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InputText {
    #[serde(rename = "text")]
    pub(crate) data: String,
    pub(crate) entities: Option<TextEntities>,
    pub(crate) parse_mode: Option<ParseMode>,
}

impl From<String> for InputText {
    fn from(value: String) -> Self {
        Self {
            data: value,
            entities: None,
            parse_mode: None,
        }
    }
}

impl From<&str> for InputText {
    fn from(value: &str) -> Self {
        Self {
            data: value.to_owned(),
            entities: None,
            parse_mode: None,
        }
    }
}

impl From<(String, ParseMode)> for InputText {
    fn from((data, parse_mode): (String, ParseMode)) -> Self {
        Self {
            data,
            entities: None,
            parse_mode: Some(parse_mode),
        }
    }
}

impl<'a> From<(&'a str, ParseMode)> for InputText {
    fn from((data, parse_mode): (&'a str, ParseMode)) -> Self {
        Self {
            data: data.to_owned(),
            entities: None,
            parse_mode: Some(parse_mode),
        }
    }
}

impl<A, B> From<(A, B)> for InputText
where
    A: Into<String>,
    B: IntoIterator<Item = TextEntity>,
{
    fn from((data, entities): (A, B)) -> Self {
        Self {
            data: data.into(),
            entities: Some(TextEntities::from_iter(entities)),
            parse_mode: None,
        }
    }
}

impl InputText {
    /// Sets a new format.
    ///
    /// # Arguments
    ///
    /// * `value` - Formatting options.
    pub fn with_format<T>(mut self, value: T) -> Self
    where
        T: Into<InputTextFormat>,
    {
        match value.into() {
            InputTextFormat::Entities(value) => {
                self.entities = Some(value);
                self.parse_mode = None;
            }
            InputTextFormat::Parse(value) => {
                self.parse_mode = Some(value);
                self.entities = None;
            }
        }
        self
    }
}

/// Represents an input text for a caption.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InputTextCaption {
    #[serde(rename = "caption")]
    data: String,
    #[serde(rename = "caption_entities")]
    entities: Option<TextEntities>,
    parse_mode: Option<ParseMode>,
}

impl<T> From<T> for InputTextCaption
where
    T: Into<InputText>,
{
    fn from(value: T) -> Self {
        let InputText {
            data,
            entities,
            parse_mode,
        } = value.into();
        Self {
            data,
            entities,
            parse_mode,
        }
    }
}

/// A text formatting options.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InputTextFormat {
    /// A list of special entities that appear in the text.
    Entities(TextEntities),
    /// Mode for parsing entities in the text.
    Parse(ParseMode),
}

impl<T> From<T> for InputTextFormat
where
    T: IntoIterator<Item = TextEntity>,
{
    fn from(value: T) -> Self {
        Self::from_iter(value)
    }
}

impl FromIterator<TextEntity> for InputTextFormat {
    fn from_iter<T: IntoIterator<Item = TextEntity>>(value: T) -> Self {
        Self::Entities(TextEntities::from_iter(value))
    }
}

impl From<ParseMode> for InputTextFormat {
    fn from(value: ParseMode) -> Self {
        Self::Parse(value)
    }
}

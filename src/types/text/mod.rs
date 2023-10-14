use std::str::EncodeUtf16;

use serde::{Deserialize, Serialize};

pub use self::entities::*;

#[cfg(test)]
mod tests;

mod entities;

/// Text with entities
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Text {
    /// The actual UTF-8 text
    pub data: String,
    /// Text entities
    pub entities: Option<TextEntities>,
}

impl Text {
    /// Returns a list of bot commands found in text
    pub fn get_bot_commands(&self) -> Option<Vec<TextEntityBotCommand>> {
        self.entities
            .as_ref()
            .map(|entities| {
                let repr = TextRepr::from(self);
                entities
                    .into_iter()
                    .filter_map(|entity| {
                        if let TextEntity::BotCommand(position) = entity {
                            let entity_data = repr.get_entity_content(*position);
                            let parts = entity_data.as_str().splitn(2, '@').collect::<Vec<&str>>();
                            let len = parts.len();
                            assert!(len >= 1);
                            let command = parts[0].to_string();
                            let bot_name = if len == 2 { Some(parts[1].to_string()) } else { None };
                            Some(TextEntityBotCommand { command, bot_name })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<TextEntityBotCommand>>()
            })
            .filter(|entities| !entities.is_empty())
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.data == other
    }
}

impl PartialEq<String> for Text {
    fn eq(&self, other: &String) -> bool {
        self.data == *other
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.data.as_str()
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self {
            data: s,
            entities: None,
        }
    }
}

/// UTF-16 text representation
struct TextRepr<'a> {
    iter: EncodeUtf16<'a>,
}

impl<'a> From<&'a Text> for TextRepr<'a> {
    fn from(text: &'a Text) -> Self {
        Self {
            iter: text.data.encode_utf16(),
        }
    }
}

impl<'a> TextRepr<'a> {
    fn get_entity_content(&self, position: TextEntityPosition) -> String {
        let (offset, length) = (position.offset as usize, position.length as usize);
        String::from_utf16_lossy(&self.iter.clone().skip(offset).take(length).collect::<Vec<u16>>())
    }
}

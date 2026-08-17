use std::str::EncodeUtf16;

use serde::{Deserialize, Serialize};

pub use self::entities::*;

mod entities;

/// Represents a text with entities.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Text {
    /// The actual UTF-8 text.
    pub data: String,
    /// Text entities.
    pub entities: Option<TextEntities>,
}

impl Text {
    /// Returns the list of bot commands found in text.
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

impl<'a> From<&'a str> for Text {
    fn from(value: &'a str) -> Self {
        Self {
            data: String::from(value),
            entities: None,
        }
    }
}

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

impl TextRepr<'_> {
    fn get_entity_content(&self, position: TextEntityPosition) -> String {
        let (offset, length) = (position.offset as usize, position.length as usize);
        String::from_utf16_lossy(&self.iter.clone().skip(offset).take(length).collect::<Vec<u16>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::*;

    #[test]
    fn get_bot_commands() {
        let input = serde_json::json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
            "text": "/command1 /command2 /command3@bot_name",
            "entities": [
                {"type": "bot_command", "offset": 0, "length": 9},
                {"type": "bot_command", "offset": 10, "length": 9},
                {"type": "bot_command", "offset": 20, "length": 18},
            ]
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        let commands = msg.get_text().and_then(|text| text.get_bot_commands()).unwrap();
        assert_eq!(commands.len(), 3);
        assert_eq!(commands[0].command, "/command1");
        assert!(commands[0].bot_name.is_none());
        assert_eq!(commands[1].command, "/command2");
        assert!(commands[1].bot_name.is_none());
        assert_eq!(commands[2].command, "/command3");
        assert_eq!(commands[2].bot_name.as_ref().unwrap(), "bot_name");
    }

    #[test]
    fn traits() {
        let text = Text::from(String::from("test"));
        assert_eq!(text, String::from("test"));
        assert_eq!(text, *"test");
        assert_eq!(text.as_ref(), "test");
    }
}

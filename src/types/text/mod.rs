use std::str::EncodeUtf16;

use serde::{Deserialize, Serialize};

pub use self::{
    entities::{TextEntities, TextEntity, TextEntityBotCommand, TextEntityPosition},
    error::TextEntityError,
};

mod entities;
mod error;
mod raw;

/// Text with entities
#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::{Message, MessageData, User};

    use super::*;

    #[test]
    fn deserialize_message_entities() {
        let input = json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "bold /botcommand $cashtag code u@h.z #hashtag italic @mention phone pre textlink textmention url underline spoiler strikethrough pre",
            "entities": [
                {"type": "bold", "offset": 0, "length": 4},
                {"type": "bot_command", "offset": 5, "length": 11},
                {"type": "cashtag", "offset": 17, "length": 8},
                {"type": "code", "offset": 26, "length": 4},
                {"type": "email", "offset": 31, "length": 5},
                {"type": "hashtag", "offset": 37, "length": 8},
                {"type": "italic", "offset": 46, "length": 6},
                {"type": "mention", "offset": 53, "length": 8},
                {"type": "phone_number", "offset": 62, "length": 5},
                {"type": "pre", "offset": 68, "length": 3},
                {"type": "text_link", "offset": 72, "length": 8, "url": "https://example.com"},
                {
                    "type": "text_mention",
                    "offset": 81,
                    "length": 11,
                    "user": {
                        "id": 1,
                        "first_name": "test",
                        "is_bot": false
                    }
                },
                {"type": "url", "offset": 93, "length": 3},
                {"type": "underline", "offset": 97, "length": 9},
                {"type": "spoiler", "offset": 107, "length": 7},
                {"type": "strikethrough", "offset": 115, "length": 13},
                {"type": "pre", "offset": 129, "length": 3, "language": "rust"},
            ]
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        if let MessageData::Text(text) = msg.data {
            let entities: Vec<TextEntity> = text.entities.unwrap().into();
            assert_eq!(
                vec![
                    TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 }),
                    TextEntity::BotCommand(TextEntityPosition { offset: 5, length: 11 }),
                    TextEntity::Cashtag(TextEntityPosition { offset: 17, length: 8 }),
                    TextEntity::Code(TextEntityPosition { offset: 26, length: 4 }),
                    TextEntity::Email(TextEntityPosition { offset: 31, length: 5 }),
                    TextEntity::Hashtag(TextEntityPosition { offset: 37, length: 8 }),
                    TextEntity::Italic(TextEntityPosition { offset: 46, length: 6 }),
                    TextEntity::Mention(TextEntityPosition { offset: 53, length: 8 }),
                    TextEntity::PhoneNumber(TextEntityPosition { offset: 62, length: 5 }),
                    TextEntity::Pre {
                        position: TextEntityPosition { offset: 68, length: 3 },
                        language: None
                    },
                    TextEntity::TextLink {
                        position: TextEntityPosition { offset: 72, length: 8 },
                        url: String::from("https://example.com")
                    },
                    TextEntity::TextMention {
                        position: TextEntityPosition { offset: 81, length: 11 },
                        user: User {
                            id: 1,
                            is_bot: false,
                            first_name: String::from("test"),
                            last_name: None,
                            username: None,
                            language_code: None
                        }
                    },
                    TextEntity::Url(TextEntityPosition { offset: 93, length: 3 }),
                    TextEntity::Underline(TextEntityPosition { offset: 97, length: 9 }),
                    TextEntity::Spoiler(TextEntityPosition { offset: 107, length: 7 }),
                    TextEntity::Strikethrough(TextEntityPosition {
                        offset: 115,
                        length: 13
                    }),
                    TextEntity::Pre {
                        position: TextEntityPosition { offset: 129, length: 3 },
                        language: Some(String::from("rust"))
                    },
                ],
                entities
            );
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }
    }

    #[test]
    fn deserialize_bad_entities() {
        for (input, error) in [
            (
                json!([{"type": "text_link", "offset": 0, "length": 2}]),
                "URL is required for text_link entity",
            ),
            (
                json!([{"type": "text_mention", "offset": 0, "length": 2}]),
                "user is required for text_mention entity",
            ),
        ] {
            let err = serde_json::from_value::<TextEntities>(input).unwrap_err();
            assert_eq!(err.to_string(), error.to_string());
        }
    }

    #[test]
    fn get_bot_commands() {
        let input = json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "/command1 /command2 /command3@botname",
            "entities": [
                {"type": "bot_command", "offset": 0, "length": 9},
                {"type": "bot_command", "offset": 10, "length": 9},
                {"type": "bot_command", "offset": 20, "length": 17},
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
        assert_eq!(commands[2].bot_name.as_ref().unwrap(), "botname");
    }

    #[test]
    fn serialize_text_entity() {
        for (entity, expected) in vec![
            (
                TextEntity::Bold(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "bold",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::BotCommand(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "bot_command",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Cashtag(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "cashtag",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Code(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "code",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Email(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "email",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Hashtag(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "hashtag",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Italic(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "italic",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Mention(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "mention",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::PhoneNumber(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "phone_number",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Pre {
                    position: TextEntityPosition { offset: 0, length: 10 },
                    language: None,
                },
                serde_json::json!({
                    "type": "pre",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Pre {
                    position: TextEntityPosition { offset: 0, length: 10 },
                    language: Some(String::from("rust")),
                },
                serde_json::json!({
                    "type": "pre",
                    "offset": 0,
                    "length": 10,
                    "language": "rust"
                }),
            ),
            (
                TextEntity::Spoiler(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "spoiler",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Strikethrough(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "strikethrough",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::TextLink {
                    position: TextEntityPosition { offset: 0, length: 21 },
                    url: String::from("https://rust-lang.org"),
                },
                serde_json::json!({
                    "type": "text_link",
                    "offset": 0,
                    "length": 21,
                    "url": "https://rust-lang.org"
                }),
            ),
            (
                TextEntity::TextMention {
                    position: TextEntityPosition { offset: 0, length: 4 },
                    user: User {
                        id: 1,
                        is_bot: false,
                        first_name: String::from("test"),
                        last_name: None,
                        username: None,
                        language_code: None,
                    },
                },
                serde_json::json!({
                    "type": "text_mention",
                    "offset": 0,
                    "length": 4,
                    "user": {
                        "id": 1,
                        "first_name": "test",
                        "is_bot": false
                    }
                }),
            ),
            (
                TextEntity::Underline(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "underline",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Url(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "url",
                    "offset": 0,
                    "length": 10
                }),
            ),
        ] {
            let value: serde_json::Value = serde_json::from_str(&serde_json::to_string(&entity).unwrap()).unwrap();
            assert_eq!(value, expected);
        }
    }
}

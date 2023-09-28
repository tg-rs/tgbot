use serde_json::json;

use crate::types::{Message, MessageData, TextEntities, TextEntity, TextEntityPosition, User};

#[test]
fn deserialize_message_entities() {
    let input = json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "bold /bot_command $cashtag code u@h.z #hashtag italic @mention phone pre text-link text-mention url underline spoiler strikethrough pre",
        "entities": [
            {"type": "bold", "offset": 0, "length": 4},
            {"type": "bot_command", "offset": 5, "length": 12},
            {"type": "cashtag", "offset": 18, "length": 8},
            {"type": "code", "offset": 27, "length": 4},
            {"type": "email", "offset": 32, "length": 5},
            {"type": "hashtag", "offset": 38, "length": 8},
            {"type": "italic", "offset": 47, "length": 6},
            {"type": "mention", "offset": 54, "length": 8},
            {"type": "phone_number", "offset": 63, "length": 5},
            {"type": "pre", "offset": 69, "length": 3},
            {"type": "text_link", "offset": 73, "length": 9, "url": "https://example.com"},
            {
                "type": "text_mention",
                "offset": 83,
                "length": 12,
                "user": {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                }
            },
            {"type": "url", "offset": 96, "length": 3},
            {"type": "underline", "offset": 100, "length": 9},
            {"type": "spoiler", "offset": 110, "length": 7},
            {"type": "strikethrough", "offset": 118, "length": 13},
            {"type": "pre", "offset": 132, "length": 3, "language": "rust"},
        ]
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let MessageData::Text(text) = msg.data {
        let entities: Vec<TextEntity> = text.entities.unwrap().into();
        assert_eq!(
            vec![
                TextEntity::Bold(TextEntityPosition { offset: 0, length: 4 }),
                TextEntity::bot_command(TextEntityPosition { offset: 5, length: 12 }),
                TextEntity::Cashtag(TextEntityPosition { offset: 18, length: 8 }),
                TextEntity::Code(TextEntityPosition { offset: 27, length: 4 }),
                TextEntity::Email(TextEntityPosition { offset: 32, length: 5 }),
                TextEntity::Hashtag(TextEntityPosition { offset: 38, length: 8 }),
                TextEntity::Italic(TextEntityPosition { offset: 47, length: 6 }),
                TextEntity::Mention(TextEntityPosition { offset: 54, length: 8 }),
                TextEntity::PhoneNumber(TextEntityPosition { offset: 63, length: 5 }),
                TextEntity::Pre {
                    position: TextEntityPosition { offset: 69, length: 3 },
                    language: None,
                },
                TextEntity::TextLink {
                    position: TextEntityPosition { offset: 73, length: 9 },
                    url: String::from("https://example.com"),
                },
                TextEntity::TextMention {
                    position: TextEntityPosition { offset: 83, length: 12 },
                    user: User {
                        id: 1,
                        is_bot: false,
                        first_name: String::from("test"),
                        last_name: None,
                        username: None,
                        language_code: None,
                    },
                },
                TextEntity::Url(TextEntityPosition { offset: 96, length: 3 }),
                TextEntity::Underline(TextEntityPosition { offset: 100, length: 9 }),
                TextEntity::Spoiler(TextEntityPosition { offset: 110, length: 7 }),
                TextEntity::Strikethrough(TextEntityPosition {
                    offset: 118,
                    length: 13,
                }),
                TextEntity::Pre {
                    position: TextEntityPosition { offset: 132, length: 3 },
                    language: Some(String::from("rust")),
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
            TextEntity::bot_command(TextEntityPosition { offset: 0, length: 10 }),
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

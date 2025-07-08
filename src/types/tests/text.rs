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

#[test]
fn deserialize() {
    let input = serde_json::json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "b /c $c cd u@h.z #h i @m p pre l tm url u s sx pre 🤡 bq ebq",
        "entities": [
            {"type": "bold", "offset": 0, "length": 1},
            {"type": "bot_command", "offset": 3, "length": 2},
            {"type": "cashtag", "offset": 6, "length": 2},
            {"type": "code", "offset": 9, "length": 2},
            {"type": "email", "offset": 12, "length": 5},
            {"type": "hashtag", "offset": 18, "length": 2},
            {"type": "italic", "offset": 21, "length": 1},
            {"type": "mention", "offset": 23, "length": 2},
            {"type": "phone_number", "offset": 26, "length": 1},
            {"type": "pre", "offset": 28, "length": 3},
            {"type": "text_link", "offset": 32, "length": 1, "url": "https://example.com"},
            {
                "type": "text_mention",
                "offset": 34,
                "length": 2,
                "user": {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                }
            },
            {"type": "url", "offset": 37, "length": 3},
            {"type": "underline", "offset": 41, "length": 1},
            {"type": "spoiler", "offset": 43, "length": 1},
            {"type": "strikethrough", "offset": 45, "length": 2},
            {"type": "pre", "offset": 48, "length": 3, "language": "rust"},
            {"type": "custom_emoji", "offset": 52, "length": 2, "custom_emoji_id": "emoji-id"},
            {"type": "blockquote", "offset": 54, "length": 2},
            {"type": "expandable_blockquote", "offset": 57, "length": 3}
        ]
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let MessageData::Text(text) = msg.data {
        let entities: Vec<TextEntity> = text.entities.unwrap().into();
        assert_eq!(
            vec![
                TextEntity::Bold(TextEntityPosition { offset: 0, length: 1 }),
                TextEntity::bot_command(TextEntityPosition { offset: 3, length: 2 }),
                TextEntity::Cashtag(TextEntityPosition { offset: 6, length: 2 }),
                TextEntity::Code(TextEntityPosition { offset: 9, length: 2 }),
                TextEntity::Email(TextEntityPosition { offset: 12, length: 5 }),
                TextEntity::Hashtag(TextEntityPosition { offset: 18, length: 2 }),
                TextEntity::Italic(TextEntityPosition { offset: 21, length: 1 }),
                TextEntity::Mention(TextEntityPosition { offset: 23, length: 2 }),
                TextEntity::PhoneNumber(TextEntityPosition { offset: 26, length: 1 }),
                TextEntity::Pre {
                    position: TextEntityPosition { offset: 28, length: 3 },
                    language: None,
                },
                TextEntity::TextLink {
                    position: TextEntityPosition { offset: 32, length: 1 },
                    url: String::from("https://example.com"),
                },
                TextEntity::TextMention {
                    position: TextEntityPosition { offset: 34, length: 2 },
                    user: User::new(1, "test", false),
                },
                TextEntity::Url(TextEntityPosition { offset: 37, length: 3 }),
                TextEntity::Underline(TextEntityPosition { offset: 41, length: 1 }),
                TextEntity::Spoiler(TextEntityPosition { offset: 43, length: 1 }),
                TextEntity::Strikethrough(TextEntityPosition { offset: 45, length: 2 }),
                TextEntity::Pre {
                    position: TextEntityPosition { offset: 48, length: 3 },
                    language: Some(String::from("rust")),
                },
                TextEntity::CustomEmoji {
                    custom_emoji_id: String::from("emoji-id"),
                    position: TextEntityPosition { offset: 52, length: 2 },
                },
                TextEntity::Blockquote(TextEntityPosition { offset: 54, length: 2 }),
                TextEntity::ExpandableBlockquote(TextEntityPosition { offset: 57, length: 3 }),
            ],
            entities
        );
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn deserialize_failed() {
    for (input, error) in [
        (
            serde_json::json!([{"type": "text_link", "offset": 0, "length": 2}]),
            "URL is required for text_link entity",
        ),
        (
            serde_json::json!([{"type": "text_mention", "offset": 0, "length": 2}]),
            "user is required for text_mention entity",
        ),
    ] {
        let err = serde_json::from_value::<TextEntities>(input).unwrap_err();
        assert_eq!(err.to_string(), error.to_string());
    }
}

#[test]
fn serialize() {
    for (entity, expected) in vec![
        (
            TextEntity::Blockquote(TextEntityPosition { offset: 0, length: 10 }),
            serde_json::json!({
                "type": "blockquote",
                "offset": 0,
                "length": 10
            }),
        ),
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
            TextEntity::custom_emoji(0..2, "emoji-id"),
            serde_json::json!({
                "type": "custom_emoji",
                "offset": 0,
                "length": 2,
                "custom_emoji_id": "emoji-id"
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
            TextEntity::ExpandableBlockquote(TextEntityPosition { offset: 0, length: 10 }),
            serde_json::json!({
                "type": "expandable_blockquote",
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
                user: User::new(1, "test", false),
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

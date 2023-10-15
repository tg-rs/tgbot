use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        Animation,
        Game,
        GameHighScore,
        GetGameHighScores,
        InlineKeyboardButton,
        PhotoSize,
        SendGame,
        SetGameScore,
        Text,
        TextEntities,
        TextEntity,
        User,
    },
};

#[test]
fn game() {
    assert_json_eq(
        Game {
            title: String::from("Game"),
            description: String::from("Description"),
            photo: vec![PhotoSize {
                file_id: String::from("photo-file-id"),
                file_unique_id: String::from("photo-file-unique-id"),
                width: 200,
                height: 200,
                file_size: None,
            }],
            text: Some(Text {
                data: String::from("text"),
                entities: Some(TextEntities::from_iter(vec![TextEntity::bold(0..2)])),
            }),
            animation: Some(Animation {
                file_id: String::from("animation-file-id"),
                file_unique_id: String::from("animation-file-unique-id"),
                width: 200,
                height: 200,
                duration: 24,
                thumb: None,
                file_name: None,
                mime_type: None,
                file_size: None,
            }),
        },
        serde_json::json!({
            "title": "Game",
            "description": "Description",
            "photo": [
                {
                    "file_id": "photo-file-id",
                    "file_unique_id": "photo-file-unique-id",
                    "width": 200,
                    "height": 200
                }
            ],
            "text": "text",
            "text_entities": [{
                "type": "bold",
                "offset": 0,
                "length": 2
            }],
            "animation": {
                "file_id": "animation-file-id",
                "file_unique_id": "animation-file-unique-id",
                "width": 200,
                "height": 200,
                "duration": 24
            }
        }),
    );
    assert_json_eq(
        Game {
            title: String::from("Game"),
            description: String::from("Description"),
            photo: vec![],
            text: None,
            animation: None,
        },
        serde_json::json!({
            "title": "Game",
            "description": "Description",
            "photo": [],
        }),
    );
}

#[test]
fn game_high_score() {
    assert_json_eq(
        GameHighScore {
            position: 1,
            user: User {
                id: 2,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            score: 3,
        },
        serde_json::json!({
            "position": 1,
            "user": {
                "id": 2,
                "first_name": "John",
                "is_bot": false
            },
            "score": 3
        }),
    );
}

#[test]
fn get_game_high_scores() {
    assert_payload_eq(
        Payload::json(
            "getGameHighScores",
            serde_json::json!({"user_id": 1, "chat_id": 2, "message_id": 3}),
        ),
        GetGameHighScores::new(1, 2, 3),
    );
    assert_payload_eq(
        Payload::json(
            "getGameHighScores",
            serde_json::json!({"user_id": 1, "inline_message_id": "inline-message-id"}),
        ),
        GetGameHighScores::with_inline_message_id(1, "inline-message-id"),
    );
}

#[test]
fn send_game() {
    let method = SendGame::new(1, "Game");
    assert_payload_eq(
        Payload::json(
            "sendGame",
            serde_json::json!({
                "chat_id": 1,
                "game_short_name": "Game"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "sendGame",
            serde_json::json!({
                "chat_id": 1,
                "game_short_name": "Game",
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "inline_keyboard": [[
                        {"text": "text", "url": "example.com"}
                    ]]
                }
            }),
        ),
        method
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "example.com")]]),
    )
}

#[test]
fn set_game_score() {
    let method = SetGameScore::new(1, 2, 3, 100);
    assert_payload_eq(
        Payload::json(
            "setGameScore",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "user_id": 3,
                "score": 100,
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setGameScore",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "user_id": 3,
                "score": 100,
                "force": true,
                "disable_edit_message": true
            }),
        ),
        method.force(true).disable_edit_message(true),
    );

    let method = SetGameScore::with_inline_message_id("inline-message-id", 3, 100);
    assert_payload_eq(
        Payload::json(
            "setGameScore",
            serde_json::json!({
                "inline_message_id": "inline-message-id",
                "user_id": 3,
                "score": 100
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setGameScore",
            serde_json::json!({
                "inline_message_id": "inline-message-id",
                "user_id": 3,
                "score": 100,
                "force": true,
                "disable_edit_message": true
            }),
        ),
        method.force(true).disable_edit_message(true),
    )
}

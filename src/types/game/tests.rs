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
        Game::new(
            "Description",
            [PhotoSize::new("photo-file-id", "photo-file-unique-id", 200, 200)],
            "Game",
        )
        .with_animation(Animation::new(
            24,
            "animation-file-id",
            "animation-file-unique-id",
            200,
            200,
        ))
        .with_text(Text::from("text").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..2)]))),
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
        Game::new("Description", [], "Game"),
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
        GameHighScore::new(1, 3, User::new(2, "John", false)),
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
        GetGameHighScores::for_chat_message(1, 2, 3),
    );
    assert_payload_eq(
        Payload::json(
            "getGameHighScores",
            serde_json::json!({"user_id": 1, "inline_message_id": "inline-message-id"}),
        ),
        GetGameHighScores::for_inline_message(1, "inline-message-id"),
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
                },
                "message_thread_id": 1
            }),
        ),
        method
            .with_allow_sending_without_reply(true)
            .with_disable_notification(true)
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "example.com")]])
            .with_reply_to_message_id(1),
    )
}

#[test]
fn set_game_score() {
    let method = SetGameScore::for_chat_message(1, 2, 3, 100);
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
        method.with_disable_edit_message(true).with_force(true),
    );

    let method = SetGameScore::for_inline_message("inline-message-id", 3, 100);
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
        method.with_disable_edit_message(true).with_force(true),
    )
}

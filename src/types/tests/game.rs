use crate::{
    api::{Payload, assert_payload_eq},
    types::*,
};

#[test]
fn game() {
    insta::assert_json_snapshot!(
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
        .with_text(Text::from("text").with_entities(TextEntities::from_iter(vec![TextEntity::bold(0..2)])))
    );
    insta::assert_json_snapshot!(Game::new("Description", [], "Game"));
}

#[test]
fn game_high_score() {
    insta::assert_json_snapshot!(GameHighScore::new(1, 3, User::new(2, "John", false)));
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
                "allow_paid_broadcast": true,
                "business_connection_id": "id",
                "disable_notification": true,
                "protect_content": true,
                "message_effect_id": "effect-id",
                "message_thread_id": 1,
                "reply_markup": {
                    "inline_keyboard": [[
                        {"text": "text", "url": "example.com"}
                    ]]
                },
                "reply_parameters": {
                    "message_id": 1
                }
            }),
        ),
        method
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "example.com")]])
            .with_reply_parameters(ReplyParameters::new(1)),
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

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{
        Game,
        GameHighScore,
        GetGameHighScores,
        InlineKeyboardButton,
        SendGame,
        SetGameScore,
        TextEntities,
        TextEntity,
    },
};

#[test]
fn game_deserialize_full() {
    let game: Game = serde_json::from_value(serde_json::json!({
        "title": "title",
        "description": "description",
        "photo": [
            {
                "file_id": "photo file id",
                "file_unique_id": "photo unique file id",
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
            "file_id": "animation file id",
            "file_unique_id": "animation unique file id",
            "width": 200,
            "height": 200,
            "duration": 24
        }
    }))
    .unwrap();
    assert_eq!(game.title, "title");
    assert_eq!(game.description, "description");
    assert_eq!(game.photo.len(), 1);
    assert_eq!(game.photo[0].file_id, "photo file id");
    assert_eq!(game.photo[0].file_unique_id, "photo unique file id");
    let text = game.text.unwrap();
    assert_eq!(text.data, "text");
    assert_eq!(
        text.entities.unwrap(),
        TextEntities::from_iter(vec![TextEntity::bold(0..2)])
    );
    let animation = game.animation.unwrap();
    assert_eq!(animation.file_id, "animation file id");
    assert_eq!(animation.file_unique_id, "animation unique file id");
}

#[test]
fn game_deserialize_partial() {
    let game: Game = serde_json::from_value(serde_json::json!({
        "title": "title",
        "description": "description",
        "photo": []
    }))
    .unwrap();
    assert_eq!(game.title, "title");
    assert_eq!(game.description, "description");
    assert_eq!(game.photo.len(), 0);
    assert!(game.text.is_none());
    assert!(game.animation.is_none());
}

#[test]
fn game_high_score_deserialize() {
    let score: GameHighScore = serde_json::from_value(serde_json::json!({
        "position": 1,
        "user": {
            "id": 2,
            "first_name": "test",
            "is_bot": false
        },
        "score": 3
    }))
    .unwrap();
    assert_eq!(score.position, 1);
    assert_eq!(score.user.id, 2);
    assert_eq!(score.score, 3);
}

#[test]
fn get_game_high_scores() {
    let request = GetGameHighScores::new(1, 2, 3).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/getGameHighScores"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["user_id"], 1);
        assert_eq!(data["chat_id"], 2);
        assert_eq!(data["message_id"], 3);
    } else {
        panic!("Unexpected request body");
    }

    let request = GetGameHighScores::with_inline_message_id(1, "msg-id").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/getGameHighScores"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["user_id"], 1);
        assert_eq!(data["inline_message_id"], "msg-id");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn send_game() {
    let request = SendGame::new(1, "name")
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendGame");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "game_short_name": "name",
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "inline_keyboard": [[
                        {"text": "text", "url": "url"}
                    ]]
                }
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn set_game_score() {
    let request = SetGameScore::new(1, 2, 3, 100)
        .force(true)
        .disable_edit_message(true)
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setGameScore");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["message_id"], 2);
        assert_eq!(data["user_id"], 3);
        assert_eq!(data["score"], 100);
        assert!(data["force"].as_bool().unwrap());
        assert!(data["disable_edit_message"].as_bool().unwrap());
    } else {
        panic!("Unexpected request body");
    }

    let request = SetGameScore::with_inline_message_id("msg-id", 3, 100)
        .force(true)
        .disable_edit_message(true)
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setGameScore");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["inline_message_id"], "msg-id");
        assert_eq!(data["user_id"], 3);
        assert_eq!(data["score"], 100);
        assert!(data["force"].as_bool().unwrap());
        assert!(data["disable_edit_message"].as_bool().unwrap());
    } else {
        panic!("Unexpected request body");
    }
}

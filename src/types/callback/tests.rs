use serde::Deserialize;

use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        AnswerCallbackQuery,
        CallbackQuery,
        Message,
        MessageData,
        SupergroupChat,
        Text,
        User,
    },
};

#[derive(Clone, Debug, Deserialize)]
struct QueryData {
    k: String,
}

#[test]
fn callback_query() {
    let user = User::new(1, "User", false);
    let expected_struct = CallbackQuery::new("id", user.clone())
        .with_message(Message::new(
            2,
            0,
            SupergroupChat::new(3, "Supergroup Chat"),
            MessageData::Text(Text::from("text")),
            user,
        ))
        .with_inline_message_id("id")
        .with_chat_instance("instance")
        .with_data(r#"{"k": "v"}"#)
        .with_game_short_name("Game");

    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": "id",
            "from": {
                "id": 1,
                "first_name": "User",
                "is_bot": false
            },
            "message": {
                "message_id": 2,
                "date": 0,
                "from": {"id": 1, "first_name": "User", "is_bot": false},
                "chat": {"id": 3, "type": "supergroup", "title": "Supergroup Chat"},
                "text": "text",
                "has_protected_content": false,
                "is_automatic_forward": false
            },
            "inline_message_id": "id",
            "chat_instance": "instance",
            "data": "{\"k\": \"v\"}",
            "game_short_name": "Game"
        }),
    );

    let parsed_query_data: QueryData = expected_struct.parse_data().unwrap().unwrap();
    assert_eq!(parsed_query_data.k, "v");

    let expected_struct = CallbackQuery::new("test", User::new(1, "test", false));
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": "test",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }),
    );

    assert!(expected_struct.parse_data::<QueryData>().unwrap().is_none());
}

#[test]
fn answer_callback_query() {
    let method = AnswerCallbackQuery::new("id");
    assert_payload_eq(
        Payload::json(
            "answerCallbackQuery",
            serde_json::json!({
                "callback_query_id": "id"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "answerCallbackQuery",
            serde_json::json!({
                "callback_query_id": "id",
                "text": "text",
                "show_alert": true,
                "url": "url",
                "cache_time": 10
            }),
        ),
        method
            .with_text("text")
            .with_show_alert(true)
            .with_url("url")
            .with_cache_time(10),
    );
}

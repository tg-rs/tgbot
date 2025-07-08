use serde::Deserialize;

use crate::{
    api::{Payload, assert_payload_eq},
    types::*,
};

#[derive(Clone, Debug, Deserialize)]
struct QueryData {
    k: String,
}

#[test]
fn callback_query() {
    let user = User::new(1, "User", false);
    let expected_struct = CallbackQuery::new("id", user.clone())
        .with_message(MaybeInaccessibleMessage::Message(Box::new(Message::new(
            2,
            1,
            SupergroupChat::new(3, "Supergroup Chat"),
            MessageData::Text(Text::from("text")),
            user,
        ))))
        .with_inline_message_id("id")
        .with_chat_instance("instance")
        .with_data(r#"{"k": "v"}"#)
        .with_game_short_name("Game");

    insta::assert_json_snapshot!(expected_struct.clone());

    let parsed_query_data: QueryData = expected_struct.parse_data().unwrap().unwrap();
    assert_eq!(parsed_query_data.k, "v");

    let expected_struct = CallbackQuery::new("test", User::new(1, "test", false));
    insta::assert_json_snapshot!(expected_struct.clone());

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

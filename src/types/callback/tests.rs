use serde::Deserialize;
use serde_json::Value;

use crate::{
    request::{RequestBody, RequestMethod},
    types::CallbackQuery,
};

use super::*;

#[derive(Clone, Debug, Deserialize)]
struct QueryData {
    k: String,
}

#[test]
fn callback_query_deserialize_full() {
    let mut data: CallbackQuery = serde_json::from_value(serde_json::json!({
        "id": "test",
        "from": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "message": {
            "message_id": 2,
            "date": 0,
            "from": {"id": 3, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 4, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test"
        },
        "inline_message_id": "inline message id",
        "chat_instance": "chat instance",
        "data": "{\"k\": \"v\"}",
        "game_short_name": "game short name"
    }))
    .unwrap();
    assert_eq!(data.id, "test");
    assert_eq!(data.from.id, 1);
    assert_eq!(data.from.first_name, "test");
    assert!(!data.from.is_bot);
    assert_eq!(data.message.take().unwrap().id, 2);
    assert_eq!(data.inline_message_id.take().unwrap(), "inline message id");
    assert_eq!(data.chat_instance.take().unwrap(), "chat instance");
    assert_eq!(data.data.take().unwrap(), r#"{"k": "v"}"#);
    assert_eq!(data.game_short_name.take().unwrap(), "game short name");
    data.data = Some(String::from(r#"{"k": "v"}"#));
    let parsed_query_data: QueryData = data.parse_data().unwrap().unwrap();
    assert_eq!(parsed_query_data.k, "v");
}

#[test]
fn callback_query_deserialize_partial() {
    let data: CallbackQuery = serde_json::from_value(serde_json::json!({
        "id": "test",
        "from": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        }
    }))
    .unwrap();
    assert_eq!(data.id, "test");
    assert_eq!(data.from.id, 1);
    assert_eq!(data.from.first_name, "test");
    assert!(!data.from.is_bot);
    assert!(data.message.is_none());
    assert!(data.inline_message_id.is_none());
    assert!(data.chat_instance.is_none());
    assert!(data.data.is_none());
    assert!(data.game_short_name.is_none());
}

#[test]
fn answer_callback_query() {
    let request = AnswerCallbackQuery::new("q-id")
        .text("text")
        .show_alert(true)
        .url("url")
        .cache_time(10)
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/answerCallbackQuery"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["callback_query_id"], "q-id");
        assert_eq!(data["text"], "text");
        assert!(data["show_alert"].as_bool().unwrap());
        assert_eq!(data["url"], "url");
        assert_eq!(data["cache_time"], 10);
    } else {
        panic!("Unexpected request body");
    }
}

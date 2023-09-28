use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{AnswerInlineQuery, ChosenInlineResult},
};

#[test]
fn answer_inline_query() {
    let request = AnswerInlineQuery::new("id", vec![])
        .cache_time(300)
        .personal(true)
        .next_offset("offset")
        .switch_pm_text("text")
        .switch_pm_parameter("param")
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/answerInlineQuery"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["inline_query_id"], "id");
        assert_eq!(data["cache_time"], 300);
        assert!(data["is_personal"].as_bool().unwrap());
        assert_eq!(data["next_offset"], "offset");
        assert_eq!(data["switch_pm_text"], "text");
        assert_eq!(data["switch_pm_parameter"], "param");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn chosen_inline_result_deserialize_full() {
    let data: ChosenInlineResult = serde_json::from_value(serde_json::json!({
        "result_id": "result-id",
        "from": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "location": {
            "latitude": 2.1,
            "longitude": 3.0
        },
        "inline_message_id": "message-id",
        "query": "q",
    }))
    .unwrap();
    assert_eq!(data.result_id, "result-id");
    assert_eq!(data.from.id, 1);
    assert_eq!(data.location.unwrap().latitude, 2.1);
    assert_eq!(data.inline_message_id.unwrap(), "message-id");
    assert_eq!(data.query, "q");
}

#[test]
fn chosen_inline_result_deserialize_partial() {
    let data: ChosenInlineResult = serde_json::from_value(serde_json::json!({
        "result_id": "result-id",
        "from": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "query": "q",
    }))
    .unwrap();
    assert_eq!(data.result_id, "result-id");
    assert_eq!(data.from.id, 1);
    assert!(data.location.is_none());
    assert!(data.inline_message_id.is_none());
    assert_eq!(data.query, "q");
}

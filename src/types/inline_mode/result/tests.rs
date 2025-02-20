use crate::types::{ChosenInlineResult, InlineQueryResultsButton, Location, User, WebAppInfo, tests::assert_json_eq};

#[test]
fn chosen_inline_result() {
    assert_json_eq(
        ChosenInlineResult::new(User::new(1, "test", false), "q", "result-id")
            .with_location(Location::new(2.0, 1.0))
            .with_inline_message_id("message-id"),
        serde_json::json!({
            "result_id": "result-id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "location": {
                "latitude": 2.0,
                "longitude": 1.0
            },
            "inline_message_id": "message-id",
            "query": "q",
        }),
    );
    assert_json_eq(
        ChosenInlineResult::new(User::new(1, "test", false), "q", "result-id"),
        serde_json::json!({
            "result_id": "result-id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "query": "q",
        }),
    );
}

#[test]
fn inline_query_results_button() {
    assert_json_eq(
        InlineQueryResultsButton::for_start_parameter("text", "param"),
        serde_json::json!({
            "text": "text",
            "start_parameter": "param"
        }),
    );
    assert_json_eq(
        InlineQueryResultsButton::for_web_app("text", WebAppInfo::from("https://example.com")),
        serde_json::json!({
            "text": "text",
            "web_app": {
                "url": "https://example.com"
            }
        }),
    );
}

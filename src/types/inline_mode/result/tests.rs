use crate::types::{tests::assert_json_eq, ChosenInlineResult, InlineQueryResultsButton, Location, User, WebAppInfo};

#[test]
fn chosen_inline_result() {
    assert_json_eq(
        ChosenInlineResult {
            result_id: String::from("result-id"),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("test"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            location: Some(Location {
                longitude: 1.0,
                latitude: 2.0,
                horizontal_accuracy: None,
                live_period: None,
                heading: None,
                proximity_alert_radius: None,
            }),
            inline_message_id: Some(String::from("message-id")),
            query: String::from("q"),
        },
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
        ChosenInlineResult {
            result_id: String::from("result-id"),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("test"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            location: None,
            inline_message_id: None,
            query: String::from("q"),
        },
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
        InlineQueryResultsButton::with_start_parameter("text", "param"),
        serde_json::json!({
            "text": "text",
            "start_parameter": "param"
        }),
    );
    assert_json_eq(
        InlineQueryResultsButton::with_web_app(
            "text",
            WebAppInfo {
                url: String::from("https://example.com"),
            },
        ),
        serde_json::json!({
            "text": "text",
            "web_app": {
                "url": "https://example.com"
            }
        }),
    );
}

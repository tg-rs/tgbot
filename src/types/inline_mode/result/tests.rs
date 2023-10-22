use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        AnswerInlineQuery,
        ChosenInlineResult,
        InlineQueryResult,
        InlineQueryResultArticle,
        InlineQueryResultsButton,
        InputMessageContent,
        InputMessageContentText,
        Location,
        User,
        WebAppInfo,
    },
};

#[test]
fn answer_inline_query() {
    let text = InputMessageContent::Text(InputMessageContentText::new("text"));
    let article = InlineQueryResult::Article(InlineQueryResultArticle::new("id", "title", text));
    let method = AnswerInlineQuery::new("id", vec![article]);

    assert_payload_eq(
        Payload::json(
            "answerInlineQuery",
            serde_json::json!({
                "inline_query_id": "id",
                "results": [
                    {
                        "type": "article",
                        "id": "id",
                        "title": "title",
                        "input_message_content": {
                            "message_text": "text"
                        }
                    }
                ]
            }),
        ),
        method.clone(),
    );

    assert_payload_eq(
        Payload::json(
            "answerInlineQuery",
            serde_json::json!({
                "inline_query_id": "id",
                "results": [
                    {
                        "type": "article",
                        "id": "id",
                        "title": "title",
                        "input_message_content": {
                            "message_text": "text"
                        }
                    }
                ],
                "cache_time": 300,
                "is_personal": true,
                "next_offset": "offset",
                "button": {
                    "text": "text",
                    "start_parameter": "param"
                }
            }),
        ),
        method
            .cache_time(300)
            .personal(true)
            .next_offset("offset")
            .button(InlineQueryResultsButton::with_start_parameter("text", "param")),
    );
}

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

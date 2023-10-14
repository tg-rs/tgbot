use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        AnswerInlineQuery,
        ChosenInlineResult,
        InlineQueryResult,
        InlineQueryResultArticle,
        InputMessageContent,
        InputMessageContentText,
        Location,
        User,
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
                "switch_pm_text": "text",
                "switch_pm_parameter": "param"
            }),
        ),
        method
            .cache_time(300)
            .personal(true)
            .next_offset("offset")
            .switch_pm_text("text")
            .switch_pm_parameter("param"),
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

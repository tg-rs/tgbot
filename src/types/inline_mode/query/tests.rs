use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        AnswerInlineQuery,
        InlineQuery,
        InlineQueryChatType,
        InlineQueryResult,
        InlineQueryResultArticle,
        InlineQueryResultsButton,
        InputMessageContent,
        InputMessageContentText,
        Location,
        User,
    },
};

#[test]
fn inline_query() {
    assert_json_eq(
        InlineQuery {
            id: String::from("query id"),
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
            query: String::from("query string"),
            offset: String::from("query offset"),
            chat_type: Some(InlineQueryChatType::Private),
            location: Some(Location {
                longitude: 2.0,
                latitude: 1.0,
                horizontal_accuracy: None,
                live_period: None,
                heading: None,
                proximity_alert_radius: None,
            }),
        },
        serde_json::json!({
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "location": {
                "latitude": 1.0,
                "longitude": 2.0
            },
            "query": "query string",
            "offset": "query offset",
            "chat_type": "private"
        }),
    );
    assert_json_eq(
        InlineQuery {
            id: String::from("query id"),
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
            query: String::from("query string"),
            offset: String::from("query offset"),
            chat_type: None,
            location: None,
        },
        serde_json::json!({
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "query": "query string",
            "offset": "query offset"
        }),
    );
}

#[test]
fn inline_query_chat_type() {
    use InlineQueryChatType::*;
    for (expected_struct, expected_value) in [
        (Sender, serde_json::json!("sender")),
        (Private, serde_json::json!("private")),
        (Group, serde_json::json!("group")),
        (Supergroup, serde_json::json!("supergroup")),
        (Channel, serde_json::json!("channel")),
    ] {
        assert_json_eq(expected_struct, expected_value);
    }
}

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

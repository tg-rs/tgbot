use crate::{
    api::{Payload, assert_payload_eq},
    types::{
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
        tests::assert_json_eq,
    },
};

#[test]
fn inline_query() {
    let expected_struct = InlineQuery::new(User::new(1, "test", false), "query id", "query offset", "query string");
    assert_json_eq(
        expected_struct
            .clone()
            .with_chat_type(InlineQueryChatType::Private)
            .with_location(Location::new(1.0, 2.0)),
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
        expected_struct,
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
    let article = InlineQueryResult::Article(InlineQueryResultArticle::new("id", text, "title"));
    let method = AnswerInlineQuery::new("id", [article]);

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
            .with_button(InlineQueryResultsButton::for_start_parameter("text", "param"))
            .with_cache_time(300)
            .with_is_personal(true)
            .with_next_offset("offset"),
    );
}

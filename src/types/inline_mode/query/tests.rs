use crate::types::{tests::assert_json_eq, InlineQuery, InlineQueryChatType, Location, User};

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

use crate::types::{InlineQuery, InlineQueryChatType};

#[test]
fn inline_query_deserialize_full() {
    let data: InlineQuery = serde_json::from_value(serde_json::json!({
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
    }))
    .unwrap();
    assert_eq!(data.id, "query id");
    assert_eq!(data.from.id, 1);
    let location = data.location.unwrap();
    assert_eq!(location.latitude, 1.0);
    assert_eq!(location.longitude, 2.0);
    assert_eq!(data.query, "query string");
    assert_eq!(data.offset, "query offset");
    assert_eq!(data.chat_type.unwrap(), InlineQueryChatType::Private);
}

#[test]
fn inline_query_deserialize_partial() {
    let data: InlineQuery = serde_json::from_value(serde_json::json!({
        "id": "query id",
        "from": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "query": "query string",
        "offset": "query offset",
    }))
    .unwrap();
    assert_eq!(data.id, "query id");
    assert_eq!(data.from.id, 1);
    assert!(data.location.is_none());
    assert_eq!(data.query, "query string");
    assert_eq!(data.offset, "query offset");
    assert!(data.chat_type.is_none());
}

#[test]
fn inline_query_chat_type_deserialize() {
    use InlineQueryChatType::*;
    for (raw, expected) in [
        (r#""sender""#, Sender),
        (r#""private""#, Private),
        (r#""group""#, Group),
        (r#""supergroup""#, Supergroup),
        (r#""channel""#, Channel),
    ] {
        assert_eq!(serde_json::from_str::<InlineQueryChatType>(raw).unwrap(), expected);
    }
}

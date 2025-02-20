use crate::types::{ParseMode, ReplyParameters, ReplyQuote, TextEntity, tests::assert_json_eq};

#[test]
fn reply_parameters() {
    assert_json_eq(ReplyParameters::new(1), serde_json::json!({"message_id": 1}));
    let mut quote = ReplyQuote::new(1, "test");
    assert_json_eq(
        ReplyParameters::new(1)
            .with_allow_sending_without_reply(true)
            .with_chat_id(1)
            .with_quote(quote.clone()),
        serde_json::json!({
            "message_id": 1,
            "allow_sending_without_reply": true,
            "chat_id": 1,
            "quote": "test",
            "quote_position": 1
        }),
    );
    quote = quote.with_entities([TextEntity::bold(0..2)]);
    assert_json_eq(
        ReplyParameters::new(1).with_quote(quote.clone()),
        serde_json::json!({
            "message_id": 1,
            "quote": "test",
            "quote_position": 1,
            "quote_entities": [{"type": "bold", "offset": 0, "length": 2}]
        }),
    );
    quote = quote.with_parse_mode(ParseMode::Markdown);
    assert_json_eq(
        ReplyParameters::new(1).with_quote(quote),
        serde_json::json!({
            "message_id": 1,
            "quote": "test",
            "quote_position": 1,
            "quote_parse_mode": "Markdown"
        }),
    );
}

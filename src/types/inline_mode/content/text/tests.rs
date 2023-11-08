use crate::types::{tests::assert_json_eq, InputMessageContent, InputMessageContentText, ParseMode, TextEntity};

#[test]
fn input_message_content_text() {
    assert_json_eq(
        InputMessageContent::from(
            InputMessageContentText::new("text")
                .with_disable_web_page_preview(true)
                .with_entities(vec![TextEntity::bold(0..10)])
                .with_parse_mode(ParseMode::Html),
        ),
        serde_json::json!({
            "message_text": "text",
            "parse_mode": "HTML",
            "disable_web_page_preview": true
        }),
    );
    assert_json_eq(
        InputMessageContent::from(
            InputMessageContentText::new("text")
                .with_parse_mode(ParseMode::Markdown)
                .with_entities(vec![TextEntity::bold(0..10)]),
        ),
        serde_json::json!({
            "message_text": "text",
            "entities": [
                {
                    "type": "bold",
                    "offset": 0,
                    "length": 10
                }
            ]
        }),
    );
}

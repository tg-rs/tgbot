use crate::types::{tests::assert_json_eq, InputMessageContent, InputMessageContentText, ParseMode, TextEntity};

#[test]
fn input_message_content_text() {
    assert_json_eq(
        InputMessageContent::from(
            InputMessageContentText::new("text")
                .entities(vec![TextEntity::bold(0..10)])
                .parse_mode(ParseMode::Html)
                .disable_web_page_preview(true),
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
                .parse_mode(ParseMode::Markdown)
                .entities(vec![TextEntity::bold(0..10)]),
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

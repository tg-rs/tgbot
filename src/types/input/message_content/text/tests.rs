use crate::types::{
    InputMessageContent,
    InputMessageContentText,
    LinkPreviewOptions,
    ParseMode,
    TextEntity,
    tests::assert_json_eq,
};

#[test]
fn input_message_content_text() {
    assert_json_eq(
        InputMessageContent::from(
            InputMessageContentText::new("text")
                .with_link_preview_options(LinkPreviewOptions::default().with_is_disabled(true))
                .with_entities(vec![TextEntity::bold(0..10)])
                .with_parse_mode(ParseMode::Html),
        ),
        serde_json::json!({
            "message_text": "text",
            "link_preview_options": {
                "is_disabled": true
            },
            "parse_mode": "HTML"
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

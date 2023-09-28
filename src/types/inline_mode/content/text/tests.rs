use crate::types::{InputMessageContent, InputMessageContentText, ParseMode, TextEntity};

#[test]
fn input_message_content_text_serialize_full() {
    assert_eq!(
        serde_json::to_value(InputMessageContent::from(
            InputMessageContentText::new("text")
                .entities(vec![TextEntity::bold(0..10)])
                .parse_mode(ParseMode::Html)
                .disable_web_page_preview(true)
        ))
        .unwrap(),
        serde_json::json!({
            "message_text": "text",
            "parse_mode": "HTML",
            "disable_web_page_preview": true
        })
    );
}

#[test]
fn input_message_content_text_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InputMessageContent::from(
            InputMessageContentText::new("text")
                .parse_mode(ParseMode::Markdown)
                .entities(vec![TextEntity::bold(0..10)])
        ))
        .unwrap(),
        serde_json::json!({
            "message_text": "text",
            "entities": [
                {
                    "type": "bold",
                    "offset": 0,
                    "length": 10
                }
            ]
        })
    );
}

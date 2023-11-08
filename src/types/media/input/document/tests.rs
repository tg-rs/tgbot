use crate::types::{tests::assert_json_eq, InputMediaDocument, ParseMode, TextEntity};

#[test]
fn input_media_document() {
    assert_json_eq(
        InputMediaDocument::default()
            .with_caption("caption")
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_disable_content_type_detection(true),
        serde_json::json!({
            "caption": "caption",
            "parse_mode": "Markdown",
            "disable_content_type_detection": true
        }),
    );
    assert_json_eq(InputMediaDocument::default(), serde_json::json!({}));
}

#[test]
fn input_media_document_entities_vs_parse_mode() {
    let mut method = InputMediaDocument::default();
    method = method.with_caption_parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&method).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    method = method.with_caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(method).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

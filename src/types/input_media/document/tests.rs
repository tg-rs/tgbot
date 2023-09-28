use crate::types::{InputMediaDocument, ParseMode, TextEntity};

#[test]
fn input_media_document_serialize() {
    assert_eq!(
        serde_json::to_value(
            InputMediaDocument::default()
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .disable_content_type_detection(true)
        )
        .unwrap(),
        serde_json::json!({
            "caption": "caption",
            "parse_mode": "Markdown",
            "disable_content_type_detection": true
        })
    );

    assert_eq!(
        serde_json::to_value(InputMediaDocument::default()).unwrap(),
        serde_json::json!({})
    );
}

#[test]
fn input_media_document_caption_entities_vs_parse_mode() {
    let mut method = InputMediaDocument::default();
    method = method.parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&method).unwrap(),
        serde_json::json!({
            "parse_mode": "Markdown"
        })
    );
    method = method.caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(method).unwrap(),
        serde_json::json!({
            "caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]
        })
    );
}

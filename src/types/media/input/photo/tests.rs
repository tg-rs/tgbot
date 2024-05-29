use crate::types::{tests::assert_json_eq, InputMediaPhoto, ParseMode, TextEntity};

#[test]
fn input_media_photo() {
    assert_json_eq(
        InputMediaPhoto::default()
            .with_caption("caption")
            .with_has_spoiler(true)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_show_caption_above_media(true),
        serde_json::json!({
            "caption": "caption",
            "parse_mode": "Markdown",
            "has_spoiler": true,
            "show_caption_above_media": true
        }),
    );
    assert_json_eq(InputMediaPhoto::default(), serde_json::json!({}));
}

#[test]
fn input_media_photo_entities_vs_parse_mode() {
    let mut method = InputMediaPhoto::default();
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

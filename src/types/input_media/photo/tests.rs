use crate::types::{InputMediaPhoto, ParseMode, TextEntity};

#[test]
fn input_media_photo_serialize() {
    assert_eq!(
        serde_json::to_value(
            InputMediaPhoto::default()
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
        )
        .unwrap(),
        serde_json::json!({
            "caption": "caption",
            "parse_mode": "Markdown"
        })
    );

    assert_eq!(
        serde_json::to_value(InputMediaPhoto::default()).unwrap(),
        serde_json::json!({})
    );
}

#[test]
fn input_media_photo_caption_entities_vs_parse_mode() {
    let mut method = InputMediaPhoto::default();
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

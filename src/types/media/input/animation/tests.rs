use crate::types::{tests::assert_json_eq, InputMediaAnimation, ParseMode, TextEntity};

#[test]
fn input_media_animation() {
    assert_json_eq(
        InputMediaAnimation::default()
            .with_caption("caption")
            .with_duration(10)
            .with_has_spoiler(true)
            .with_height(200)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_show_caption_above_media(true)
            .with_width(200),
        serde_json::json!({
            "caption": "caption",
            "parse_mode": "Markdown",
            "width": 200,
            "height": 200,
            "duration": 10,
            "has_spoiler": true,
            "show_caption_above_media": true
        }),
    );
    assert_json_eq(InputMediaAnimation::default(), serde_json::json!({}));
}

#[test]
fn input_media_animation_entities_vs_parse_mode() {
    let mut data = InputMediaAnimation::default();
    data = data.with_caption_parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&data).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    data = data.with_caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(data).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

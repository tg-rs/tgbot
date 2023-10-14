use crate::types::{tests::assert_json_eq, InputMediaVideo, ParseMode, TextEntity};

#[test]
fn input_media_video() {
    assert_json_eq(
        InputMediaVideo::default()
            .caption("caption")
            .parse_mode(ParseMode::Markdown)
            .width(200)
            .height(200)
            .duration(100)
            .supports_streaming(true),
        serde_json::json!({
            "caption": "caption",
            "parse_mode": "Markdown",
            "width": 200,
            "height": 200,
            "duration": 100,
            "supports_streaming": true
        }),
    );
    assert_json_eq(InputMediaVideo::default(), serde_json::json!({}));
}

#[test]
fn input_media_video_caption_entities_vs_parse_mode() {
    let mut method = InputMediaVideo::default();
    method = method.parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&method).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    method = method.caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(method).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

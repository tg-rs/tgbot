use crate::types::{tests::assert_json_eq, InputMediaVideo, ParseMode, TextEntity};

#[test]
fn input_media_video() {
    assert_json_eq(
        InputMediaVideo::default()
            .with_caption("caption")
            .with_duration(100)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_has_spoiler(true)
            .with_height(200)
            .with_show_caption_above_media(true)
            .with_start_timestamp(450)
            .with_supports_streaming(true)
            .with_width(200),
        serde_json::json!({
            "caption": "caption",
            "duration": 100,
            "has_spoiler": true,
            "height": 200,
            "parse_mode": "Markdown",
            "show_caption_above_media": true,
            "start_timestamp": 450,
            "supports_streaming": true,
            "width": 200,
        }),
    );
    assert_json_eq(InputMediaVideo::default(), serde_json::json!({}));
}

#[test]
fn input_media_video_entities_vs_parse_mode() {
    let mut method = InputMediaVideo::default();
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

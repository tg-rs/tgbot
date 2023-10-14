use crate::types::{tests::assert_json_eq, InputMediaAudio, ParseMode, TextEntity};

#[test]
fn input_media_audio() {
    assert_json_eq(
        InputMediaAudio::default()
            .caption("caption")
            .parse_mode(ParseMode::Markdown)
            .duration(10)
            .performer("test performer")
            .title("test title"),
        serde_json::json!({
            "caption": "caption",
            "parse_mode": "Markdown",
            "duration": 10,
            "performer": "test performer",
            "title": "test title"
        }),
    );
    assert_json_eq(InputMediaAudio::default(), serde_json::json!({}));
}

#[test]
fn input_media_audio_caption_entities_vs_parse_mode() {
    let mut data = InputMediaAudio::default();
    data = data.parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&data).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    data = data.caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(data).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

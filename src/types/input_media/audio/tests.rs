use crate::types::{InputMediaAudio, ParseMode, TextEntity};

#[test]
fn input_media_audio_serialize() {
    assert_eq!(
        serde_json::to_value(
            InputMediaAudio::default()
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .duration(10)
                .performer("test performer")
                .title("test title")
        )
        .unwrap(),
        serde_json::json!({
            "caption": "caption",
            "parse_mode": "Markdown",
            "duration": 10,
            "performer": "test performer",
            "title": "test title"
        })
    );

    assert_eq!(
        serde_json::to_value(InputMediaAudio::default()).unwrap(),
        serde_json::json!({})
    );
}

#[test]
fn input_media_audiocaption_entities_vs_parse_mode() {
    let mut method = InputMediaAudio::default();
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

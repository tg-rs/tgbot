use crate::types::{InputMediaAnimation, ParseMode, TextEntity};

#[test]
fn input_media_animation_serialize() {
    assert_eq!(
        serde_json::to_value(
            InputMediaAnimation::default()
                .thumb("attach://thumb.jpg")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .width(200)
                .height(200)
                .duration(10)
        )
        .unwrap(),
        serde_json::json!({
            "thumb": "attach://thumb.jpg",
            "caption": "caption",
            "parse_mode": "Markdown",
            "width": 200,
            "height": 200,
            "duration": 10
        })
    );

    assert_eq!(
        serde_json::to_value(InputMediaAnimation::default()).unwrap(),
        serde_json::json!({})
    );
}

#[test]
fn input_media_animation_caption_entities_vs_parse_mode() {
    let mut method = InputMediaAnimation::default();
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

use crate::types::{tests::assert_json_eq, Text, TextEntities, TextEntity, TextQuote};

#[test]
fn text_quote() {
    assert_json_eq(
        TextQuote::new(0, "test"),
        serde_json::json!({
            "position": 0,
            "text": "test"
        }),
    );
    assert_json_eq(
        TextQuote::new(
            0,
            Text::from("test").with_entities(TextEntities::from_iter([TextEntity::bold(0..2)])),
        )
        .with_is_manual(true),
        serde_json::json!({
            "position": 0,
            "text": "test",
            "entities": [
                {
                    "type": "bold",
                    "offset": 0,
                    "length": 2
                }
            ],
            "is_manual": true
        }),
    );
}

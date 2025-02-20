use crate::types::{ForceReply, ReplyMarkup, tests::assert_json_eq};

#[test]
fn force_reply() {
    for (expected_struct, expected_value) in [
        (
            ReplyMarkup::from(ForceReply::from(true)),
            serde_json::json!({"force_reply": true}),
        ),
        (
            ForceReply::new(true)
                .with_input_field_placeholder("placeholder")
                .with_selective(true)
                .into(),
            serde_json::json!({
                "force_reply": true,
                "selective": true,
                "input_field_placeholder": "placeholder"
            }),
        ),
        (
            ForceReply::new(true).with_selective(false).into(),
            serde_json::json!({
                "force_reply": true,
                "selective": false
            }),
        ),
    ] {
        assert_json_eq(expected_struct, expected_value)
    }
}

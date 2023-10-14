use crate::types::{tests::assert_json_eq, ForceReply, ReplyMarkup};

#[test]
fn force_reply() {
    for (expected_struct, expected_value) in [
        (
            ReplyMarkup::from(ForceReply::new(true)),
            serde_json::json!({"force_reply": true}),
        ),
        (
            ForceReply::new(true)
                .selective(true)
                .input_field_placeholder("placeholder")
                .into(),
            serde_json::json!({
                "force_reply": true,
                "selective": true,
                "input_field_placeholder": "placeholder"
            }),
        ),
        (
            ForceReply::new(true).selective(false).into(),
            serde_json::json!({
                "force_reply": true,
                "selective": false
            }),
        ),
    ] {
        assert_json_eq(expected_struct, expected_value)
    }
}

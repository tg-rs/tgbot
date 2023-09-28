use crate::types::{ForceReply, ReplyMarkup};

#[test]
fn serialize() {
    let markup: ReplyMarkup = ForceReply::new(true).into();
    let data = serde_json::to_value(&markup).unwrap();
    assert_eq!(data, serde_json::json!({"force_reply":true}));

    let markup: ReplyMarkup = ForceReply::new(true)
        .selective(true)
        .input_field_placeholder("placeholder")
        .into();
    let data = serde_json::to_value(&markup).unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "force_reply": true,
            "selective": true,
            "input_field_placeholder": "placeholder"
        })
    );

    let markup: ReplyMarkup = ForceReply::new(true).selective(false).into();
    let data = serde_json::to_value(&markup).unwrap();
    assert_eq!(data, serde_json::json!({"force_reply":true,"selective":false}));
}

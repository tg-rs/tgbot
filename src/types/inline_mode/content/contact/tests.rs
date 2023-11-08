use crate::types::{tests::assert_json_eq, InputMessageContent, InputMessageContentContact};

#[test]
fn input_message_content_contact() {
    let content = InputMessageContentContact::new("V", "+79001231212");
    assert_json_eq(
        InputMessageContent::from(content.clone().with_last_name("P").with_vcard("vcard")),
        serde_json::json!({
            "phone_number": "+79001231212",
            "first_name": "V",
            "last_name": "P",
            "vcard": "vcard"
        }),
    );
    assert_json_eq(
        content,
        serde_json::json!({
            "phone_number": "+79001231212",
            "first_name": "V"
        }),
    );
}

use crate::{
    tests::assert_json_eq,
    types::{InputMessageContent, InputMessageContentContact},
};

#[test]
fn input_message_content_contact() {
    let content = InputMessageContentContact::new("+79001231212", "V");
    assert_json_eq(
        InputMessageContent::from(content.clone().last_name("P").vcard("vcard")),
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

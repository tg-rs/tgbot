use crate::types::{InputMessageContent, InputMessageContentContact};

#[test]
fn input_message_content_contact_serialize_full() {
    assert_eq!(
        serde_json::to_value(InputMessageContent::from(
            InputMessageContentContact::new("+79001231212", "Vasya")
                .last_name("Pupkin")
                .vcard("vcard")
        ))
        .unwrap(),
        serde_json::json!({
            "phone_number": "+79001231212",
            "first_name": "Vasya",
            "last_name": "Pupkin",
            "vcard": "vcard"
        })
    );
}

#[test]
fn input_message_content_contact_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InputMessageContent::from(InputMessageContentContact::new(
            "+79001231212",
            "Vasya",
        )))
        .unwrap(),
        serde_json::json!({
            "phone_number": "+79001231212",
            "first_name": "Vasya"
        })
    );
}

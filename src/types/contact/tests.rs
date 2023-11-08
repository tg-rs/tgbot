use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, Contact, ForceReply, SendContact},
};

#[test]
fn concat() {
    assert_json_eq(
        Contact::new("John", "+79001231212")
            .with_last_name("Doe")
            .with_user_id(1)
            .with_vcard("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD"),
        serde_json::json!({
            "phone_number": "+79001231212",
            "first_name": "John",
            "last_name": "Doe",
            "user_id": 1,
            "vcard": "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD"
        }),
    );
    assert_json_eq(
        Contact::new("John", "+79001231212"),
        serde_json::json!({
            "phone_number": "+79001231212",
            "first_name": "John"
        }),
    );
}

#[test]
fn send_contact() {
    let method = SendContact::new(1, "John", "+79001231212");
    assert_payload_eq(
        Payload::json(
            "sendContact",
            serde_json::json!({
                "chat_id": 1,
                "phone_number": "+79001231212",
                "first_name": "John"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "sendContact",
            serde_json::json!({
                "chat_id": 1,
                "phone_number": "+79001231212",
                "first_name": "John",
                "last_name": "Doe",
                "vcard": "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD",
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "force_reply": true
                },
                "message_thread_id": 1
            }),
        ),
        method
            .with_allow_sending_without_reply(true)
            .with_disable_notification(true)
            .with_last_name("Doe")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_to_message_id(1)
            .with_vcard("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD"),
    );
}

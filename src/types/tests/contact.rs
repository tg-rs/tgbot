use crate::{
    api::{Payload, assert_payload_eq},
    types::*,
};

#[test]
fn concat() {
    insta::assert_json_snapshot!(
        Contact::new("John", "+79001231212")
            .with_last_name("Doe")
            .with_user_id(1)
            .with_vcard("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD"),
    );
    insta::assert_json_snapshot!(Contact::new("John", "+79001231212"));
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
                "allow_paid_broadcast": true,
                "business_connection_id": "id",
                "disable_notification": true,
                "protect_content": true,
                "reply_markup": {
                    "force_reply": true
                },
                "reply_parameters": {
                    "message_id": 1
                },
                "message_effect_id": "effect-id",
                "message_thread_id": 1
            }),
        ),
        method
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_last_name("Doe")
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_parameters(ReplyParameters::new(1))
            .with_vcard("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n\nEND:VCARD"),
    );
}

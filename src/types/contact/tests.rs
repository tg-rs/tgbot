use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{Contact, ForceReply, SendContact},
};

#[test]
fn concat_deserialize_full() {
    let data: Contact = serde_json::from_value(serde_json::json!({
        "phone_number": "+79001231212",
        "first_name": "First name",
        "last_name": "Last name",
        "user_id": 1234,
        "vcard": "Test vcard"
    }))
    .unwrap();

    assert_eq!(data.phone_number, "+79001231212");
    assert_eq!(data.first_name, "First name");
    assert_eq!(data.last_name.unwrap(), "Last name");
    assert_eq!(data.user_id.unwrap(), 1234);
    assert_eq!(data.vcard.unwrap(), "Test vcard");
}

#[test]
fn contact_deserialize_partial() {
    let data: Contact = serde_json::from_value(serde_json::json!({
        "phone_number": "+79001231212",
        "first_name": "First name"
    }))
    .unwrap();

    assert_eq!(data.phone_number, "+79001231212");
    assert_eq!(data.first_name, "First name");
    assert!(data.last_name.is_none());
    assert!(data.user_id.is_none());
    assert!(data.vcard.is_none());
}

#[test]
fn send_contact() {
    let request = SendContact::new(1, "phone", "first name")
        .last_name("last name")
        .vcard("vcard")
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendContact");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "phone_number": "phone",
                "first_name": "first name",
                "last_name": "last name",
                "vcard": "vcard",
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "force_reply": true
                }
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

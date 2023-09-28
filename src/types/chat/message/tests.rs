use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{PinChatMessage, UnpinAllChatMessages, UnpinChatMessage},
};

#[test]
fn pin_chat_message() {
    let request = PinChatMessage::new(1, 2).disable_notification(true).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/pinChatMessage"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["message_id"], 2);
        assert!(data["disable_notification"].as_bool().unwrap());
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn unpin_chat_message_full() {
    let request = UnpinChatMessage::new(1).message_id(2).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/unpinChatMessage"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["message_id"], 2);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn unpin_chat_message_partial() {
    let request = UnpinChatMessage::new(1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/unpinChatMessage"
    );
    if let RequestBody::Json(data) = request.into_body() {
        assert_eq!(data.unwrap(), r#"{"chat_id":1}"#);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn unpin_all_chat_messages() {
    let request = UnpinAllChatMessages::new(1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/unpinAllChatMessages"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
    } else {
        panic!("Unexpected request body");
    }
}

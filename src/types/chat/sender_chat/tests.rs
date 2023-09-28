use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{BanChatSenderChat, UnbanChatSenderChat},
};

#[test]
fn ban_chat_sender_chat() {
    let request = BanChatSenderChat::new(1, 1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/banChatSenderChat"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "sender_chat_id": 1,
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn unban_chat_sender_chat() {
    let request = UnbanChatSenderChat::new(1, 1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/unbanChatSenderChat"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "sender_chat_id": 1,
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

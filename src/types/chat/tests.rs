use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{GetChat, LeaveChat, SetChatDescription, SetChatTitle},
};

#[test]
fn get_chat() {
    let request = GetChat::new(1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/getChat");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn leave_chat() {
    let request = LeaveChat::new(1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/leaveChat");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn set_chat_description() {
    let request = SetChatDescription::new(1).description("desc").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/setChatDescription"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["description"], "desc");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn set_chat_title() {
    let request = SetChatTitle::new(1, "title").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setChatTitle");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["title"], "title");
    } else {
        panic!("Unexpected request body");
    }
}

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ApproveChatJoinRequest, ChatJoinRequest, DeclineChatJoinRequest},
};

#[test]
fn chat_join_request_deserialize_full() {
    let data: ChatJoinRequest = serde_json::from_value(serde_json::json!({
        "chat": {
            "id": 1,
            "type": "channel",
            "title": "channeltitle"
        },
        "from": {
            "id": 1,
            "is_bot": false,
            "first_name": "firstname"
        },
        "date": 0,
        "bio": "bio",
        "invite_link": {
            "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
            "creator": {
                "id": 2,
                "is_bot": false,
                "first_name": "firstname"
            },
            "creates_join_request": false,
            "is_primary": true,
            "is_revoked": false
        }
    }))
    .unwrap();
    assert_eq!(data.chat.get_id(), 1);
    assert_eq!(data.from.id, 1);
    assert_eq!(data.date, 0);
    assert_eq!(data.bio, Some(String::from("bio")));
    assert_eq!(
        data.invite_link.unwrap().invite_link,
        "https://t.me/joinchat/o8oIBrbCI3U2OGJi"
    );
}

#[test]
fn chat_join_request_deserialize_partial() {
    let data: ChatJoinRequest = serde_json::from_value(serde_json::json!({
        "chat": {
            "id": 1,
            "type": "channel",
            "title": "channeltitle"
        },
        "from": {
            "id": 1,
            "is_bot": false,
            "first_name": "firstname"
        },
        "date": 0
    }))
    .unwrap();
    assert_eq!(data.chat.get_id(), 1);
    assert_eq!(data.from.id, 1);
    assert_eq!(data.date, 0);
}

#[test]
fn approve_chat_join_request() {
    let request = ApproveChatJoinRequest::new(1, 1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/approveChatJoinRequest"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn decline_chat_join_request() {
    let request = DeclineChatJoinRequest::new(1, 1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/declineChatJoinRequest"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

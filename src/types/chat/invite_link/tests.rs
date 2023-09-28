use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ChatInviteLink, CreateChatInviteLink, EditChatInviteLink, ExportChatInviteLink, RevokeChatInviteLink},
};

#[test]
fn chat_invite_link_deserialize_full() {
    let data: ChatInviteLink = serde_json::from_value(serde_json::json!({
        "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
        "creator": {
            "id": 1,
            "is_bot": false,
            "first_name": "firstname"
        },
        "creates_join_request": true,
        "is_primary": true,
        "is_revoked": false,
        "name": "test",
        "expire_date": 0,
        "member_limit": 10,
        "pending_join_request_count": 0
    }))
    .unwrap();
    assert_eq!(data.invite_link, "https://t.me/joinchat/o8oIBrbCI3U2OGJi");
    assert_eq!(data.creator.id, 1);
    assert!(data.creates_join_request);
    assert!(data.is_primary);
    assert!(!data.is_revoked);
    assert_eq!(data.name, Some(String::from("test")));
    assert_eq!(data.expire_date, Some(0));
    assert_eq!(data.member_limit, Some(10));
    assert_eq!(data.pending_join_request_count, Some(0));
}

#[test]
fn chat_invite_link_deserialize_partial() {
    let data: ChatInviteLink = serde_json::from_value(serde_json::json!({
        "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
        "creator": {
            "id": 1,
            "is_bot": false,
            "first_name": "firstname"
        },
        "creates_join_request": false,
        "is_primary": true,
        "is_revoked": false
    }))
    .unwrap();
    assert_eq!(data.invite_link, "https://t.me/joinchat/o8oIBrbCI3U2OGJi");
    assert_eq!(data.creator.id, 1);
    assert!(!data.creates_join_request);
    assert!(data.is_primary);
    assert!(!data.is_revoked);
    assert!(data.expire_date.is_none());
    assert!(data.member_limit.is_none());
}

#[test]
fn create_chat_invite_link() {
    let request = CreateChatInviteLink::new(1)
        .name("test")
        .expire_date(0)
        .member_limit(1)
        .creates_join_request(false)
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/createChatInviteLink"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "name": "test",
                "expire_date": 0,
                "member_limit": 1,
                "creates_join_request": false
            })
        );
    } else {
        panic!("Unexpected request body");
    }

    let request = CreateChatInviteLink::new(1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/createChatInviteLink"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data, serde_json::json!({"chat_id": 1}));
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn edit_chat_invite_link() {
    let request = EditChatInviteLink::new(1, "test")
        .name("test")
        .expire_date(0)
        .member_limit(1)
        .creates_join_request(false)
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editChatInviteLink"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "test",
                "name": "test",
                "expire_date": 0,
                "member_limit": 1,
                "creates_join_request": false
            })
        );
    } else {
        panic!("Unexpected request body");
    }

    let request = EditChatInviteLink::new(1, "test").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editChatInviteLink"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data, serde_json::json!({"chat_id": 1, "invite_link": "test"}));
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn export_chat_invite_link() {
    let request = ExportChatInviteLink::new(1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/exportChatInviteLink"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn revoke_chat_invite_link() {
    let request = RevokeChatInviteLink::new(1, "test").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/revokeChatInviteLink"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data, serde_json::json!({"chat_id": 1, "invite_link": "test"}));
    } else {
        panic!("Unexpected request body");
    }
}

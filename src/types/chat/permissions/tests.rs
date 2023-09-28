use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ChatPermissions, SetChatPermissions},
};

#[test]
fn chat_permissions_deserialize() {
    let data: ChatPermissions = serde_json::from_value(serde_json::json!({
        "can_send_messages": true,
        "can_send_media_messages": false,
        "can_send_polls": true,
        "can_send_other_messages": false,
        "can_add_web_page_previews": true,
        "can_change_info": false,
        "can_invite_users": true,
        "can_pin_messages": false,
    }))
    .unwrap();
    assert!(data.can_send_messages.unwrap());
    assert!(!data.can_send_media_messages.unwrap());
    assert!(data.can_send_polls.unwrap());
    assert!(!data.can_send_other_messages.unwrap());
    assert!(data.can_add_web_page_previews.unwrap());
    assert!(!data.can_change_info.unwrap());
    assert!(data.can_invite_users.unwrap());
    assert!(!data.can_pin_messages.unwrap());
}

#[test]
fn chat_permissions_serialize() {
    let permissions = ChatPermissions::default()
        .with_send_messages(true)
        .with_send_media_messages(false)
        .with_send_polls(true)
        .with_send_other_messages(false)
        .with_add_web_page_previews(true)
        .with_change_info(false)
        .with_invite_users(true)
        .with_pin_messages(false);
    assert_eq!(
        serde_json::to_value(permissions).unwrap(),
        serde_json::json!({
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_polls": true,
            "can_send_other_messages": false,
            "can_add_web_page_previews": true,
            "can_change_info": false,
            "can_invite_users": true,
            "can_pin_messages": false,
        })
    );
    assert_eq!(
        serde_json::to_value(ChatPermissions::default()).unwrap(),
        serde_json::json!({})
    );

    assert_eq!(
        serde_json::to_value(ChatPermissions::allowed()).unwrap(),
        serde_json::json!({
            "can_send_messages": true,
            "can_send_media_messages": true,
            "can_send_polls": true,
            "can_send_other_messages": true,
            "can_add_web_page_previews": true,
            "can_change_info": true,
            "can_invite_users": true,
            "can_pin_messages": true,
        })
    );

    assert_eq!(
        serde_json::to_value(ChatPermissions::restricted()).unwrap(),
        serde_json::json!({
            "can_send_messages": false,
            "can_send_media_messages": false,
            "can_send_polls": false,
            "can_send_other_messages": false,
            "can_add_web_page_previews": false,
            "can_change_info": false,
            "can_invite_users": false,
            "can_pin_messages": false,
        })
    );
}

#[test]
fn set_chat_permissions() {
    let request = SetChatPermissions::new(1, ChatPermissions::default().with_send_messages(true)).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/setChatPermissions"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["permissions"], serde_json::json!({"can_send_messages": true}));
    } else {
        panic!("Unexpected request body");
    }
}

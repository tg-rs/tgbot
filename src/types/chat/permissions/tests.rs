use crate::{
    tests::{assert_json_eq, assert_request_eq, ExpectedRequest},
    types::{ChatPermissions, SetChatPermissions},
};

#[test]
fn chat_permissions() {
    assert_json_eq(
        ChatPermissions::default()
            .with_send_messages(true)
            .with_send_media_messages(false)
            .with_send_polls(true)
            .with_send_other_messages(false)
            .with_add_web_page_previews(true)
            .with_change_info(false)
            .with_invite_users(true)
            .with_pin_messages(false),
        serde_json::json!({
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_polls": true,
            "can_send_other_messages": false,
            "can_add_web_page_previews": true,
            "can_change_info": false,
            "can_invite_users": true,
            "can_pin_messages": false,
        }),
    );
    assert_json_eq(ChatPermissions::default(), serde_json::json!({}));
    assert_json_eq(
        ChatPermissions::allowed(),
        serde_json::json!({
            "can_send_messages": true,
            "can_send_media_messages": true,
            "can_send_polls": true,
            "can_send_other_messages": true,
            "can_add_web_page_previews": true,
            "can_change_info": true,
            "can_invite_users": true,
            "can_pin_messages": true,
        }),
    );
    assert_json_eq(
        ChatPermissions::restricted(),
        serde_json::json!({
            "can_send_messages": false,
            "can_send_media_messages": false,
            "can_send_polls": false,
            "can_send_other_messages": false,
            "can_add_web_page_previews": false,
            "can_change_info": false,
            "can_invite_users": false,
            "can_pin_messages": false,
        }),
    );
}

#[test]
fn set_chat_permissions() {
    let permissions = ChatPermissions::default().with_send_messages(true);
    assert_request_eq(
        ExpectedRequest::post_json(
            "setChatPermissions",
            serde_json::json!({
                "chat_id": 1,
                "permissions": {
                    "can_send_messages": true
                }
            }),
        ),
        SetChatPermissions::new(1, permissions),
    );
}

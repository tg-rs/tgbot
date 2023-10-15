use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, ChatAdministratorRights, ChatPermissions, SetChatPermissions},
};

#[test]
fn chat_administrator_rights() {
    assert_json_eq(
        ChatAdministratorRights::default(),
        serde_json::json!({
            "is_anonymous": false,
            "can_manage_chat": false,
            "can_delete_messages": false,
            "can_manage_video_chats": false,
            "can_restrict_members": false,
            "can_promote_members": false,
            "can_change_info": false,
            "can_invite_users": false,
        }),
    );

    assert_json_eq(
        ChatAdministratorRights::all(),
        serde_json::json!({
            "is_anonymous": true,
            "can_manage_chat": true,
            "can_delete_messages": true,
            "can_manage_video_chats": true,
            "can_restrict_members": true,
            "can_promote_members": true,
            "can_change_info": true,
            "can_invite_users": true,
            "can_post_messages": true,
            "can_edit_messages": true,
            "can_pin_messages": true,
            "can_post_stories": true,
            "can_edit_stories": true,
            "can_delete_stories": true,
            "can_manage_topics": true,
        }),
    );

    assert_json_eq(
        ChatAdministratorRights::default()
            .with_can_post_messages(true)
            .with_can_edit_messages(false)
            .with_can_pin_messages(true)
            .with_can_post_stories(false)
            .with_can_edit_stories(true)
            .with_can_delete_stories(false)
            .with_can_manage_topics(true),
        serde_json::json!({
            "is_anonymous": false,
            "can_manage_chat": false,
            "can_delete_messages": false,
            "can_manage_video_chats": false,
            "can_restrict_members": false,
            "can_promote_members": false,
            "can_change_info": false,
            "can_invite_users": false,
            "can_post_messages": true,
            "can_edit_messages": false,
            "can_pin_messages": true,
            "can_post_stories": false,
            "can_edit_stories": true,
            "can_delete_stories": false,
            "can_manage_topics": true,
        }),
    )
}

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
    assert_payload_eq(
        Payload::json(
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

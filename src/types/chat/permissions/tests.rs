use crate::{
    api::{Payload, assert_payload_eq},
    types::{ChatAdministratorRights, ChatPermissions, SetChatPermissions, tests::assert_json_eq},
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
            .with_can_change_info(true)
            .with_can_delete_messages(false)
            .with_can_delete_stories(false)
            .with_can_edit_messages(false)
            .with_can_edit_stories(true)
            .with_can_invite_users(false)
            .with_can_manage_chat(true)
            .with_can_manage_topics(true)
            .with_can_manage_video_chats(false)
            .with_can_pin_messages(true)
            .with_can_post_messages(true)
            .with_can_post_stories(false)
            .with_can_promote_members(true)
            .with_can_restrict_members(false)
            .with_is_anonymous(true),
        serde_json::json!({
            "can_change_info": true,
            "can_delete_messages": false,
            "can_delete_stories": false,
            "can_edit_messages": false,
            "can_edit_stories": true,
            "can_invite_users": false,
            "can_manage_chat": true,
            "can_manage_topics": true,
            "can_manage_video_chats": false,
            "can_pin_messages": true,
            "can_post_messages": true,
            "can_post_stories": false,
            "can_promote_members": true,
            "can_restrict_members": false,
            "is_anonymous": true,
        }),
    )
}

#[test]
fn chat_permissions() {
    assert_json_eq(
        ChatPermissions::default()
            .with_can_send_messages(true)
            .with_can_send_audios(false)
            .with_can_send_documents(true)
            .with_can_send_photos(false)
            .with_can_send_videos(true)
            .with_can_send_video_notes(false)
            .with_can_send_voice_notes(true)
            .with_can_send_polls(true)
            .with_can_send_other_messages(false)
            .with_can_add_web_page_previews(true)
            .with_can_change_info(false)
            .with_can_invite_users(true)
            .with_can_pin_messages(false)
            .with_can_manage_topics(true),
        serde_json::json!({
            "can_send_messages": true,
            "can_send_audios": false,
            "can_send_documents": true,
            "can_send_photos": false,
            "can_send_videos": true,
            "can_send_video_notes": false,
            "can_send_voice_notes": true,
            "can_send_polls": true,
            "can_send_other_messages": false,
            "can_add_web_page_previews": true,
            "can_change_info": false,
            "can_invite_users": true,
            "can_pin_messages": false,
            "can_manage_topics": true,
        }),
    );
    assert_json_eq(ChatPermissions::default(), serde_json::json!({}));
    assert_json_eq(
        ChatPermissions::allowed(),
        serde_json::json!({
            "can_send_messages": true,
            "can_send_audios": true,
            "can_send_documents": true,
            "can_send_photos": true,
            "can_send_videos": true,
            "can_send_video_notes": true,
            "can_send_voice_notes": true,
            "can_send_polls": true,
            "can_send_other_messages": true,
            "can_add_web_page_previews": true,
            "can_change_info": true,
            "can_invite_users": true,
            "can_pin_messages": true,
            "can_manage_topics": true,
        }),
    );
    assert_json_eq(
        ChatPermissions::restricted(),
        serde_json::json!({
            "can_send_messages": false,
            "can_send_audios": false,
            "can_send_documents": false,
            "can_send_photos": false,
            "can_send_videos": false,
            "can_send_video_notes": false,
            "can_send_voice_notes": false,
            "can_send_polls": false,
            "can_send_other_messages": false,
            "can_add_web_page_previews": false,
            "can_change_info": false,
            "can_invite_users": false,
            "can_pin_messages": false,
            "can_manage_topics": false,
        }),
    );
}

#[test]
fn set_chat_permissions() {
    let permissions = ChatPermissions::default().with_can_send_messages(true);
    let method = SetChatPermissions::new(1, permissions);
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
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setChatPermissions",
            serde_json::json!({
                "chat_id": 1,
                "permissions": {
                    "can_send_messages": true
                },
                "use_independent_chat_permissions": true
            }),
        ),
        method.with_use_independent_chat_permissions(true),
    );
}

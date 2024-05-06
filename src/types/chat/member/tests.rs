use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq, BanChatMember, Chat, ChatInviteLink, ChatMember, ChatMemberAdministrator,
        ChatMemberCreator, ChatMemberKicked, ChatMemberRestricted, ChatMemberUpdated, ChatPermissions,
        GetChatAdministrators, GetChatMember, GetChatMemberCount, GroupChat, PromoteChatMember, RestrictChatMember,
        SetChatAdministratorCustomTitle, UnbanChatMember, User,
    },
};

#[test]
fn chat_member_admin() {
    let expected_struct = ChatMember::Administrator(
        ChatMemberAdministrator::new(
            User::new(1, "John", false)
                .with_last_name("Doe")
                .with_username("john_doe")
                .with_language_code("RU"),
        )
        .with_custom_title("Alpha")
        .with_is_anonymous(false)
        .with_can_be_edited(true)
        .with_can_change_info(false)
        .with_can_post_messages(true)
        .with_can_edit_messages(false)
        .with_can_delete_messages(true)
        .with_can_invite_users(false)
        .with_can_restrict_members(true)
        .with_can_pin_messages(false)
        .with_can_post_stories(true)
        .with_can_edit_stories(true)
        .with_can_promote_members(true)
        .with_can_manage_video_chats(false)
        .with_can_manage_chat(true)
        .with_can_manage_topics(true)
        .with_can_delete_stories(true),
    );
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "administrator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John",
                "last_name": "Doe",
                "username": "john_doe",
                "language_code": "RU"
            },
            "custom_title": "Alpha",
            "is_anonymous": false,
            "can_be_edited": true,
            "can_change_info": false,
            "can_post_messages": true,
            "can_edit_messages": false,
            "can_delete_messages": true,
            "can_invite_users": false,
            "can_restrict_members": true,
            "can_pin_messages": false,
            "can_promote_members": true,
            "can_manage_video_chats": false,
            "can_manage_chat": true,
            "can_manage_topics": true,
            "can_post_stories": true,
            "can_edit_stories": true,
            "can_delete_stories": true,
        }),
    );
    let expected_struct = ChatMember::Administrator(
        ChatMemberAdministrator::new(User::new(1, "John", false))
            .with_can_be_edited(true)
            .with_can_delete_messages(true)
            .with_can_restrict_members(true)
            .with_can_promote_members(true)
            .with_can_manage_chat(true),
    );
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "administrator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John",
            },
            "is_anonymous": false,
            "can_be_edited": true,
            "can_change_info": false,
            "can_delete_messages": true,
            "can_invite_users": false,
            "can_restrict_members": true,
            "can_promote_members": true,
            "can_manage_video_chats": false,
            "can_manage_chat": true,
        }),
    );
}

#[test]
fn chat_member_creator() {
    let expected_struct = ChatMember::Creator(
        ChatMemberCreator::new(User::new(1, "John", false))
            .with_is_anonymous(false)
            .with_custom_title("Alpha"),
    );

    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "creator",
            "is_anonymous": false,
            "custom_title": "Alpha",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John"
            }
        }),
    );
    let expected_struct = ChatMember::Creator(ChatMemberCreator::new(User::new(1, "John", false)));
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "creator",
            "is_anonymous": false,
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John"
            }
        }),
    );
}

#[test]
fn chat_member_kicked() {
    let expected_struct = ChatMember::Kicked(ChatMemberKicked::new(
        0,
        User::new(1, "John", false)
            .with_last_name("Doe")
            .with_language_code("RU")
            .with_username("john_doe"),
    ));
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "kicked",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John",
                "last_name": "Doe",
                "username": "john_doe",
                "language_code": "RU"
            },
            "until_date": 0
        }),
    );
}

#[test]
fn chat_member_left() {
    let expected_struct = ChatMember::Left(User::new(1, "John", true));
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "left",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            }
        }),
    );
}

#[test]
fn chat_member() {
    let expected_struct = ChatMember::Member(User::new(1, "John", false));
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "member",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John"
            }
        }),
    );
}

#[test]
fn chat_member_restricted() {
    let expected_struct = ChatMember::Restricted(
        ChatMemberRestricted::new(User::new(1, "John", false), 0)
            .with_can_change_info(true)
            .with_can_invite_users(false)
            .with_can_send_polls(true)
            .with_can_pin_messages(false)
            .with_can_send_messages(true)
            .with_can_send_audios(true)
            .with_can_send_documents(false)
            .with_can_send_photos(true)
            .with_can_send_videos(false)
            .with_can_send_video_notes(true)
            .with_can_send_other_messages(true)
            .with_can_add_web_page_previews(false)
            .with_can_manage_topics(false)
            .with_is_member(true)
            .with_can_send_voice_notes(false),
    );
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "restricted",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John"
            },
            "until_date": 0,
            "can_change_info": true,
            "can_invite_users": false,
            "can_send_polls": true,
            "can_pin_messages": false,
            "can_send_messages": true,
            "can_send_audios": true,
            "can_send_documents": false,
            "can_send_photos": true,
            "can_send_videos": false,
            "can_send_video_notes": true,
            "can_send_voice_notes": false,
            "can_send_other_messages": true,
            "can_add_web_page_previews": false,
            "can_manage_topics": false,
            "is_member": true
        }),
    );
    let expected_struct = ChatMember::Restricted(
        ChatMemberRestricted::new(User::new(1, "John", true), 0)
            .with_can_change_info(true)
            .with_can_send_polls(true)
            .with_can_send_messages(true)
            .with_can_send_other_messages(true),
    );
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "restricted",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            },
            "until_date": 0,
            "can_change_info": true,
            "can_invite_users": false,
            "can_send_polls": true,
            "can_send_messages": true,
            "can_send_other_messages": true,
            "can_add_web_page_previews": false,
            "can_manage_topics": false,
            "is_member": false
        }),
    );
}

#[test]
fn chat_member_updated() {
    assert_json_eq(
        ChatMemberUpdated::new(
            Chat::Group(GroupChat::new(1, "group-title")),
            0,
            User::new(1, "John", true),
            ChatMember::Kicked(ChatMemberKicked::new(0, User::new(2, "John", false))),
            ChatMember::Member(User::new(2, "John", false)),
        )
        .with_invite_link(
            ChatInviteLink::new("https://t.me/joinchat/o8oIBrbCI3U2OGJi", User::new(1, "John", false))
                .with_is_primary(true),
        )
        .with_via_chat_folder_invite_link(true)
        .with_via_join_request(true),
        serde_json::json!({
            "chat": {
                "id": 1,
                "type": "group",
                "title": "group-title"
            },
            "from": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            },
            "date": 0,
            "old_chat_member": {
                "status": "member",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "John"
                }
            },
            "new_chat_member": {
                "status": "kicked",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "John",
                },
                "until_date": 0
            },
            "invite_link": {
                "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
                "creator": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "John"
                },
                "creates_join_request": false,
                "is_primary": true,
                "is_revoked": false
            },
            "via_chat_folder_invite_link": true,
            "via_join_request": true
        }),
    );
    assert_json_eq(
        ChatMemberUpdated::new(
            Chat::Group(GroupChat::new(1, "group-title")),
            0,
            User::new(1, "John", true),
            ChatMember::Kicked(ChatMemberKicked::new(0, User::new(2, "John", false))),
            ChatMember::Member(User::new(2, "John", false)),
        ),
        serde_json::json!({
            "chat": {
                "id": 1,
                "type": "group",
                "title": "group-title"
            },
            "from": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            },
            "date": 0,
            "old_chat_member": {
                "status": "member",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "John"
                }
            },
            "new_chat_member": {
                "status": "kicked",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "John",
                },
                "until_date": 0
            }
        }),
    );
}

#[test]
fn ban_chat_member() {
    let method = BanChatMember::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "banChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "banChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 3,
                "revoke_messages": true,
            }),
        ),
        method.with_until_date(3).with_revoke_messages(true),
    );
}

#[test]
fn get_chat_administrators() {
    assert_payload_eq(
        Payload::json(
            "getChatAdministrators",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChatAdministrators::new(1),
    );
}

#[test]
fn get_chat_member() {
    assert_payload_eq(
        Payload::json(
            "getChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        GetChatMember::new(1, 2),
    );
}

#[test]
fn get_chat_members_count() {
    assert_payload_eq(
        Payload::json(
            "getChatMemberCount",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChatMemberCount::new(1),
    );
}

#[test]
fn promote_chat_member() {
    assert_payload_eq(
        Payload::json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": true,
                "can_change_info": true,
                "can_delete_messages": true,
                "can_edit_messages": true,
                "can_invite_users": true,
                "can_manage_chat": true,
                "can_manage_video_chats": true,
                "can_pin_messages": true,
                "can_post_messages": true,
                "can_promote_members": true,
                "can_restrict_members": true,
                "can_manage_topics": true,
                "can_post_stories": true,
                "can_edit_stories": true,
                "can_delete_stories": true
            }),
        ),
        PromoteChatMember::new(1, 2).promote_all(),
    );
    assert_payload_eq(
        Payload::json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": false,
                "can_change_info": false,
                "can_delete_messages": false,
                "can_edit_messages": false,
                "can_invite_users": false,
                "can_manage_chat": false,
                "can_manage_video_chats": false,
                "can_pin_messages": false,
                "can_post_messages": false,
                "can_promote_members": false,
                "can_restrict_members": false,
                "can_manage_topics": false,
                "can_post_stories": false,
                "can_edit_stories": false,
                "can_delete_stories": false
            }),
        ),
        PromoteChatMember::new(1, 2).demote_all(),
    );
    assert_payload_eq(
        Payload::json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": false,
                "can_change_info": true,
                "can_delete_messages": false,
                "can_edit_messages": true,
                "can_invite_users": true,
                "can_manage_chat": false,
                "can_manage_video_chats": true,
                "can_pin_messages": true,
                "can_post_messages": false,
                "can_promote_members": false,
                "can_restrict_members": false,
                "can_manage_topics": true,
                "can_post_stories": true,
                "can_edit_stories": true,
                "can_delete_stories": true
            }),
        ),
        PromoteChatMember::new(1, 2)
            .with_is_anonymous(false)
            .with_can_change_info(true)
            .with_can_edit_messages(true)
            .with_can_delete_messages(false)
            .with_can_invite_users(true)
            .with_can_manage_chat(false)
            .with_can_manage_video_chats(true)
            .with_can_pin_messages(true)
            .with_can_post_messages(false)
            .with_can_promote_members(false)
            .with_can_restrict_members(false)
            .with_can_manage_topics(true)
            .with_can_post_stories(true)
            .with_can_edit_stories(true)
            .with_can_delete_stories(true),
    );
    assert_payload_eq(
        Payload::json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        PromoteChatMember::new(1, 2),
    );
}

#[test]
fn restrict_chat_member() {
    let method = RestrictChatMember::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "permissions": {}
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
                "permissions": {}
            }),
        ),
        method.with_until_date(100),
    );
    let method = RestrictChatMember::new(1, 2).allow_all();
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "permissions": {
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
                }
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
                "permissions": {
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
                }
            }),
        ),
        method.with_until_date(100),
    );
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
                "permissions": {
                    "can_send_messages": true,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": false
                },
                "use_independent_chat_permissions": true
            }),
        ),
        RestrictChatMember::new(1, 2)
            .with_permissions(
                ChatPermissions::default()
                    .with_can_send_messages(true)
                    .with_can_send_other_messages(true)
                    .with_can_add_web_page_previews(false),
            )
            .with_until_date(100)
            .with_use_independent_chat_permissions(true),
    );
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "permissions": {}
            }),
        ),
        RestrictChatMember::new(1, 2),
    );
}

#[test]
fn set_chat_administrator_custom_title() {
    assert_payload_eq(
        Payload::json(
            "setChatAdministratorCustomTitle",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
                "custom_title": "Alpha"
            }),
        ),
        SetChatAdministratorCustomTitle::new(1, "Alpha", 1),
    );
}

#[test]
fn unban_chat_member() {
    let method = UnbanChatMember::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "unbanChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "unbanChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "only_if_banned": true
            }),
        ),
        method.with_only_if_banned(true),
    );
}

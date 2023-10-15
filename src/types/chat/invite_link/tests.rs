use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        ChatInviteLink,
        CreateChatInviteLink,
        EditChatInviteLink,
        ExportChatInviteLink,
        RevokeChatInviteLink,
        User,
    },
};

#[test]
fn chat_invite_link() {
    assert_json_eq(
        ChatInviteLink {
            invite_link: String::from("example.com/join/chat"),
            creator: User {
                id: 1,
                is_bot: false,
                first_name: String::from("User"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
            },
            creates_join_request: true,
            is_primary: true,
            is_revoked: false,
            name: Some(String::from("Link")),
            expire_date: Some(0),
            member_limit: Some(10),
            pending_join_request_count: Some(0),
        },
        serde_json::json!({
            "invite_link": "example.com/join/chat",
            "creator": {
                "id": 1,
                "is_bot": false,
                "first_name": "User"
            },
            "creates_join_request": true,
            "is_primary": true,
            "is_revoked": false,
            "name": "Link",
            "expire_date": 0,
            "member_limit": 10,
            "pending_join_request_count": 0
        }),
    );
    assert_json_eq(
        ChatInviteLink {
            invite_link: String::from("example.com/join/chat"),
            creator: User {
                id: 1,
                is_bot: false,
                first_name: String::from("User"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
            },
            creates_join_request: true,
            is_primary: true,
            is_revoked: false,
            name: None,
            expire_date: None,
            member_limit: None,
            pending_join_request_count: None,
        },
        serde_json::json!({
            "invite_link": "example.com/join/chat",
            "creator": {
                "id": 1,
                "is_bot": false,
                "first_name": "User"
            },
            "creates_join_request": true,
            "is_primary": true,
            "is_revoked": false,
        }),
    );
}

#[test]
fn create_chat_invite_link() {
    let method = CreateChatInviteLink::new(1);
    assert_payload_eq(
        Payload::json(
            "createChatInviteLink",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "createChatInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "name": "Link",
                "expire_date": 0,
                "member_limit": 1,
                "creates_join_request": false
            }),
        ),
        method
            .name("Link")
            .expire_date(0)
            .member_limit(1)
            .creates_join_request(false),
    )
}

#[test]
fn edit_chat_invite_link() {
    let method = EditChatInviteLink::new(1, "example.com/join/chat");
    assert_payload_eq(
        Payload::json(
            "editChatInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "example.com/join/chat"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "editChatInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "example.com/join/chat",
                "name": "Link",
                "expire_date": 0,
                "member_limit": 1,
                "creates_join_request": false
            }),
        ),
        method
            .name("Link")
            .expire_date(0)
            .member_limit(1)
            .creates_join_request(false),
    );
}

#[test]
fn export_chat_invite_link() {
    assert_payload_eq(
        Payload::json(
            "exportChatInviteLink",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        ExportChatInviteLink::new(1),
    );
}

#[test]
fn revoke_chat_invite_link() {
    assert_payload_eq(
        Payload::json(
            "revokeChatInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "example.com/join/chat"
            }),
        ),
        RevokeChatInviteLink::new(1, "example.com/join/chat"),
    );
}

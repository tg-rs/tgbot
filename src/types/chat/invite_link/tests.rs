use crate::{
    api::{Payload, assert_payload_eq},
    types::{
        ChatInviteLink,
        CreateChatInviteLink,
        CreateChatSubscriptionInviteLink,
        EditChatInviteLink,
        EditChatSubscriptionInviteLink,
        ExportChatInviteLink,
        RevokeChatInviteLink,
        User,
        tests::assert_json_eq,
    },
};

#[test]
fn chat_invite_link() {
    assert_json_eq(
        ChatInviteLink::new("example.com/join/chat", User::new(1, "User", false))
            .with_creates_join_request(true)
            .with_is_primary(true)
            .with_is_revoked(false)
            .with_name("Link")
            .with_expire_date(0)
            .with_member_limit(10)
            .with_pending_join_request_count(0),
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
        ChatInviteLink::new("example.com/join/chat", User::new(1, "User", false)),
        serde_json::json!({
            "invite_link": "example.com/join/chat",
            "creator": {
                "id": 1,
                "is_bot": false,
                "first_name": "User"
            },
            "creates_join_request": false,
            "is_primary": false,
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
            .with_name("Link")
            .with_expire_date(0)
            .with_member_limit(1)
            .with_creates_join_request(false),
    )
}

#[test]
fn create_chat_subscription_invite_link() {
    let method = CreateChatSubscriptionInviteLink::new(1, 2592000, 1);

    assert_payload_eq(
        Payload::json(
            "createChatSubscriptionInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "subscription_period": 2592000,
                "subscription_price": 1,
            }),
        ),
        method.clone(),
    );

    assert_payload_eq(
        Payload::json(
            "createChatSubscriptionInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "subscription_period": 2592000,
                "subscription_price": 1,
                "name": "test",
            }),
        ),
        method.with_name("test"),
    );
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
            .with_name("Link")
            .with_expire_date(0)
            .with_member_limit(1)
            .with_creates_join_request(false),
    );
}

#[test]
fn edit_chat_subscription_invite_link() {
    let method = EditChatSubscriptionInviteLink::new(1, "test");

    assert_payload_eq(
        Payload::json(
            "editChatSubscriptionInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "test",
            }),
        ),
        method.clone(),
    );

    assert_payload_eq(
        Payload::json(
            "editChatSubscriptionInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "test",
                "name": "test",
            }),
        ),
        method.with_name("test"),
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

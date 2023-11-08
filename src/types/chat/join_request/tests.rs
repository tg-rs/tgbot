use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        ApproveChatJoinRequest,
        ChannelChat,
        Chat,
        ChatInviteLink,
        ChatJoinRequest,
        DeclineChatJoinRequest,
        User,
    },
};

#[test]
fn chat_join_request() {
    assert_json_eq(
        ChatJoinRequest::new(
            Chat::Channel(ChannelChat::new(1, "Channel")),
            0,
            User::new(1, "User", false),
        )
        .with_bio("Bio")
        .with_invite_link(
            ChatInviteLink::new("example.com/join/channel", User::new(2, "User", false)).with_is_primary(true),
        )
        .with_user_chat_id(1),
        serde_json::json!({
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "Channel"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "User"
            },
            "date": 0,
            "bio": "Bio",
            "invite_link": {
                "invite_link": "example.com/join/channel",
                "creator": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "User"
                },
                "creates_join_request": false,
                "is_primary": true,
                "is_revoked": false
            },
            "user_chat_id": 1,
        }),
    );
    assert_json_eq(
        ChatJoinRequest::new(
            Chat::Channel(ChannelChat::new(1, "Channel")),
            0,
            User::new(1, "User", false),
        ),
        serde_json::json!({
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "Channel"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "User"
            },
            "date": 0,
        }),
    );
}

#[test]
fn approve_chat_join_request() {
    assert_payload_eq(
        Payload::json(
            "approveChatJoinRequest",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
            }),
        ),
        ApproveChatJoinRequest::new(1, 1),
    );
}

#[test]
fn decline_chat_join_request() {
    assert_payload_eq(
        Payload::json(
            "declineChatJoinRequest",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
            }),
        ),
        DeclineChatJoinRequest::new(1, 1),
    );
}

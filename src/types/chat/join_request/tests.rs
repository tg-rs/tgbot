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
        ChatJoinRequest {
            chat: Chat::Channel(ChannelChat {
                id: 1,
                title: String::from("Channel"),
                username: None,
                photo: None,
                description: None,
                invite_link: None,
                pinned_message: None,
                linked_chat_id: None,
                has_protected_content: None,
                message_auto_delete_time: None,
            }),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("User"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            date: 0,
            bio: Some(String::from("Bio")),
            invite_link: Some(ChatInviteLink {
                invite_link: String::from("example.com/join/channel"),
                creator: User {
                    id: 2,
                    is_bot: false,
                    first_name: String::from("User"),
                    last_name: None,
                    username: None,
                    language_code: None,
                    is_premium: None,
                    added_to_attachment_menu: None,
                },
                creates_join_request: false,
                is_primary: true,
                is_revoked: false,
                name: None,
                expire_date: None,
                member_limit: None,
                pending_join_request_count: None,
            }),
        },
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
            }
        }),
    );
    assert_json_eq(
        ChatJoinRequest {
            chat: Chat::Channel(ChannelChat {
                id: 1,
                title: String::from("Channel"),
                username: None,
                photo: None,
                description: None,
                invite_link: None,
                pinned_message: None,
                linked_chat_id: None,
                has_protected_content: None,
                message_auto_delete_time: None,
            }),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("User"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            date: 0,
            bio: None,
            invite_link: None,
        },
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

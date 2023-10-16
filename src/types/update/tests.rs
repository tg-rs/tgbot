use std::{collections::HashSet, time::Duration};

use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        AllowedUpdate,
        CallbackQuery,
        ChannelChat,
        Chat,
        ChatJoinRequest,
        ChatMember,
        ChatMemberKicked,
        ChatMemberUpdated,
        ChosenInlineResult,
        GetUpdates,
        GroupChat,
        InlineQuery,
        Message,
        MessageData,
        MessageSender,
        Poll,
        PollAnswer,
        PollOption,
        PreCheckoutQuery,
        PrivateChat,
        RegularPoll,
        ShippingAddress,
        ShippingQuery,
        Text,
        Update,
        UpdateKind,
        User,
    },
};

#[test]
fn allowed_update() {
    use crate::types::AllowedUpdate::*;
    for (expected_struct, expected_value) in [
        (Message, serde_json::json!("message")),
        (EditedMessage, serde_json::json!("edited_message")),
        (ChannelPost, serde_json::json!("channel_post")),
        (EditedChannelPost, serde_json::json!("edited_channel_post")),
        (InlineQuery, serde_json::json!("inline_query")),
        (ChosenInlineResult, serde_json::json!("chosen_inline_result")),
        (CallbackQuery, serde_json::json!("callback_query")),
        (ShippingQuery, serde_json::json!("shipping_query")),
        (PreCheckoutQuery, serde_json::json!("pre_checkout_query")),
        (Poll, serde_json::json!("poll")),
        (PollAnswer, serde_json::json!("poll_answer")),
        (ChatMember, serde_json::json!("chat_member")),
    ] {
        assert_json_eq(expected_struct, expected_value);
    }
}

#[test]
fn bot_status() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::BotStatus(ChatMemberUpdated {
            chat: Chat::Group(GroupChat {
                id: 1,
                title: String::from("Group"),
                photo: None,
                invite_link: None,
                pinned_message: None,
                permissions: None,
                has_protected_content: None,
                message_auto_delete_time: None,
            }),
            from: User {
                id: 1,
                is_bot: true,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            date: 0,
            old_chat_member: ChatMember::Member(User {
                id: 2,
                is_bot: true,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            }),
            new_chat_member: ChatMember::Kicked(ChatMemberKicked {
                user: User {
                    id: 2,
                    is_bot: true,
                    first_name: String::from("John"),
                    last_name: None,
                    username: None,
                    language_code: None,
                    is_premium: None,
                    added_to_attachment_menu: None,
                },
                until_date: 0,
            }),
            invite_link: None,
        }),
    };
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "my_chat_member": {
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "Group"
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
                        "is_bot": true,
                        "first_name": "John"
                    }
                },
                "new_chat_member": {
                    "status": "kicked",
                    "user": {
                        "id": 2,
                        "is_bot": true,
                        "first_name": "John",
                    },
                    "until_date": 0
                }
            }
        }),
    );
}

#[test]
fn callback_query() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::CallbackQuery(CallbackQuery {
            id: String::from("query-id"),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            message: None,
            inline_message_id: None,
            chat_instance: None,
            data: None,
            game_short_name: None,
        }),
    };
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "callback_query": {
                "id": "query-id",
                "from": {
                    "id": 1,
                    "first_name": "John",
                    "is_bot": false
                }
            }
        }),
    );
}

#[test]
fn channel_post() {
    let chat = Chat::Channel(ChannelChat {
        id: 1,
        title: String::from("Channel"),
        username: Some(String::from("channel_username")),
        photo: None,
        description: None,
        invite_link: None,
        pinned_message: None,
        linked_chat_id: None,
        has_protected_content: None,
        message_auto_delete_time: None,
        active_usernames: None,
    });
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::ChannelPost(Message {
            id: 1111,
            date: 0,
            edit_date: None,
            sender: MessageSender::Chat(chat.clone()),
            chat,
            author_signature: Some(String::from("John D.")),
            has_protected_content: false,
            forward: None,
            is_automatic_forward: false,
            is_topic_message: None,
            message_thread_id: None,
            reply_to: None,
            via_bot: None,
            media_group_id: None,
            reply_markup: None,
            data: MessageData::Text(Text {
                data: String::from("test message from channel"),
                entities: None,
            }),
        }),
    };
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "channel_username");
    assert!(expected_struct.get_user().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "channel_post": {
                "message_id": 1111,
                "date": 0,
                "author_signature": "John D.",
                "chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "Channel",
                    "username": "channel_username"
                },
                "sender_chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "Channel",
                    "username": "channel_username"
                },
                "text": "test message from channel",
                "has_protected_content": false,
                "is_automatic_forward": false
            }
        }),
    );
}

#[test]
fn chat_join_request() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::ChatJoinRequest(ChatJoinRequest {
            chat: Chat::Group(GroupChat {
                id: 1,
                title: String::from("Group"),
                photo: None,
                invite_link: None,
                pinned_message: None,
                permissions: None,
                has_protected_content: None,
                message_auto_delete_time: None,
            }),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            date: 0,
            bio: None,
            invite_link: None,
        }),
    };
    assert_eq!(expected_struct.get_chat_id(), Some(1));
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "chat_join_request": {
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "Group"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "John"
                },
                "date": 0
            }
        }),
    );
}

#[test]
fn chosen_inline_result() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::ChosenInlineResult(ChosenInlineResult {
            result_id: String::from("chosen-inline-result-id"),
            from: User {
                id: 1111,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            location: None,
            inline_message_id: None,
            query: String::from("q"),
        }),
    };
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1111);
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "chosen_inline_result": {
                "result_id": "chosen-inline-result-id",
                "from": {
                    "id": 1111,
                    "first_name": "John",
                    "is_bot": false
                },
                "query": "q",
            }
        }),
    );
}

#[test]
fn edited_channel_post() {
    let chat = Chat::Channel(ChannelChat {
        id: 1,
        title: String::from("Channel"),
        username: Some(String::from("channel_username")),
        photo: None,
        description: None,
        invite_link: None,
        pinned_message: None,
        linked_chat_id: None,
        has_protected_content: None,
        message_auto_delete_time: None,
        active_usernames: None,
    });
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::EditedChannelPost(Message {
            id: 1111,
            date: 0,
            edit_date: None,
            sender: MessageSender::Chat(chat.clone()),
            chat,
            author_signature: Some(String::from("John D.")),
            has_protected_content: false,
            forward: None,
            is_automatic_forward: false,
            is_topic_message: None,
            message_thread_id: None,
            reply_to: None,
            via_bot: None,
            media_group_id: None,
            reply_markup: None,
            data: MessageData::Text(Text {
                data: String::from("test message from channel"),
                entities: None,
            }),
        }),
    };
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "channel_username");
    assert!(expected_struct.get_user().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "edited_channel_post": {
                "message_id": 1111,
                "date": 0,
                "author_signature": "John D.",
                "chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "Channel",
                    "username": "channel_username"
                },
                "sender_chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "Channel",
                    "username": "channel_username"
                },
                "text": "test message from channel",
                "has_protected_content": false,
                "is_automatic_forward": false
            }
        }),
    );
}

#[test]
fn edited_message() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::EditedMessage(Message {
            author_signature: None,
            chat: Chat::Private(PrivateChat {
                bio: None,
                first_name: String::from("John"),
                has_private_forwards: None,
                id: 1111,
                last_name: Some(String::from("Doe")),
                message_auto_delete_time: None,
                photo: None,
                pinned_message: None,
                username: Some(String::from("john_doe")),
                has_restricted_voice_and_video_messages: None,
                active_usernames: None,
            }),
            data: MessageData::Text(Text {
                data: String::from("Edited text"),
                entities: None,
            }),
            date: 1441,
            edit_date: Some(1441),
            forward: None,
            has_protected_content: false,
            id: 1365,
            is_automatic_forward: false,
            is_topic_message: None,
            media_group_id: None,
            reply_markup: None,
            reply_to: None,
            sender: MessageSender::User(User {
                first_name: String::from("John"),
                id: 1111,
                is_bot: false,
                language_code: None,
                last_name: Some(String::from("Doe")),
                username: Some(String::from("john_doe")),
                is_premium: None,
                added_to_attachment_menu: None,
            }),
            via_bot: None,
            message_thread_id: None,
        }),
    };
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "john_doe");
    assert_eq!(expected_struct.get_user_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_user_username().unwrap(), "john_doe");

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "edited_message": {
                "chat": {
                    "id": 1111,
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "john_doe",
                    "type": "private",
                },
                "date": 1441,
                "edit_date": 1441,
                "from": {
                    "id": 1111,
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "john_doe",
                    "is_bot": false
                },
                "has_protected_content": false,
                "is_automatic_forward": false,
                "message_id": 1365,
                "text": "Edited text",
            }
        }),
    );
}

#[test]
fn inline_query() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::InlineQuery(InlineQuery {
            id: String::from("query-id"),
            from: User {
                id: 1111,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            query: String::from("query query"),
            offset: String::from("query offset"),
            chat_type: None,
            location: None,
        }),
    };
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user().map(|u| u.id).unwrap(), 1111);

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "inline_query": {
                "id": "query-id",
                "from": {
                    "id": 1111,
                    "first_name": "John",
                    "is_bot": false
                },
                "query": "query query",
                "offset": "query offset"
            }
        }),
    );
}

#[test]
fn message() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::Message(Message {
            id: 1,
            date: 0,
            edit_date: None,
            sender: MessageSender::User(User {
                id: 1,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            }),
            chat: Chat::Private(PrivateChat {
                id: 1,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                photo: None,
                bio: None,
                pinned_message: None,
                has_private_forwards: None,
                message_auto_delete_time: None,
                has_restricted_voice_and_video_messages: None,
                active_usernames: None,
            }),
            author_signature: None,
            has_protected_content: false,
            forward: None,
            is_automatic_forward: false,
            is_topic_message: None,
            message_thread_id: None,
            reply_to: None,
            via_bot: None,
            media_group_id: None,
            reply_markup: None,
            data: MessageData::Text(Text {
                data: String::from("message-text"),
                entities: None,
            }),
        }),
    };
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());
    assert_eq!(expected_struct.get_user().map(|u| u.id).unwrap(), 1);

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "John"
                },
                "chat": {
                    "id": 1,
                    "type": "private",
                    "first_name": "John"
                },
                "text": "message-text",
                "has_protected_content": false,
                "is_automatic_forward": false
            }
        }),
    );
}

#[test]
fn poll() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::Poll(Poll::Regular(RegularPoll {
            id: String::from("poll-id"),
            question: String::from("Rust?"),
            options: vec![
                PollOption {
                    text: String::from("Yes"),
                    voter_count: 1000,
                },
                PollOption {
                    text: String::from("No"),
                    voter_count: 0,
                },
            ],
            total_voter_count: 1000,
            is_closed: true,
            is_anonymous: true,
            allows_multiple_answers: false,
            open_period: None,
            close_date: None,
        })),
    };
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "poll": {
                "id": "poll-id",
                "question": "Rust?",
                "options": [
                    {"text": "Yes", "voter_count": 1000},
                    {"text": "No", "voter_count": 0}
                ],
                "is_closed": true,
                "total_voter_count": 1000,
                "is_anonymous": true,
                "type": "regular",
                "allows_multiple_answers": false
            }
        }),
    );
}

#[test]
fn poll_answer() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::PollAnswer(PollAnswer {
            poll_id: String::from("poll-id"),
            user: User {
                id: 1,
                is_bot: false,
                first_name: String::from("Jamie"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            option_ids: vec![0],
        }),
    };
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "poll_answer": {
                "poll_id": "poll-id",
                "user": {
                    "id": 1,
                    "first_name": "Jamie",
                    "is_bot": false
                },
                "option_ids": [0],
            }
        }),
    );
}

#[test]
fn pre_checkout_query() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::PreCheckoutQuery(PreCheckoutQuery {
            id: String::from("query-id"),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            currency: String::from("GEL"),
            total_amount: 100,
            invoice_payload: String::from("invoice payload"),
            shipping_option_id: None,
            order_info: None,
        }),
    };
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "pre_checkout_query": {
                "id": "query-id",
                "from": {
                    "id": 1,
                    "first_name": "John",
                    "is_bot": false
                },
                "currency": "GEL",
                "total_amount": 100,
                "invoice_payload": "invoice payload"
            }
        }),
    );
}

#[test]
fn shipping_query() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::ShippingQuery(ShippingQuery {
            id: String::from("query-id"),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("Ramazan"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            invoice_payload: String::from("payload"),
            shipping_address: ShippingAddress {
                country_code: String::from("RU"),
                state: String::from("Chechen Republic"),
                city: String::from("Gudermes"),
                street_line1: String::from("Nuradilov st., 12"),
                street_line2: String::from(""),
                post_code: String::from("366200"),
            },
        }),
    };
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "shipping_query": {
                "id": "query-id",
                "from": {
                    "id": 1,
                    "first_name": "Ramazan",
                    "is_bot": false
                },
                "invoice_payload": "payload",
                "shipping_address": {
                    "country_code": "RU",
                    "state": "Chechen Republic",
                    "city": "Gudermes",
                    "street_line1": "Nuradilov st., 12",
                    "street_line2": "",
                    "post_code": "366200",
                }
            }
        }),
    );
}

#[test]
fn unknown() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::Unknown(serde_json::json!({"key": "value"})),
    };
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "unknown": {
                "key": "value"
            }
        }),
    );
}

#[test]
fn user_status() {
    let expected_struct = Update {
        id: 1,
        kind: UpdateKind::UserStatus(ChatMemberUpdated {
            chat: Chat::Group(GroupChat {
                id: 1,
                title: String::from("Group"),
                photo: None,
                invite_link: None,
                pinned_message: None,
                permissions: None,
                has_protected_content: None,
                message_auto_delete_time: None,
            }),
            from: User {
                id: 1,
                is_bot: true,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            },
            date: 0,
            old_chat_member: ChatMember::Member(User {
                id: 2,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            }),
            new_chat_member: ChatMember::Kicked(ChatMemberKicked {
                user: User {
                    id: 2,
                    is_bot: false,
                    first_name: String::from("John"),
                    last_name: None,
                    username: None,
                    language_code: None,
                    is_premium: None,
                    added_to_attachment_menu: None,
                },
                until_date: 0,
            }),
            invite_link: None,
        }),
    };
    assert_eq!(expected_struct.get_chat_id(), Some(1));
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "chat_member": {
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "Group"
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
            }
        }),
    );
}

#[test]
fn get_updates() {
    assert_payload_eq(
        Payload::json("getUpdates", serde_json::json!({})),
        GetUpdates::default(),
    );

    let mut updates = HashSet::new();
    updates.insert(AllowedUpdate::Message);
    assert_payload_eq(
        Payload::json(
            "getUpdates",
            serde_json::json!({
                "offset": 0,
                "limit": 10,
                "timeout": 10,
                "allowed_updates": ["message"]
            }),
        ),
        GetUpdates::default()
            .offset(0)
            .limit(10)
            .timeout(Duration::from_secs(10))
            .allowed_updates(updates)
            .add_allowed_update(AllowedUpdate::Message),
    );
}

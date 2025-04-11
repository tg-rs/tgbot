use std::{collections::HashSet, time::Duration};

use crate::{
    api::{Payload, assert_payload_eq},
    types::{
        AllowedUpdate,
        BusinessConnection,
        BusinessMessagesDeleted,
        CallbackQuery,
        ChannelChat,
        Chat,
        ChatBoost,
        ChatBoostRemoved,
        ChatBoostSource,
        ChatBoostUpdated,
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
        MessageReactionCountUpdated,
        MessageReactionUpdated,
        PaidMediaPurchased,
        Poll,
        PollAnswer,
        PollAnswerVoter,
        PollOption,
        PreCheckoutQuery,
        PrivateChat,
        ReactionCount,
        ReactionType,
        RegularPoll,
        ShippingAddress,
        ShippingQuery,
        Text,
        Update,
        UpdateType,
        User,
        tests::assert_json_eq,
    },
};

#[test]
fn allowed_update() {
    use crate::types::AllowedUpdate::*;
    for (expected_struct, expected_value) in [
        (BusinessConnection, serde_json::json!("business_connection")),
        (CallbackQuery, serde_json::json!("callback_query")),
        (ChannelPost, serde_json::json!("channel_post")),
        (ChatBoostRemoved, serde_json::json!("removed_chat_boost")),
        (ChatBoostUpdated, serde_json::json!("chat_boost")),
        (ChosenInlineResult, serde_json::json!("chosen_inline_result")),
        (EditedChannelPost, serde_json::json!("edited_channel_post")),
        (EditedMessage, serde_json::json!("edited_message")),
        (InlineQuery, serde_json::json!("inline_query")),
        (Message, serde_json::json!("message")),
        (MessageReaction, serde_json::json!("message_reaction")),
        (Poll, serde_json::json!("poll")),
        (PollAnswer, serde_json::json!("poll_answer")),
        (PreCheckoutQuery, serde_json::json!("pre_checkout_query")),
        (PurchasedPaidMedia, serde_json::json!("purchased_paid_media")),
        (ShippingQuery, serde_json::json!("shipping_query")),
        (UserStatus, serde_json::json!("chat_member")),
    ] {
        assert_json_eq(expected_struct, expected_value);
    }
}

#[test]
fn bot_status() {
    let expected_struct = Update::new(
        1,
        UpdateType::BotStatus(ChatMemberUpdated::new(
            Chat::Group(GroupChat::new(1, "Group")),
            0,
            User::new(1, "John", false),
            ChatMember::Kicked(ChatMemberKicked::new(0, User::new(2, "John", true))),
            ChatMember::Member {
                user: User::new(2, "John", true),
                until_date: None,
            },
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ChatMemberUpdated::try_from(expected_struct.clone()).is_ok());

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
                    "is_bot": false,
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
fn business_connection() {
    let expected_struct = Update::new(
        1,
        UpdateType::BusinessConnection(BusinessConnection::new(0, "id", User::new(1, "John", false), 2)),
    );

    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(BusinessConnection::try_from(expected_struct.clone()).is_ok());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "business_connection": {
                "date": 0,
                "id": "id",
                "is_enabled": false,
                "user": {
                    "id": 1,
                    "first_name": "John",
                    "is_bot": false
                },
                "user_chat_id": 2
            }
        }),
    );
}

#[test]
fn business_message() {
    let expected_struct = Update::new(
        1,
        UpdateType::BusinessMessage(Message::new(
            1,
            0,
            PrivateChat::new(1, "John"),
            MessageData::Text(Text::from("message-text")),
            User::new(1, "John", false),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());
    assert_eq!(expected_struct.get_user().map(|u| u.id).unwrap(), 1);

    assert!(Message::try_from(expected_struct.clone()).is_ok());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "business_message": {
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
fn callback_query() {
    let expected_struct = Update::new(
        1,
        UpdateType::CallbackQuery(CallbackQuery::new("query-id", User::new(1, "John", false))),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(CallbackQuery::try_from(expected_struct.clone()).is_ok());

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
    let chat = Chat::Channel(ChannelChat::new(1, "Channel").with_username("channel_username"));
    let expected_struct = Update::new(
        1,
        UpdateType::ChannelPost(
            Message::new(
                1111,
                0,
                chat.clone(),
                MessageData::Text(Text::from("test message from channel")),
                chat,
            )
            .with_author_signature("John D."),
        ),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "channel_username");
    assert!(expected_struct.get_user().is_none());

    assert!(Message::try_from(expected_struct.clone()).is_ok());

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
fn chat_boost_removed() {
    let expected_struct = Update::new(
        1,
        UpdateType::ChatBoostRemoved(ChatBoostRemoved::new(
            "id",
            ChannelChat::new(1, "test"),
            0,
            ChatBoostSource::GiftCode(User::new(1, "test", false)),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
                "update_id": 1,
                "removed_chat_boost": {
                    "boost_id": "id",
                    "chat": {
                        "type": "channel",
                        "id": 1,
                        "title": "test"
                    },
                    "remove_date": 0,
                    "source": {
                        "source": "gift_code",
                        "user": {
                            "id": 1,
                            "first_name": "test",
                            "is_bot": false
                        }
                    },
                }
            }
        ),
    );
}

#[test]
fn chat_boost_updated() {
    let expected_struct = Update::new(
        1,
        UpdateType::ChatBoostUpdated(ChatBoostUpdated::new(
            ChatBoost::new(0, "id", 0, ChatBoostSource::GiftCode(User::new(1, "test", false))),
            ChannelChat::new(1, "test"),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "chat_boost": {
                "boost": {
                    "add_date": 0,
                    "boost_id": "id",
                    "expiration_date": 0,
                    "source": {
                        "source": "gift_code",
                        "user": {
                            "id": 1,
                            "first_name": "test",
                            "is_bot": false
                        }
                    },
                },
                "chat": {
                    "type": "channel",
                    "id": 1,
                    "title": "test"
                }
            }
        }),
    );
}

#[test]
fn chat_join_request() {
    let expected_struct = Update::new(
        1,
        UpdateType::ChatJoinRequest(ChatJoinRequest::new(
            Chat::Group(GroupChat::new(1, "Group")),
            0,
            User::new(1, "John", false),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ChatJoinRequest::try_from(expected_struct.clone()).is_ok());

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
    let expected_struct = Update::new(
        1,
        UpdateType::ChosenInlineResult(ChosenInlineResult::new(
            User::new(1, "John", false),
            "q",
            "chosen-inline-result-id",
        )),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ChosenInlineResult::try_from(expected_struct.clone()).is_ok());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "chosen_inline_result": {
                "result_id": "chosen-inline-result-id",
                "from": {
                    "id": 1,
                    "first_name": "John",
                    "is_bot": false
                },
                "query": "q",
            }
        }),
    );
}

#[test]
fn deleted_business_messages() {
    let expected_struct = Update::new(
        1,
        UpdateType::DeletedBusinessMessages(BusinessMessagesDeleted::new(
            "id",
            PrivateChat::new(1, "John").with_username("john_doe"),
            [2],
        )),
    );

    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "john_doe");
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());

    assert!(BusinessMessagesDeleted::try_from(expected_struct.clone()).is_ok());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "deleted_business_messages": {
                "business_connection_id": "id",
                "chat": {
                    "type": "private",
                    "id": 1,
                    "first_name": "John",
                    "username": "john_doe"
                },
                "message_ids": [2]
            }
        }),
    );
}

#[test]
fn edited_business_message() {
    let expected_struct = Update::new(
        1,
        UpdateType::EditedBusinessMessage(
            Message::new(
                1365,
                1441,
                PrivateChat::new(1111, "John")
                    .with_last_name("Doe")
                    .with_username("john_doe"),
                MessageData::Text(Text::from("Edited text")),
                User::new(1111, "John", false)
                    .with_last_name("Doe")
                    .with_username("john_doe"),
            )
            .with_edit_date(1441),
        ),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "john_doe");
    assert_eq!(expected_struct.get_user_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_user_username().unwrap(), "john_doe");

    assert!(Message::try_from(expected_struct.clone()).is_ok());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "edited_business_message": {
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
fn edited_channel_post() {
    let chat = Chat::Channel(ChannelChat::new(1, "Channel").with_username("channel_username"));
    let expected_struct = Update::new(
        1,
        UpdateType::EditedChannelPost(
            Message::new(
                1111,
                0,
                chat.clone(),
                MessageData::Text(Text::from("test message from channel")),
                chat,
            )
            .with_author_signature("John D."),
        ),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "channel_username");
    assert!(expected_struct.get_user().is_none());

    assert!(Message::try_from(expected_struct.clone()).is_ok());

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
    let expected_struct = Update::new(
        1,
        UpdateType::EditedMessage(
            Message::new(
                1365,
                1441,
                PrivateChat::new(1111, "John")
                    .with_last_name("Doe")
                    .with_username("john_doe"),
                MessageData::Text(Text::from("Edited text")),
                User::new(1111, "John", false)
                    .with_last_name("Doe")
                    .with_username("john_doe"),
            )
            .with_edit_date(1441),
        ),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_chat_username().unwrap(), "john_doe");
    assert_eq!(expected_struct.get_user_id().unwrap(), 1111);
    assert_eq!(expected_struct.get_user_username().unwrap(), "john_doe");

    assert!(Message::try_from(expected_struct.clone()).is_ok());

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
    let expected_struct = Update::new(
        1,
        UpdateType::InlineQuery(InlineQuery::new(
            User::new(1, "John", false),
            "query-id",
            "query offset",
            "query query",
        )),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user().map(|u| u.id).unwrap(), 1);

    assert!(InlineQuery::try_from(expected_struct.clone()).is_ok());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "inline_query": {
                "id": "query-id",
                "from": {
                    "id": 1,
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
    let expected_struct = Update::new(
        1,
        UpdateType::Message(Message::new(
            1,
            0,
            PrivateChat::new(1, "John"),
            MessageData::Text(Text::from("message-text")),
            User::new(1, "John", false),
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());
    assert_eq!(expected_struct.get_user().map(|u| u.id).unwrap(), 1);

    assert!(Message::try_from(expected_struct.clone()).is_ok());

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
fn message_reaction() {
    let expected_struct = Update::new(
        1,
        UpdateType::MessageReaction(MessageReactionUpdated::new(
            PrivateChat::new(1, "test"),
            0,
            1,
            [ReactionType::emoji("🤡")],
            [ReactionType::emoji("🤮")],
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());
    assert!(expected_struct.get_user().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "message_reaction": {
                "chat": {
                    "type": "private",
                    "id": 1,
                    "first_name": "test"
                },
                "date": 0,
                "message_id": 1,
                "old_reaction": [
                    {
                        "type": "emoji",
                        "emoji": "🤮"
                    }
                ],
                "new_reaction": [
                    {
                        "type": "emoji",
                        "emoji": "🤡"
                    }
                ]
            }
        }),
    );
}

#[test]
fn message_reaction_count() {
    let expected_struct = Update::new(
        1,
        UpdateType::MessageReactionCount(MessageReactionCountUpdated::new(
            PrivateChat::new(1, "test"),
            0,
            1,
            [ReactionCount::new(ReactionType::emoji("🤡"), 1)],
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());
    assert!(expected_struct.get_user().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "message_reaction_count": {
                "chat": {
                    "type": "private",
                    "id": 1,
                    "first_name": "test"
                },
                "date": 0,
                "message_id": 1,
                "reactions": [
                    {
                        "type": {
                            "type": "emoji",
                            "emoji": "🤡"
                        },
                        "total_count": 1
                    }
                ]
            }
        }),
    );
}

#[test]
fn poll() {
    let expected_struct = Update::new(
        1,
        UpdateType::Poll(
            RegularPoll::new("poll-id", "Rust?")
                .with_allows_multiple_answers(false)
                .with_is_closed(true)
                .with_is_anonymous(true)
                .with_options([PollOption::new("Yes", 1000), PollOption::new("No", 0)])
                .with_total_voter_count(1000)
                .into(),
        ),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user().is_none());

    assert!(Poll::try_from(expected_struct.clone()).is_ok());

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
    let expected_struct = Update::new(
        1,
        UpdateType::PollAnswer(PollAnswer {
            poll_id: String::from("poll-id"),
            voter: PollAnswerVoter::User(User::new(1, "John", false)),
            option_ids: vec![0],
        }),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(PollAnswer::try_from(expected_struct.clone()).is_ok());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "poll_answer": {
                "poll_id": "poll-id",
                "user": {
                    "id": 1,
                    "first_name": "John",
                    "is_bot": false
                },
                "option_ids": [0],
            }
        }),
    );
}

#[test]
fn pre_checkout_query() {
    let expected_struct = Update::new(
        1,
        UpdateType::PreCheckoutQuery(PreCheckoutQuery::new(
            "GEL",
            User::new(1, "John", false),
            "query-id",
            "invoice payload",
            100,
        )),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(PreCheckoutQuery::try_from(expected_struct.clone()).is_ok());

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
fn purchased_paid_media() {
    let expected_struct = Update::new(
        1,
        UpdateType::PurchasedPaidMedia(PaidMediaPurchased::new(User::new(1, "John", false), "payload")),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(PaidMediaPurchased::try_from(expected_struct.clone()).is_ok());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "purchased_paid_media": {
                "from": {
                    "id": 1,
                    "first_name": "John",
                    "is_bot": false,
                },
                "paid_media_payload": "payload"
            }
        }),
    )
}

#[test]
fn shipping_query() {
    let expected_struct = Update::new(
        1,
        UpdateType::ShippingQuery(ShippingQuery::new(
            "query-id",
            User::new(1, "Ramazan", false),
            "payload",
            ShippingAddress::new("Gudermes", "RU", "366200", "Chechen Republic", "Nuradilov st., 12", ""),
        )),
    );
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ShippingQuery::try_from(expected_struct.clone()).is_ok());

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
    let expected_struct = Update::new(1, UpdateType::Unknown(serde_json::json!({"key": "value"})));
    assert!(expected_struct.get_chat_id().is_none());
    assert!(expected_struct.get_chat_username().is_none());
    assert!(expected_struct.get_user_id().is_none());
    assert!(expected_struct.get_user_username().is_none());

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "update_id": 1,
            "key": "value",
        }),
    );
}

#[test]
fn user_status() {
    let expected_struct = Update::new(
        1,
        UpdateType::UserStatus(ChatMemberUpdated::new(
            Chat::Group(GroupChat::new(1, "Group")),
            0,
            User::new(1, "John", false),
            ChatMember::Kicked(ChatMemberKicked::new(0, User::new(2, "John", false))),
            ChatMember::Member {
                user: User::new(2, "John", false),
                until_date: None,
            },
        )),
    );
    assert_eq!(expected_struct.get_chat_id().unwrap(), 1);
    assert!(expected_struct.get_chat_username().is_none());
    assert_eq!(expected_struct.get_user_id().unwrap(), 1);
    assert!(expected_struct.get_user_username().is_none());

    assert!(ChatMemberUpdated::try_from(expected_struct.clone()).is_ok());

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
                    "is_bot": false,
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
            .with_offset(0)
            .with_limit(10)
            .with_timeout(Duration::from_secs(10))
            .with_allowed_updates(updates)
            .add_allowed_update(AllowedUpdate::Message),
    );
}

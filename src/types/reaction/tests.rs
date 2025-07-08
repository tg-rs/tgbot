use crate::{
    api::{Payload, assert_payload_eq},
    types::{
        ChannelChat,
        MessageReactionCountUpdated,
        MessageReactionUpdated,
        PrivateChat,
        ReactionCount,
        ReactionType,
        SetMessageReaction,
        User,
        tests::assert_json_eq,
    },
};

#[test]
fn reaction_count() {
    assert_json_eq(
        ReactionCount::new(ReactionType::emoji("🤡"), 1),
        serde_json::json!({
            "type": {
                "type": "emoji",
                "emoji": "🤡"
            },
            "total_count": 1
        }),
    );
}

#[test]
fn reaction_type() {
    assert_json_eq(
        ReactionType::custom_emoji("5420319826440644296"),
        serde_json::json!({
            "type": "custom_emoji",
            "custom_emoji_id": "5420319826440644296"
        }),
    );
    assert_json_eq(
        ReactionType::emoji("🤡"),
        serde_json::json!({
            "type": "emoji",
            "emoji": "🤡"
        }),
    );
    assert_json_eq(
        ReactionType::Paid,
        serde_json::json!({
            "type": "paid",
        }),
    );
}

#[test]
fn message_reaction_count_updated() {
    assert_json_eq(
        MessageReactionCountUpdated::new(
            PrivateChat::new(1, "test"),
            0,
            1,
            [ReactionCount::new(ReactionType::emoji("🤡"), 1)],
        ),
        serde_json::json!({
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
        }),
    );
}

#[test]
fn message_reaction_updated() {
    let expected_struct = MessageReactionUpdated::new(
        PrivateChat::new(1, "test"),
        0,
        1,
        [ReactionType::emoji("🤡")],
        [ReactionType::emoji("🤮")],
    );
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
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
        }),
    );

    assert_json_eq(
        expected_struct.clone().with_actor_chat(ChannelChat::new(1, "test")),
        serde_json::json!({
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
            ],
            "actor_chat": {
                "type": "channel",
                "id": 1,
                "title": "test"
            }
        }),
    );

    assert_json_eq(
        expected_struct.clone().with_user(User::new(1, "test", false)),
        serde_json::json!({
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
            ],
            "user": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }),
    );
}

#[test]
fn set_message_reaction() {
    let method = SetMessageReaction::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "setMessageReaction",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "is_big": false
            }),
        ),
        method.clone(),
    );

    let method = method.with_is_big(true).with_reaction([ReactionType::emoji("🤡")]);
    assert_payload_eq(
        Payload::json(
            "setMessageReaction",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "is_big": true,
                "reaction": [
                    {
                        "type": "emoji",
                        "emoji": "🤡"
                    }
                ]
            }),
        ),
        method.clone(),
    );
}

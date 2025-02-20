use crate::{
    api::{Payload, assert_payload_eq},
    types::{
        ChannelChat,
        ChatBoost,
        ChatBoostRemoved,
        ChatBoostSource,
        ChatBoostSourceGiveaway,
        ChatBoostUpdated,
        GetUserChatBoosts,
        User,
        UserChatBoosts,
        tests::assert_json_eq,
    },
};

#[test]
fn chat_boost() {
    assert_json_eq(
        ChatBoost::new(0, "id", 0, ChatBoostSource::GiftCode(User::new(1, "test", false))),
        serde_json::json!({
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
        }),
    );
}

#[test]
fn chat_boost_removed() {
    assert_json_eq(
        ChatBoostRemoved::new(
            "id",
            ChannelChat::new(1, "test"),
            0,
            ChatBoostSource::GiftCode(User::new(1, "test", false)),
        ),
        serde_json::json!({
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
        }),
    );
}

#[test]
fn chat_boost_source() {
    assert_json_eq(
        ChatBoostSource::GiftCode(User::new(1, "test", false)),
        serde_json::json!({
            "source": "gift_code",
            "user": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }),
    );
    assert_json_eq(
        ChatBoostSource::Giveaway(ChatBoostSourceGiveaway::new(1)),
        serde_json::json!({
            "source": "giveaway",
            "giveaway_message_id": 1
        }),
    );
    assert_json_eq(
        ChatBoostSource::Giveaway(ChatBoostSourceGiveaway::new(1).with_is_unclaimed(true)),
        serde_json::json!({
            "source": "giveaway",
            "giveaway_message_id": 1,
            "is_unclaimed": true
        }),
    );
    assert_json_eq(
        ChatBoostSource::Giveaway(
            ChatBoostSourceGiveaway::new(1)
                .with_prize_star_count(2)
                .with_user(User::new(1, "test", false)),
        ),
        serde_json::json!({
            "source": "giveaway",
            "giveaway_message_id": 1,
            "prize_star_count": 2,
            "user": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }),
    );
    assert_json_eq(
        ChatBoostSource::Premium(User::new(1, "test", false)),
        serde_json::json!({
            "source": "premium",
            "user": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }),
    );
}

#[test]
fn chat_boost_updated() {
    assert_json_eq(
        ChatBoostUpdated::new(
            ChatBoost::new(0, "id", 0, ChatBoostSource::GiftCode(User::new(1, "test", false))),
            ChannelChat::new(1, "test"),
        ),
        serde_json::json!({
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
        }),
    );
}

#[test]
fn user_chat_boosts() {
    assert_json_eq(
        UserChatBoosts::from([ChatBoost::new(
            0,
            "id",
            0,
            ChatBoostSource::GiftCode(User::new(1, "test", false)),
        )]),
        serde_json::json!({
            "boosts": [
                {
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
                }
            ]
        }),
    );
}

#[test]
fn get_user_chat_boosts() {
    assert_payload_eq(
        Payload::json(
            "getUserChatBoosts",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
            }),
        ),
        GetUserChatBoosts::new(1, 2),
    );
}

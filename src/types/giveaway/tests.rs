use crate::types::{
    ChannelChat,
    Giveaway,
    GiveawayCompleted,
    GiveawayCreated,
    GiveawayWinners,
    Message,
    MessageData,
    User,
    tests::assert_json_eq,
};

#[test]
fn giveaway() {
    let expected_struct = Giveaway::new([ChannelChat::new(1, "test")], 0, 1);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "chats": [
                {
                    "type": "channel",
                    "id": 1,
                    "title": "test"
                }
            ],
            "winners_selection_date": 0,
            "winner_count": 1
        }),
    );
    assert_json_eq(
        expected_struct
            .with_country_codes(["FR", "GE", "RU", "SK"])
            .with_has_public_winners(true)
            .with_only_new_members(true)
            .with_premium_subscription_month_count(1)
            .with_prize_description("test")
            .with_prize_star_count(2),
        serde_json::json!({
            "chats": [
                {
                    "type": "channel",
                    "id": 1,
                    "title": "test"
                }
            ],
            "winners_selection_date": 0,
            "winner_count": 1,
            "country_codes": ["FR", "GE", "RU", "SK"],
            "has_public_winners": true,
            "only_new_members": true,
            "premium_subscription_month_count": 1,
            "prize_description": "test",
            "prize_star_count": 2,
        }),
    )
}

#[test]
fn giveaway_created() {
    let expected_struct = GiveawayCreated::default();
    assert_json_eq(expected_struct.clone(), serde_json::json!({}));
    let expected_struct = expected_struct.with_prize_star_count(1);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "prize_star_count": 1,
        }),
    );
}

#[test]
fn giveaway_completed() {
    let expected_struct = GiveawayCompleted::new(1);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "winner_count": 1
        }),
    );
    let chat = ChannelChat::new(1, "test");
    assert_json_eq(
        expected_struct
            .with_giveaway_message(Message::new(
                1,
                0,
                chat.clone(),
                MessageData::Giveaway(Giveaway::new([chat], 0, 1)),
                User::new(1, "test", false),
            ))
            .with_is_star_giveaway(true)
            .with_unclaimed_prize_count(1),
        serde_json::json!({
            "winner_count": 1,
            "giveaway_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "type": "channel",
                    "id": 1,
                    "title": "test"
                },
                "giveaway": {
                    "chats": [
                        {
                            "type": "channel",
                            "id": 1,
                            "title": "test"
                        }
                    ],
                    "winners_selection_date": 0,
                    "winner_count": 1
                },
                "from": {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                },
                "has_protected_content": false,
                "is_automatic_forward": false
            },
            "is_star_giveaway": true,
            "unclaimed_prize_count": 1
        }),
    );
}

#[test]
fn giveaway_winners() {
    let expected_struct = GiveawayWinners::new(ChannelChat::new(1, "test"), 1, 1, [User::new(1, "test", false)], 0);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "chat": {
                "type": "channel",
                "id": 1,
                "title": "test"
            },
            "giveaway_message_id": 1,
            "winner_count": 1,
            "winners": [
                {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                }
            ],
            "winners_selection_date": 0
        }),
    );
    assert_json_eq(
        expected_struct
            .with_additional_chat_count(1)
            .with_only_new_members(true)
            .with_premium_subscription_month_count(2)
            .with_prize_description("test")
            .with_prize_star_count(3)
            .with_unclaimed_prize_count(1)
            .with_was_refunded(false),
        serde_json::json!({
            "chat": {
                "type": "channel",
                "id": 1,
                "title": "test"
            },
            "giveaway_message_id": 1,
            "winner_count": 1,
            "winners": [
                {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                }
            ],
            "winners_selection_date": 0,
            "additional_chat_count": 1,
            "only_new_members": true,
            "premium_subscription_month_count": 2,
            "prize_description": "test",
            "prize_star_count": 3,
            "unclaimed_prize_count": 1,
            "was_refunded": false,
        }),
    );
}

use crate::types::*;

#[test]
fn giveaway() {
    let expected_struct = Giveaway::new([ChannelChat::new(1, "test")], 0, 1);
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_country_codes(["FR", "GE", "RU", "SK"])
            .with_has_public_winners(true)
            .with_only_new_members(true)
            .with_premium_subscription_month_count(1)
            .with_prize_description("test")
            .with_prize_star_count(2)
    )
}

#[test]
fn giveaway_created() {
    let expected_struct = GiveawayCreated::default();
    insta::assert_json_snapshot!(expected_struct.clone());
    let expected_struct = expected_struct.with_prize_star_count(1);
    insta::assert_json_snapshot!(expected_struct.clone());
}

#[test]
fn giveaway_completed() {
    let expected_struct = GiveawayCompleted::new(1);
    insta::assert_json_snapshot!(expected_struct.clone());
    let chat = ChannelChat::new(1, "test");
    insta::assert_json_snapshot!(
        expected_struct
            .with_giveaway_message(Message::new(
                1,
                0,
                chat.clone(),
                MessageData::Giveaway(Giveaway::new([chat], 0, 1)),
                User::new(1, "test", false),
            ))
            .with_is_star_giveaway(true)
            .with_unclaimed_prize_count(1)
    );
}

#[test]
fn giveaway_winners() {
    let expected_struct = GiveawayWinners::new(ChannelChat::new(1, "test"), 1, 1, [User::new(1, "test", false)], 0);
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_additional_chat_count(1)
            .with_only_new_members(true)
            .with_premium_subscription_month_count(2)
            .with_prize_description("test")
            .with_prize_star_count(3)
            .with_unclaimed_prize_count(1)
            .with_was_refunded(false)
    );
}

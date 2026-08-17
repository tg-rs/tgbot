#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("send-gift-for-chat-id", SendGift::for_chat_id(1, "test"), |x| {
            assert!(x)
        }),
        (
            "send-gift-for-chat-username",
            SendGift::for_chat_id("@chat", "test"),
            |x| assert!(x),
        ),
        ("send-gift-for-user-id", SendGift::for_user_id(1, "test"), |x| {
            assert!(x)
        }),
        (
            "send-gift-full",
            SendGift::for_user_id(1, "test")
                .with_pay_for_upgrade(true)
                .with_text(("test", ParseMode::Markdown)),
            |x| assert!(x),
        ),
        (
            "send-gift-text-entities",
            SendGift::for_user_id(1, "test")
                .with_text(InputText::from(("test", ParseMode::Markdown)).with_format([TextEntity::bold(0..2)])),
            |x| assert!(x),
        ),
        (
            "send-gift-text-parse-mode",
            SendGift::for_user_id(1, "test")
                .with_text(InputText::from(("test", [TextEntity::bold(0..2)])).with_format(ParseMode::Markdown)),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([("get-available-gifts", GetAvailableGifts, |x| {
        let x: Vec<Gift> = x.into();
        assert_eq!(x.len(), 2);
        let base = &x[0];
        assert_eq!(base.id, "gift-1");
        assert_eq!(base.sticker.file_id, "fid");
        assert_eq!(base.star_count, 10);
        assert!(base.background.is_none());
        assert!(base.has_colors.is_none());
        assert!(base.is_premium.is_none());
        assert!(base.personal_remaining_count.is_none());
        assert!(base.personal_total_count.is_none());
        assert!(base.publisher_chat.is_none());
        assert!(base.remaining_count.is_none());
        assert!(base.total_count.is_none());
        assert!(base.unique_gift_variant_count.is_none());
        assert!(base.upgrade_star_count.is_none());

        let all = &x[1];
        assert_eq!(all.id, "gift-2");
        assert_eq!(all.sticker.file_id, "fid");
        assert_eq!(all.star_count, 10);
        let bg = all.background.unwrap();
        assert_eq!(bg.center_color, 0);
        assert_eq!(bg.edge_color, 1);
        assert_eq!(bg.text_color, 2);
        assert!(all.has_colors.unwrap());
        assert!(all.is_premium.unwrap());
        assert_eq!(all.personal_remaining_count.unwrap(), 1);
        assert_eq!(all.personal_total_count.unwrap(), 1);
        assert!(all.publisher_chat.is_some());
        assert_eq!(all.remaining_count.unwrap(), 2);
        assert_eq!(all.total_count.unwrap(), 4);
        assert_eq!(all.unique_gift_variant_count.unwrap(), 0);
        assert_eq!(all.upgrade_star_count.unwrap(), 10);
    })])
    .await;
    cx.execute_batch([
        (
            "get-business-account-gifts-base",
            GetBusinessAccountGifts::new("id"),
            |x| {
                assert_eq!(x.total_count, 2);
                assert!(x.next_offset.is_none());
                let data = x.gifts;
                let OwnedGift::Regular(gift) = &data[0] else {
                    panic!("Unexpected gift")
                };
                assert_eq!(gift.gift.id, "gift-1");
                let OwnedGift::Unique(unique_gift) = &data[1] else {
                    panic!("Unexpected gift")
                };
                assert_eq!(unique_gift.gift.gift_id, "gift-2");
            },
        ),
        (
            "get-business-account-gifts-all",
            GetBusinessAccountGifts::new("id")
                .with_exclude_from_blockchain(false)
                .with_exclude_limited_non_upgradable(true)
                .with_exclude_limited_upgradable(false)
                .with_exclude_saved(true)
                .with_exclude_unique(false)
                .with_exclude_unlimited(true)
                .with_exclude_unsaved(false)
                .with_limit(10)
                .with_offset("test")
                .with_sort_by_price(true),
            |x| {
                assert_eq!(x.total_count, 2);
                assert_eq!(x.next_offset.unwrap(), "test");
                let data = x.gifts;
                let OwnedGift::Regular(gift) = &data[0] else {
                    panic!("Unexpected gift")
                };
                assert_eq!(gift.gift.id, "gift-1");
                let OwnedGift::Unique(unique_gift) = &data[1] else {
                    panic!("Unexpected gift")
                };
                assert_eq!(unique_gift.gift.gift_id, "gift-2");
            },
        ),
    ])
    .await;
    cx.execute_batch([
        ("get-chat-gifts-base", GetChatGifts::new(1), |x| {
            assert_eq!(x.total_count, 2);
            assert!(x.next_offset.is_none());
            assert_eq!(x.gifts.len(), 2);
        }),
        (
            "get-chat-gifts-all",
            GetChatGifts::new(1)
                .with_exclude_from_blockchain(true)
                .with_exclude_limited_non_upgradable(false)
                .with_exclude_limited_upgradable(true)
                .with_exclude_saved(false)
                .with_exclude_unique(true)
                .with_exclude_unlimited(false)
                .with_exclude_unsaved(true)
                .with_limit(1)
                .with_offset("test")
                .with_sort_by_price(true),
            |x| {
                assert_eq!(x.total_count, 2);
                assert!(x.next_offset.is_none());
                assert_eq!(x.gifts.len(), 2);
            },
        ),
    ])
    .await;
    cx.execute_batch([
        ("get-user-gifts-base", GetUserGifts::new(1), |x| {
            assert_eq!(x.total_count, 2);
            assert!(x.next_offset.is_none());
            assert_eq!(x.gifts.len(), 2);
        }),
        (
            "get-user-gifts-all",
            GetUserGifts::new(1)
                .with_exclude_from_blockchain(true)
                .with_exclude_limited_non_upgradable(false)
                .with_exclude_limited_upgradable(true)
                .with_exclude_unique(false)
                .with_exclude_unlimited(true)
                .with_limit(100)
                .with_offset("test")
                .with_sort_by_price(true),
            |x| {
                assert_eq!(x.total_count, 2);
                assert!(x.next_offset.is_none());
                assert_eq!(x.gifts.len(), 2);
            },
        ),
    ])
    .await;
    cx.execute_batch([
        (
            "gift-premium-subscription-base",
            GiftPremiumSubscription::new(1, 2, 3),
            |x| assert!(x),
        ),
        (
            "gift-premium-subscription-text-entities",
            GiftPremiumSubscription::new(1, 2, 3).with_text(("text", [TextEntity::bold(0..2)])),
            |x| assert!(x),
        ),
        (
            "gift-premium-subscription-text-parse-mode",
            GiftPremiumSubscription::new(1, 2, 3).with_text(("text", ParseMode::Markdown)),
            |x| assert!(x),
        ),
    ])
    .await;
}

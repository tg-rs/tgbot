use crate::types::*;

#[test]
fn accepted_gift_types() {
    let expected_struct = AcceptedGiftTypes::default();
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = expected_struct
        .with_limited_gifts(true)
        .with_premium_subscription(true)
        .with_unique_gifts(true)
        .with_unlimited_gifts(true);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn gift() {
    let expected_struct = Gift::new(
        "id",
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
        100,
    );
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_personal_remaining_count(1)
            .with_personal_total_count(2)
            .with_publisher_chat(PrivateChat::new(1, "John"))
            .with_remaining_count(10)
            .with_total_count(20)
            .with_upgrade_star_count(30)
    );
}

#[test]
fn gift_info() {
    let expected_struct = GiftInfo::new(Gift::new(
        "id",
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
        100,
    ));
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_can_be_upgraded(true)
            .with_convert_star_count(100)
            .with_entities([TextEntity::bold(0..2)])
            .with_is_private(true)
            .with_owned_gift_id("id")
            .with_prepaid_upgrade_star_count(100)
            .with_text("test")
    );
}

#[test]
fn gifts() {
    let expected_struct = Gifts::from([Gift::new(
        "id",
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
        100,
    )]);
    insta::assert_json_snapshot!(expected_struct.clone());
}

#[test]
fn owned_gift() {
    let expected_struct = OwnedGift::from(OwnedGiftRegular::new(
        Gift::new(
            "id",
            Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            100,
        ),
        2,
    ));
    insta::assert_json_snapshot!(expected_struct);
    let unique = OwnedGiftUnique::new(
        UniqueGift::new(
            UniqueGiftBackdrop {
                colors: UniqueGiftBackdropColors {
                    center_color: 1,
                    edge_color: 2,
                    symbol_color: 3,
                    text_color: 4,
                },
                name: String::from("name"),
                rarity_per_mille: 5,
            },
            String::from("base-name"),
            String::from("gift-id"),
            UniqueGiftModel {
                name: String::from("name"),
                rarity_per_mille: 6,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
            String::from("name"),
            7,
            UniqueGiftSymbol {
                name: String::from("name"),
                rarity_per_mille: 8,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
        ),
        9,
    );
    insta::assert_json_snapshot!(OwnedGift::from(unique.clone()));
    insta::assert_json_snapshot!(OwnedGift::from(
        unique
            .with_transfer_star_count(1)
            .with_can_be_transferred(true)
            .with_is_saved(true)
            .with_next_transfer_date(500)
            .with_owned_gift_id("test-id")
            .with_sender_user(User::new(1, "John", false)),
    ));
}

#[test]
fn owned_gift_regular() {
    let expected_struct = OwnedGiftRegular::new(
        Gift::new(
            "id",
            Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            100,
        ),
        2,
    );
    insta::assert_json_snapshot!(expected_struct.clone(),);
    insta::assert_json_snapshot!(
        expected_struct
            .with_can_be_upgraded(true)
            .with_convert_star_count(3)
            .with_entities([TextEntity::bold(0..2)])
            .with_is_private(true)
            .with_is_saved(true)
            .with_owned_gift_id("id")
            .with_prepaid_upgrade_star_count(4)
            .with_sender_user(User::new(1, "John", false))
            .with_text("test")
            .with_was_refunded(false),
    );
}

#[test]
fn owned_gifts() {
    let expected_struct = OwnedGifts::new(
        [OwnedGift::from(OwnedGiftRegular::new(
            Gift::new(
                "id",
                Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
                100,
            ),
            2,
        ))],
        1,
    );
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(expected_struct.with_next_offset("next"));
}

#[test]
fn unique_gift_info() {
    let expected_struct = UniqueGiftInfo::new(
        UniqueGift::new(
            UniqueGiftBackdrop {
                colors: UniqueGiftBackdropColors {
                    center_color: 1,
                    edge_color: 2,
                    symbol_color: 3,
                    text_color: 4,
                },
                name: String::from("name"),
                rarity_per_mille: 5,
            },
            String::from("base-name"),
            String::from("gift-id"),
            UniqueGiftModel {
                name: String::from("name"),
                rarity_per_mille: 6,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
            String::from("name"),
            7,
            UniqueGiftSymbol {
                name: String::from("name"),
                rarity_per_mille: 8,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
        )
        .with_is_from_blockchain(true)
        .with_publisher_chat(PrivateChat::new(1, "John")),
        UniqueGiftOrigin::Transfer,
    );
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_next_transfer_date(500)
            .with_owned_gift_id("test-id")
            .with_transfer_star_count(400)
            .with_last_resale_star_count(430)
    );
}

#[test]
fn get_available_gifts() {
    assert_payload_eq!(GET "getAvailableGifts" => GetAvailableGifts);
}

#[test]
fn get_business_account_gifts() {
    let method = GetBusinessAccountGifts::new("id");
    assert_payload_eq!(POST JSON "getBusinessAccountGifts" => method.clone());
    let method = method
        .with_exclude_from_blockchain(false)
        .with_exclude_limited_non_upgradable(true)
        .with_exclude_limited_upgradable(false)
        .with_exclude_saved(true)
        .with_exclude_unique(false)
        .with_exclude_unlimited(true)
        .with_exclude_unsaved(false)
        .with_limit(10)
        .with_offset("test")
        .with_sort_by_price(true);
    assert_payload_eq!(POST JSON "getBusinessAccountGifts" => method);
}

#[test]
fn get_chat_gifts() {
    let method = GetChatGifts::new(1);
    assert_payload_eq!(POST JSON "getChatGifts" => method.clone());
    let method = method
        .with_exclude_from_blockchain(true)
        .with_exclude_limited_non_upgradable(false)
        .with_exclude_limited_upgradable(true)
        .with_exclude_saved(false)
        .with_exclude_unique(true)
        .with_exclude_unlimited(false)
        .with_exclude_unsaved(true)
        .with_limit(1)
        .with_offset("")
        .with_sort_by_price(true);
    assert_payload_eq!(POST JSON "getChatGifts" => method);
}

#[test]
fn get_user_gifts() {
    let method = GetUserGifts::new(1);
    assert_payload_eq!(POST JSON "getUserGifts" => method.clone());
    let method = method
        .with_exclude_from_blockchain(true)
        .with_exclude_limited_non_upgradable(false)
        .with_exclude_limited_upgradable(true)
        .with_exclude_unique(false)
        .with_exclude_unlimited(true)
        .with_limit(100)
        .with_offset("")
        .with_sort_by_price(true);
    assert_payload_eq!(POST JSON "getUserGifts" => method);
}

#[test]
fn gift_premium_subscription() {
    let method = GiftPremiumSubscription::new(1, 2, 3);
    assert_payload_eq!(POST JSON "giftPremiumSubscription" => method.clone());
    let method = method
        .clone()
        .with_text("text")
        .with_text_parse_mode(ParseMode::Markdown)
        .with_text_entities([TextEntity::bold(0..2)]);
    assert_payload_eq!(POST JSON "giftPremiumSubscription" => method.clone());
    let method = method
        .clone()
        .with_text("text")
        .with_text_entities([TextEntity::bold(0..2)])
        .with_text_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST JSON "giftPremiumSubscription" => method);
}

#[test]
fn send_gift() {
    let method = SendGift::for_chat_id(1, "test");
    assert_payload_eq!(POST JSON "sendGift" => method.clone());

    let method = SendGift::for_chat_id("@chat", "test");
    assert_payload_eq!(POST JSON "sendGift" => method.clone());

    let method = SendGift::for_user_id(1, "test");
    assert_payload_eq!(POST JSON "sendGift" => method.clone());
    let method = method
        .with_pay_for_upgrade(true)
        .with_text("test")
        .with_text_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST JSON "sendGift" => method.clone());
    let method = method.with_text_entities([TextEntity::bold(0..2)]);
    assert_payload_eq!(POST JSON "sendGift" => method.clone());
    let method = method.with_text_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST JSON "sendGift" => method);
}

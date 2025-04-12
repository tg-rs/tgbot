use crate::{
    api::{Payload, assert_payload_eq},
    types::{
        AcceptedGiftTypes,
        GetAvailableGifts,
        Gift,
        GiftInfo,
        GiftPremiumSubscription,
        Gifts,
        OwnedGift,
        OwnedGiftRegular,
        OwnedGiftUnique,
        OwnedGifts,
        ParseMode,
        SendGift,
        Sticker,
        StickerType,
        TextEntity,
        UniqueGift,
        UniqueGiftBackdrop,
        UniqueGiftBackdropColors,
        UniqueGiftInfo,
        UniqueGiftModel,
        UniqueGiftOrigin,
        UniqueGiftSymbol,
        User,
        tests::assert_json_eq,
    },
};

#[test]
fn accepted_gift_types() {
    let expected_struct = AcceptedGiftTypes::default();
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "limited_gifts": false,
            "premium_subscription": false,
            "unique_gifts": false,
            "unlimited_gifts": false,
        }),
    );
    let expected_struct = expected_struct
        .with_limited_gifts(true)
        .with_premium_subscription(true)
        .with_unique_gifts(true)
        .with_unlimited_gifts(true);
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "limited_gifts": true,
            "premium_subscription": true,
            "unique_gifts": true,
            "unlimited_gifts": true,
        }),
    );
}

#[test]
fn gift() {
    let expected_struct = Gift::new(
        "id",
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
        100,
    );
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": "id",
            "sticker": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "type": "regular",
                "is_animated": false,
                "is_video": false,
                "height": 512,
                "width": 512,
            },
            "star_count": 100,
        }),
    );
    assert_json_eq(
        expected_struct
            .with_remaining_count(10)
            .with_total_count(20)
            .with_upgrade_star_count(30),
        serde_json::json!({
            "id": "id",
            "sticker": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "type": "regular",
                "is_animated": false,
                "is_video": false,
                "height": 512,
                "width": 512,
            },
            "star_count": 100,
            "remaining_count": 10,
            "total_count": 20,
            "upgrade_star_count": 30,
        }),
    );
}

#[test]
fn gift_info() {
    let expected_struct = GiftInfo::new(Gift::new(
        "id",
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
        100,
    ));
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "gift": {
                "id": "id",
                "sticker": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "type": "regular",
                    "is_animated": false,
                    "is_video": false,
                    "height": 512,
                    "width": 512,
                },
                "star_count": 100,
            }
        }),
    );
    assert_json_eq(
        expected_struct
            .with_can_be_upgraded(true)
            .with_convert_star_count(100)
            .with_entities([TextEntity::bold(0..2)])
            .with_is_private(true)
            .with_owned_gift_id("id")
            .with_prepaid_upgrade_star_count(100)
            .with_text("test"),
        serde_json::json!({
            "gift": {
                "id": "id",
                "sticker": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "type": "regular",
                    "is_animated": false,
                    "is_video": false,
                    "height": 512,
                    "width": 512,
                },
                "star_count": 100,
            },
            "can_be_upgraded": true,
            "convert_star_count": 100,
            "entities": [{
                "type": "bold",
                "offset": 0,
                "length": 2,
            }],
            "is_private": true,
            "owned_gift_id": "id",
            "prepaid_upgrade_star_count": 100,
            "text": "test",
        }),
    );
}

#[test]
fn gifts() {
    let expected_struct = Gifts::from([Gift::new(
        "id",
        Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
        100,
    )]);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "gifts": [
                {
                    "id": "id",
                    "sticker": {
                        "file_id": "file-id",
                        "file_unique_id": "file-unique-id",
                        "type": "regular",
                        "is_animated": false,
                        "is_video": false,
                        "height": 512,
                        "width": 512,
                    },
                    "star_count": 100,
                }
            ]
        }),
    );
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
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "regular",
            "gift": {
                "id": "id",
                "sticker": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "type": "regular",
                    "is_animated": false,
                    "is_video": false,
                    "height": 512,
                    "width": 512,
                },
                "star_count": 100,
            },
            "send_date": 2,
        }),
    );
    let expected_struct = OwnedGift::from(OwnedGiftUnique::new(
        UniqueGift {
            backdrop: UniqueGiftBackdrop {
                colors: UniqueGiftBackdropColors {
                    center_color: 1,
                    edge_color: 2,
                    symbol_color: 3,
                    text_color: 4,
                },
                name: String::from("name"),
                rarity_per_mille: 5,
            },
            base_name: String::from("base-name"),
            model: UniqueGiftModel {
                name: String::from("name"),
                rarity_per_mille: 6,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
            name: String::from("name"),
            number: 7,
            symbol: UniqueGiftSymbol {
                name: String::from("name"),
                rarity_per_mille: 8,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
        },
        9,
    ));
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "unique",
            "gift": {
                "backdrop": {
                    "colors": {
                        "center_color": 1,
                        "edge_color": 2,
                        "symbol_color": 3,
                        "text_color": 4,
                    },
                    "name": "name",
                    "rarity_per_mille": 5,
                },
                "base_name": "base-name",
                "model": {
                    "name": "name",
                    "rarity_per_mille": 6,
                    "sticker": {
                        "file_id": "file-id",
                        "file_unique_id": "file-unique-id",
                        "type": "regular",
                        "is_animated": false,
                        "is_video": false,
                        "height": 512,
                        "width": 512,
                    },
                },
                "name": "name",
                "number": 7,
                "symbol": {
                    "name": "name",
                    "rarity_per_mille": 8,
                    "sticker": {
                        "file_id": "file-id",
                        "file_unique_id": "file-unique-id",
                        "type": "regular",
                        "is_animated": false,
                        "is_video": false,
                        "height": 512,
                        "width": 512,
                    },
                },
            },
            "send_date": 9
        }),
    );
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
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "gift": {
                "id": "id",
                "sticker": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "type": "regular",
                    "is_animated": false,
                    "is_video": false,
                    "height": 512,
                    "width": 512,
                },
                "star_count": 100,
            },
            "send_date": 2,
        }),
    );
    assert_json_eq(
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
        serde_json::json!({
            "gift": {
                "id": "id",
                "sticker": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "type": "regular",
                    "is_animated": false,
                    "is_video": false,
                    "height": 512,
                    "width": 512,
                },
                "star_count": 100,
            },
            "send_date": 2,
            "can_be_upgraded": true,
            "convert_star_count": 3,
            "entities": [{"type": "bold", "offset": 0, "length": 2}],
            "is_private": true,
            "is_saved": true,
            "owned_gift_id": "id",
            "prepaid_upgrade_star_count": 4,
            "sender_user": {
                "id": 1,
                "first_name": "John",
                "is_bot": false
            },
            "text": "test",
            "was_refunded": false,
        }),
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
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "gifts": [{
                "type": "regular",
                "gift": {
                    "id": "id",
                    "sticker": {
                        "file_id": "file-id",
                        "file_unique_id": "file-unique-id",
                        "type": "regular",
                        "is_animated": false,
                        "is_video": false,
                        "height": 512,
                        "width": 512,
                    },
                    "star_count": 100,
                },
                "send_date": 2,
            }],
            "total_count": 1,
        }),
    );
    assert_json_eq(
        expected_struct.with_next_offset("next"),
        serde_json::json!({
            "gifts": [{
                "type": "regular",
                "gift": {
                    "id": "id",
                    "sticker": {
                        "file_id": "file-id",
                        "file_unique_id": "file-unique-id",
                        "type": "regular",
                        "is_animated": false,
                        "is_video": false,
                        "height": 512,
                        "width": 512,
                    },
                    "star_count": 100,
                },
                "send_date": 2,
            }],
            "total_count": 1,
            "next_offset": "next",
        }),
    );
}

#[test]
fn unique_gift_info() {
    let expected_struct = UniqueGiftInfo::new(
        UniqueGift {
            backdrop: UniqueGiftBackdrop {
                colors: UniqueGiftBackdropColors {
                    center_color: 1,
                    edge_color: 2,
                    symbol_color: 3,
                    text_color: 4,
                },
                name: String::from("name"),
                rarity_per_mille: 5,
            },
            base_name: String::from("base-name"),
            model: UniqueGiftModel {
                name: String::from("name"),
                rarity_per_mille: 6,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
            name: String::from("name"),
            number: 7,
            symbol: UniqueGiftSymbol {
                name: String::from("name"),
                rarity_per_mille: 8,
                sticker: Sticker::new("file-id", "file-unique-id", StickerType::Regular, 512, 512),
            },
        },
        UniqueGiftOrigin::Transfer,
    );
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "gift": {
                "backdrop": {
                    "colors": {
                        "center_color": 1,
                        "edge_color": 2,
                        "symbol_color": 3,
                        "text_color": 4,
                    },
                    "name": "name",
                    "rarity_per_mille": 5,
                },
                "base_name": "base-name",
                "model": {
                    "name": "name",
                    "rarity_per_mille": 6,
                    "sticker": {
                        "file_id": "file-id",
                        "file_unique_id": "file-unique-id",
                        "type": "regular",
                        "is_animated": false,
                        "is_video": false,
                        "height": 512,
                        "width": 512,
                    },
                },
                "name": "name",
                "number": 7,
                "symbol": {
                    "name": "name",
                    "rarity_per_mille": 8,
                    "sticker": {
                        "file_id": "file-id",
                        "file_unique_id": "file-unique-id",
                        "type": "regular",
                        "is_animated": false,
                        "is_video": false,
                        "height": 512,
                        "width": 512,
                    },
                },
            },
            "origin": "transfer",
        }),
    );
}

#[test]
fn get_available_gifts() {
    assert_payload_eq(Payload::empty("getAvailableGifts"), GetAvailableGifts);
}

#[test]
fn gift_premium_subscription() {
    let method = GiftPremiumSubscription::new(1, 2, 3);
    assert_payload_eq(
        Payload::json(
            "giftPremiumSubscription",
            serde_json::json!({
                "month_count": 1,
                "star_count": 2,
                "user_id": 3
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "giftPremiumSubscription",
            serde_json::json!({
                "month_count": 1,
                "star_count": 2,
                "user_id": 3,
                "text": "text",
                "text_entities": [
                    {"type": "bold", "offset": 0, "length": 2}
                ]
            }),
        ),
        method
            .clone()
            .with_text("text")
            .with_text_parse_mode(ParseMode::Markdown)
            .with_text_entities([TextEntity::bold(0..2)]),
    );
    assert_payload_eq(
        Payload::json(
            "giftPremiumSubscription",
            serde_json::json!({
                "month_count": 1,
                "star_count": 2,
                "user_id": 3,
                "text": "text",
                "text_parse_mode": "Markdown",
            }),
        ),
        method
            .clone()
            .with_text("text")
            .with_text_entities([TextEntity::bold(0..2)])
            .with_text_parse_mode(ParseMode::Markdown),
    );
}

#[test]
fn send_gift() {
    let method = SendGift::for_chat_id(1, "test");
    assert_payload_eq(
        Payload::json(
            "sendGift",
            serde_json::json!({
                "chat_id": 1,
                "gift_id": "test",
            }),
        ),
        method.clone(),
    );

    let method = SendGift::for_chat_id("@chat", "test");
    assert_payload_eq(
        Payload::json(
            "sendGift",
            serde_json::json!({
                "chat_id": "@chat",
                "gift_id": "test",
            }),
        ),
        method.clone(),
    );

    let method = SendGift::for_user_id(1, "test");
    assert_payload_eq(
        Payload::json(
            "sendGift",
            serde_json::json!({
                "user_id": 1,
                "gift_id": "test",
            }),
        ),
        method.clone(),
    );
    let method = method
        .with_pay_for_upgrade(true)
        .with_text("test")
        .with_text_parse_mode(ParseMode::Markdown);
    assert_payload_eq(
        Payload::json(
            "sendGift",
            serde_json::json!({
                "user_id": 1,
                "gift_id": "test",
                "pay_for_upgrade": true,
                "text": "test",
                "text_parse_mode": "Markdown",
            }),
        ),
        method.clone(),
    );
    let method = method.with_text_entities([TextEntity::bold(0..2)]);
    assert_payload_eq(
        Payload::json(
            "sendGift",
            serde_json::json!({
                "user_id": 1,
                "gift_id": "test",
                "pay_for_upgrade": true,
                "text": "test",
                "text_entities": [
                    {
                        "type": "bold",
                        "offset": 0,
                        "length": 2,
                    }
                ],
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "sendGift",
            serde_json::json!({
                "user_id": 1,
                "gift_id": "test",
                "pay_for_upgrade": true,
                "text": "test",
                "text_parse_mode": "Markdown",
            }),
        ),
        method.with_text_parse_mode(ParseMode::Markdown),
    );
}

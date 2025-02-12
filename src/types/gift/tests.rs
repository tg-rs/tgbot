use super::GetAvailableGifts;
use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, Gift, Gifts, ParseMode, SendGift, Sticker, StickerType, TextEntity},
};

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
fn get_available_gifts() {
    assert_payload_eq(Payload::empty("getAvailableGifts"), GetAvailableGifts);
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

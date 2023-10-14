use crate::{
    api::{assert_payload_eq, Payload},
    types::{DeleteChatStickerSet, SetChatStickerSet},
};

#[test]
fn delete_chat_sticker_set() {
    assert_payload_eq(
        Payload::json(
            "deleteChatStickerSet",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        DeleteChatStickerSet::new(1),
    );
}

#[test]
fn set_chat_sticker_set() {
    assert_payload_eq(
        Payload::json(
            "setChatStickerSet",
            serde_json::json!({"chat_id": 1, "sticker_set_name": "Sticker Set"}),
        ),
        SetChatStickerSet::new(1, "Sticker Set"),
    );
}

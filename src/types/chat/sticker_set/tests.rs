use crate::{
    tests::{assert_request_eq, ExpectedRequest},
    types::{DeleteChatStickerSet, SetChatStickerSet},
};

#[test]
fn delete_chat_sticker_set() {
    assert_request_eq(
        ExpectedRequest::post_json(
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
    assert_request_eq(
        ExpectedRequest::post_json(
            "setChatStickerSet",
            serde_json::json!({"chat_id": 1, "sticker_set_name": "Sticker Set"}),
        ),
        SetChatStickerSet::new(1, "Sticker Set"),
    );
}

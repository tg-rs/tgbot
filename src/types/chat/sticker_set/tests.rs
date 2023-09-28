use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{DeleteChatStickerSet, SetChatStickerSet},
};

#[test]
fn delete_chat_sticker_set() {
    let request = DeleteChatStickerSet::new(1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/deleteChatStickerSet"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn set_chat_sticker_set() {
    let request = SetChatStickerSet::new(1, "name").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/setChatStickerSet"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["sticker_set_name"], "name");
    } else {
        panic!("Unexpected request body");
    }
}

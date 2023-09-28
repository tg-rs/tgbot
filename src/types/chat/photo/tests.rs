use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ChatPhoto, DeleteChatPhoto, InputFile, SetChatPhoto},
};

#[test]
fn chat_photo_deserialize() {
    let data: ChatPhoto = serde_json::from_value(serde_json::json!({
        "small_file_id": "small-id",
        "big_file_id": "big-id",
        "small_file_unique_id": "small-unique-id",
        "big_file_unique_id": "big-unique-id"
    }))
    .unwrap();
    assert_eq!(data.small_file_id, "small-id");
    assert_eq!(data.big_file_id, "big-id");
    assert_eq!(data.small_file_unique_id, "small-unique-id");
    assert_eq!(data.big_file_unique_id, "big-unique-id");
}

#[test]
fn delete_chat_photo() {
    let request = DeleteChatPhoto::new(1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/deleteChatPhoto"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn set_chat_photo() {
    let request = SetChatPhoto::new(1, InputFile::file_id("sticker-id")).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setChatPhoto");
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert!(form.fields["photo"].get_file().is_some());
    } else {
        panic!("Unexpected request body");
    }
}

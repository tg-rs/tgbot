use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{tests::assert_json_eq, ChatPhoto, DeleteChatPhoto, InputFile, SetChatPhoto},
};

#[test]
fn chat_photo() {
    assert_json_eq(
        ChatPhoto::new(
            "big-file-id",
            "big-file-unique-id",
            "small-file-id",
            "small-file-unique-id",
        ),
        serde_json::json!({
            "small_file_id": "small-file-id",
            "big_file_id": "big-file-id",
            "small_file_unique_id": "small-file-unique-id",
            "big_file_unique_id": "big-file-unique-id"
        }),
    );
}

#[test]
fn delete_chat_photo() {
    assert_payload_eq(
        Payload::json(
            "deleteChatPhoto",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        DeleteChatPhoto::new(1),
    );
}

#[test]
fn set_chat_photo() {
    assert_payload_eq(
        Payload::form(
            "setChatPhoto",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("photo", InputFile::file_id("photo-id").into()),
            ]),
        ),
        SetChatPhoto::new(1, InputFile::file_id("photo-id")),
    );
}

use crate::{
    api::{Payload, assert_payload_eq},
    types::{ChatAction, SendChatAction, tests::assert_json_eq},
};

#[test]
fn chat_action() {
    for (expected_struct, expected_value) in [
        (ChatAction::ChooseSticker, serde_json::json!("choose_sticker")),
        (ChatAction::FindLocation, serde_json::json!("find_location")),
        (ChatAction::RecordVideo, serde_json::json!("record_video")),
        (ChatAction::RecordVideoNote, serde_json::json!("record_video_note")),
        (ChatAction::RecordVoice, serde_json::json!("record_voice")),
        (ChatAction::Typing, serde_json::json!("typing")),
        (ChatAction::UploadDocument, serde_json::json!("upload_document")),
        (ChatAction::UploadPhoto, serde_json::json!("upload_photo")),
        (ChatAction::UploadVideo, serde_json::json!("upload_video")),
        (ChatAction::UploadVideoNote, serde_json::json!("upload_video_note")),
        (ChatAction::UploadVoice, serde_json::json!("upload_voice")),
    ] {
        assert_json_eq(expected_struct, expected_value);
    }
}

#[test]
fn send_chat_action() {
    let method = SendChatAction::new(1, ChatAction::Typing);
    assert_payload_eq(
        Payload::json(
            "sendChatAction",
            serde_json::json!({
                "chat_id": 1,
                "action": "typing"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "sendChatAction",
            serde_json::json!({
                "chat_id": 1,
                "action": "typing",
                "business_connection_id": "id",
                "message_thread_id": 1,
            }),
        ),
        method.with_business_connection_id("id").with_message_thread_id(1),
    );
}

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ChatAction, SendChatAction},
};

fn to_json(value: &impl Serialize) -> String {
    serde_json::to_string(value).unwrap()
}

fn from_json<T>(value: &str) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_str(value).unwrap()
}

#[test]
fn deserialize_chat_action() {
    assert_eq!(
        from_json::<ChatAction>(r#""choose_sticker""#),
        ChatAction::ChooseSticker
    );
    assert_eq!(from_json::<ChatAction>(r#""find_location""#), ChatAction::FindLocation);
    assert_eq!(from_json::<ChatAction>(r#""record_video""#), ChatAction::RecordVideo);
    assert_eq!(
        from_json::<ChatAction>(r#""record_video_note""#),
        ChatAction::RecordVideoNote
    );
    assert_eq!(from_json::<ChatAction>(r#""record_voice""#), ChatAction::RecordVoice);
    assert_eq!(from_json::<ChatAction>(r#""typing""#), ChatAction::Typing);
    assert_eq!(
        from_json::<ChatAction>(r#""upload_document""#),
        ChatAction::UploadDocument
    );
    assert_eq!(from_json::<ChatAction>(r#""upload_photo""#), ChatAction::UploadPhoto);
    assert_eq!(from_json::<ChatAction>(r#""upload_video""#), ChatAction::UploadVideo);
    assert_eq!(
        from_json::<ChatAction>(r#""upload_video_note""#),
        ChatAction::UploadVideoNote
    );
    assert_eq!(from_json::<ChatAction>(r#""upload_voice""#), ChatAction::UploadVoice);
}

#[test]
fn serialize_chat_action() {
    assert_eq!(to_json(&ChatAction::ChooseSticker), r#""choose_sticker""#);
    assert_eq!(to_json(&ChatAction::FindLocation), r#""find_location""#);
    assert_eq!(to_json(&ChatAction::RecordVideo), r#""record_video""#);
    assert_eq!(to_json(&ChatAction::RecordVideoNote), r#""record_video_note""#);
    assert_eq!(to_json(&ChatAction::RecordVoice), r#""record_voice""#);
    assert_eq!(to_json(&ChatAction::Typing), r#""typing""#);
    assert_eq!(to_json(&ChatAction::UploadDocument), r#""upload_document""#);
    assert_eq!(to_json(&ChatAction::UploadPhoto), r#""upload_photo""#);
    assert_eq!(to_json(&ChatAction::UploadVideo), r#""upload_video""#);
    assert_eq!(to_json(&ChatAction::UploadVideoNote), r#""upload_video_note""#);
    assert_eq!(to_json(&ChatAction::UploadVoice), r#""upload_voice""#);
}

#[test]
fn send_chat_action() {
    let request = SendChatAction::new(1, ChatAction::Typing).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/sendChatAction"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["action"], "typing");
    } else {
        panic!("Unexpected request body");
    }
}

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ForceReply, InputFile, ParseMode, SendVoice, TextEntity, Voice},
};

#[test]
fn voice_deserialize_full() {
    let data: Voice = serde_json::from_value(serde_json::json!({
        "file_id": "voice file id",
        "file_unique_id": "unique-id",
        "duration": 123,
        "mime_type": "audio/ogg",
        "file_size": 1234
    }))
    .unwrap();

    assert_eq!(data.file_id, "voice file id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.duration, 123);
    assert_eq!(data.mime_type.unwrap(), "audio/ogg");
    assert_eq!(data.file_size.unwrap(), 1234);
}

#[test]
fn voice_deserialize_partial() {
    let data: Voice = serde_json::from_value(serde_json::json!({
        "file_id": "voice file id",
        "file_unique_id": "unique-id",
        "duration": 123
    }))
    .unwrap();

    assert_eq!(data.file_id, "voice file id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.duration, 123);
    assert!(data.mime_type.is_none());
    assert!(data.file_size.is_none());
}

#[test]
fn send_voice() {
    let request = SendVoice::new(1, InputFile::file_id("file-id"))
        .caption("caption")
        .parse_mode(ParseMode::Markdown)
        .duration(100)
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .unwrap()
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendVoice");
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert!(form.fields["voice"].get_file().is_some());
        assert_eq!(form.fields["caption"].get_text().unwrap(), "caption");
        assert_eq!(form.fields["parse_mode"].get_text().unwrap(), "Markdown");
        assert_eq!(form.fields["duration"].get_text().unwrap(), "100");
        assert_eq!(form.fields["disable_notification"].get_text().unwrap(), "true");
        assert_eq!(form.fields["protect_content"].get_text().unwrap(), "true");
        assert_eq!(form.fields["reply_to_message_id"].get_text().unwrap(), "1");
        assert_eq!(form.fields["allow_sending_without_reply"].get_text().unwrap(), "true");
        assert_eq!(
            form.fields["reply_markup"].get_text().unwrap(),
            r#"{"force_reply":true}"#
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn send_voice_caption() {
    let mut method = SendVoice::new(1, InputFile::file_id("file-id"));
    method = method.parse_mode(ParseMode::Markdown);
    assert_eq!(method.form.fields["parse_mode"].get_text().unwrap(), "Markdown");
    method = method.caption_entities(vec![TextEntity::bold(0..10)]).unwrap();
    assert!(!method.form.fields.contains_key("parse_mode"));
    let caption_entities = method.form.fields["caption_entities"].get_text().unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(caption_entities).unwrap(),
        serde_json::json!([{"type": "bold", "offset":0, "length": 10}])
    );
}

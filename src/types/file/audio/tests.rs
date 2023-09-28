use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{Audio, ForceReply, InputFile, ParseMode, SendAudio, TextEntity},
};

#[test]
fn audio_deserialize_full() {
    let data: Audio = serde_json::from_value(serde_json::json!({
        "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
        "file_unique_id": "unique-id",
        "duration": 243,
        "performer": "Performer",
        "title": "Title",
        "file_name": "Filename",
        "mime_type": "audio/mpeg",
        "file_size": 1234,
        "thumb": {
            "file_id": "AdddddUuUUUUccccUUmm_PPP",
            "file_unique_id": "unique-thumb-id",
            "width": 24,
            "height": 24,
            "file_size": 12324
        }
    }))
    .unwrap();

    assert_eq!(data.file_id, "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.duration, 243);
    assert_eq!(data.performer.unwrap(), "Performer");
    assert_eq!(data.title.unwrap(), "Title");
    assert_eq!(data.file_name.unwrap(), "Filename");
    assert_eq!(data.mime_type.unwrap(), "audio/mpeg");
    assert_eq!(data.file_size.unwrap(), 1234);

    let thumb = data.thumb.unwrap();
    assert_eq!(thumb.file_id, "AdddddUuUUUUccccUUmm_PPP");
    assert_eq!(thumb.file_unique_id, "unique-thumb-id");
    assert_eq!(thumb.width, 24);
    assert_eq!(thumb.height, 24);
    assert_eq!(thumb.file_size.unwrap(), 12324);
}

#[test]
fn audio_deserialize_partial() {
    let data: Audio = serde_json::from_value(serde_json::json!({
        "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
        "file_unique_id": "unique-id",
        "duration": 243
    }))
    .unwrap();
    assert_eq!(data.file_id, "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.duration, 243);
    assert!(data.performer.is_none());
    assert!(data.title.is_none());
    assert!(data.file_name.is_none());
    assert!(data.mime_type.is_none());
    assert!(data.file_size.is_none());
    assert!(data.thumb.is_none());
}

#[test]
fn send_audio() {
    let request = SendAudio::new(1, InputFile::file_id("file-id"))
        .caption("caption")
        .parse_mode(ParseMode::Markdown)
        .duration(100)
        .performer("performer")
        .title("title")
        .thumb(InputFile::file_id("thumb-id"))
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .unwrap()
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendAudio");
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert!(form.fields["audio"].get_file().is_some());
        assert_eq!(form.fields["caption"].get_text().unwrap(), "caption");
        assert_eq!(form.fields["parse_mode"].get_text().unwrap(), "Markdown");
        assert_eq!(form.fields["duration"].get_text().unwrap(), "100");
        assert_eq!(form.fields["performer"].get_text().unwrap(), "performer");
        assert_eq!(form.fields["title"].get_text().unwrap(), "title");
        assert!(form.fields["thumb"].get_file().is_some());
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
fn send_audio_caption() {
    let mut method = SendAudio::new(1, InputFile::file_id("file-id"));
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

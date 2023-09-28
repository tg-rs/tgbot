use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ForceReply, InputFile, SendVideoNote, VideoNote},
};

#[test]
fn video_note_deserialize_full() {
    let data: VideoNote = serde_json::from_value(serde_json::json!({
        "file_id": "video note file id",
        "file_unique_id": "unique-id",
        "length": 124,
        "duration": 1234,
        "thumb": {
            "file_id": "AdddddUuUUUUccccUUmm_PPP",
            "file_unique_id": "unique-thumb-id",
            "width": 24,
            "height": 24,
            "file_size": 12324
        },
        "file_size": 12345
    }))
    .unwrap();

    assert_eq!(data.file_id, "video note file id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.length, 124);
    assert_eq!(data.duration, 1234);

    let thumb = data.thumb.unwrap();
    assert_eq!(thumb.file_id, "AdddddUuUUUUccccUUmm_PPP");
    assert_eq!(thumb.file_unique_id, "unique-thumb-id");
    assert_eq!(thumb.width, 24);
    assert_eq!(thumb.height, 24);
    assert_eq!(thumb.file_size.unwrap(), 12324);

    assert_eq!(data.file_size.unwrap(), 12345);
}

#[test]
fn video_note_deserialize_partial() {
    let data: VideoNote = serde_json::from_value(serde_json::json!({
        "file_id": "video note file id",
        "file_unique_id": "unique-id",
        "length": 124,
        "duration": 1234
    }))
    .unwrap();

    assert_eq!(data.file_id, "video note file id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.length, 124);
    assert_eq!(data.duration, 1234);
    assert!(data.thumb.is_none());
    assert!(data.file_size.is_none());
}

#[test]
fn send_video_note() {
    let request = SendVideoNote::new(1, InputFile::file_id("file-id"))
        .duration(50)
        .length(100)
        .thumb(InputFile::file_id("thumb-id"))
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .unwrap()
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/sendVideoNote"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert!(form.fields["video_note"].get_file().is_some());
        assert_eq!(form.fields["duration"].get_text().unwrap(), "50");
        assert_eq!(form.fields["length"].get_text().unwrap(), "100");
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

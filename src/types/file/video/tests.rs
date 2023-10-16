use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{tests::assert_json_eq, ForceReply, InputFile, ParseMode, PhotoSize, SendVideo, TextEntity, Video},
};

#[test]
fn video() {
    assert_json_eq(
        Video {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            width: 1,
            height: 2,
            duration: 3,
            thumb: Some(PhotoSize {
                file_id: String::from("thumb-file-id"),
                file_unique_id: String::from("thumb-file-unique-id"),
                width: 24,
                height: 24,
                file_size: Some(1024),
            }),
            file_name: Some(String::from("File Name")),
            mime_type: Some(String::from("video/mpeg")),
            file_size: Some(10240),
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 1,
            "height": 2,
            "duration": 3,
            "thumb": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-file-unique-id",
                "width": 24,
                "height": 24,
                "file_size": 1024
            },
            "file_name": "File Name",
            "mime_type": "video/mpeg",
            "file_size": 10240
        }),
    );
    assert_json_eq(
        Video {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            width: 1,
            height: 2,
            duration: 3,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 1,
            "height": 2,
            "duration": 3,
        }),
    );
}

#[test]
fn send_video() {
    assert_payload_eq(
        Payload::form(
            "sendVideo",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendVideo::new(1, InputFile::file_id("file-id")),
    );
    assert_payload_eq(
        Payload::form(
            "sendVideo",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video", InputFile::file_id("file-id").into()),
                ("duration", 100.into()),
                ("width", 200.into()),
                ("height", 300.into()),
                ("thumb", InputFile::file_id("thumb-id").into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
                ("supports_streaming", true.into()),
                ("disable_notification", true.into()),
                ("protect_content", true.into()),
                ("reply_to_message_id", 1.into()),
                ("allow_sending_without_reply", true.into()),
                (
                    "reply_markup",
                    serde_json::to_string(&ForceReply::new(true)).unwrap().into(),
                ),
                ("message_thread_id", 1.into()),
            ]),
        ),
        SendVideo::new(1, InputFile::file_id("file-id"))
            .duration(100)
            .width(200)
            .height(300)
            .thumb(InputFile::file_id("thumb-id"))
            .caption("Caption")
            .parse_mode(ParseMode::Markdown)
            .supports_streaming(true)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .unwrap()
            .message_thread_id(1),
    );
}

#[test]
fn send_video_caption_entities_vs_parse_mode() {
    let mut method = SendVideo::new(1, InputFile::file_id("file-id"));

    method = method.parse_mode(ParseMode::Markdown);
    assert_eq!(
        method.form.get_field("parse_mode").unwrap().get_text().unwrap(),
        "Markdown"
    );

    method = method.caption_entities(vec![TextEntity::bold(0..10)]).unwrap();
    assert!(!method.form.has_field("parse_mode"));

    let caption_entities = method.form.get_field("caption_entities").unwrap().get_text().unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(caption_entities).unwrap(),
        serde_json::json!([{"type": "bold", "offset":0, "length": 10}])
    );

    method = method.parse_mode(ParseMode::Markdown);
    assert!(!method.form.has_field("caption_entities"));
}

use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        ForceReply,
        InputFile,
        ParseMode,
        PhotoSize,
        ReplyParameters,
        SendVideo,
        SendVideoError,
        TextEntity,
        Video,
    },
};

#[test]
fn video() {
    assert_json_eq(
        Video::new(3, "file-id", "file-unique-id", 2, 1)
            .with_cover([PhotoSize::new("cover-file-id", "cover-file-unique-id", 24, 24)])
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_name("File Name")
            .with_mime_type("video/mpeg")
            .with_file_size(10240)
            .with_start_timestamp(20),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 1,
            "height": 2,
            "duration": 3,
            "cover": [
                {
                    "file_id": "cover-file-id",
                    "file_unique_id": "cover-file-unique-id",
                    "width": 24,
                    "height": 24
                }
            ],
            "thumbnail": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-file-unique-id",
                "width": 24,
                "height": 24,
                "file_size": 1024
            },
            "file_name": "File Name",
            "mime_type": "video/mpeg",
            "file_size": 10240,
            "start_timestamp": 20
        }),
    );
    assert_json_eq(
        Video::new(3, "file-id", "file-unique-id", 2, 1),
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
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendVideo",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video", InputFile::file_id("file-id").into()),
                ("duration", 100.into()),
                ("width", 200.into()),
                ("height", 300.into()),
                ("thumbnail", InputFile::url("https://example.com/image.jpg").into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
                ("supports_streaming", true.into()),
                ("allow_paid_broadcast", true.into()),
                ("business_connection_id", "id".into()),
                ("disable_notification", true.into()),
                ("has_spoiler", true.into()),
                ("message_effect_id", "effect-id".into()),
                ("message_thread_id", 1.into()),
                ("protect_content", true.into()),
                (
                    "reply_markup",
                    serde_json::to_string(&ForceReply::new(true)).unwrap().into(),
                ),
                ("reply_parameters", reply_parameters.serialize().unwrap().into()),
                ("show_caption_above_media", true.into()),
                ("start_timestamp", 20.into()),
                ("cover", InputFile::file_id("cover-id").into()),
            ]),
        ),
        SendVideo::new(1, InputFile::file_id("file-id"))
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_caption("Caption")
            .with_disable_notification(true)
            .with_duration(100)
            .with_has_spoiler(true)
            .with_height(300)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_parameters(reply_parameters)
            .unwrap()
            .with_show_caption_above_media(true)
            .with_supports_streaming(true)
            .with_thumbnail(InputFile::url("https://example.com/image.jpg"))
            .unwrap()
            .with_width(200)
            .with_start_timestamp(20)
            .with_cover(InputFile::file_id("cover-id")),
    );
}

#[test]
fn send_video_with_thumbnail() {
    let err = SendVideo::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("file-id"))
        .unwrap_err();
    assert!(matches!(err, SendVideoError::InvalidThumbnail));
}

#[test]
fn send_video_entities_vs_parse_mode() {
    assert_payload_eq(
        Payload::form(
            "sendVideo",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video", InputFile::file_id("file-id").into()),
                ("parse_mode", "Markdown".into()),
            ]),
        ),
        SendVideo::new(1, InputFile::file_id("file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_payload_eq(
        Payload::form(
            "sendVideo",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video", InputFile::file_id("file-id").into()),
                ("caption_entities", r#"[{"offset":0,"length":10,"type":"bold"}]"#.into()),
            ]),
        ),
        SendVideo::new(1, InputFile::file_id("file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
}

use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        Animation,
        ForceReply,
        InputFile,
        ParseMode,
        PhotoSize,
        ReplyParameters,
        SendAnimation,
        SendAnimationError,
        TextEntity,
    },
};

#[test]
fn animation() {
    assert_json_eq(
        Animation::new(243, "file-id", "file-unique-id", 200, 200)
            .with_file_name("File Name")
            .with_file_size(20480)
            .with_mime_type("image/gif")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024)),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 200,
            "height": 200,
            "duration": 243,
            "thumbnail": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-file-unique-id",
                "width": 24,
                "height": 24,
                "file_size": 1024
            },
            "file_name": "File Name",
            "mime_type": "image/gif",
            "file_size": 20480
        }),
    );
    assert_json_eq(
        Animation::new(30, "file-id", "file-unique-id", 200, 200),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 200,
            "height": 200,
            "duration": 30,
        }),
    );
}

#[test]
fn send_animation() {
    assert_payload_eq(
        Payload::form(
            "sendAnimation",
            Form::from([
                ("animation", InputFile::file_id("file-id").into()),
                ("chat_id", FormValue::from(1)),
            ]),
        ),
        SendAnimation::new(InputFile::file_id("file-id"), 1),
    );
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendAnimation",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("animation", InputFile::file_id("file-id").into()),
                ("duration", 100.into()),
                ("width", 200.into()),
                ("height", 300.into()),
                ("thumbnail", InputFile::url("https://google.com/favicon.ico").into()),
                ("business_connection_id", "id".into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
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
            ]),
        ),
        SendAnimation::new(InputFile::file_id("file-id"), 1)
            .with_duration(100)
            .with_width(200)
            .with_height(300)
            .with_thumbnail(InputFile::url("https://google.com/favicon.ico"))
            .unwrap()
            .with_business_connection_id("id")
            .with_caption("Caption")
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_disable_notification(true)
            .with_has_spoiler(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_parameters(reply_parameters)
            .unwrap()
            .with_show_caption_above_media(true),
    );
}

#[test]
fn send_animation_with_thumbnail() {
    let err = SendAnimation::new(InputFile::file_id("file-id"), 1)
        .with_thumbnail(InputFile::file_id("file-id"))
        .unwrap_err();
    assert!(matches!(err, SendAnimationError::InvalidThumbnail));
}

#[test]
fn send_animation_entities_vs_parse_mode() {
    assert_payload_eq(
        Payload::form(
            "sendAnimation",
            Form::from([
                ("chat_id", 1.into()),
                ("animation", InputFile::file_id("file-id").into()),
                ("caption_entities", r#"[{"offset":0,"length":10,"type":"bold"}]"#.into()),
            ]),
        ),
        SendAnimation::new(InputFile::file_id("file-id"), 1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
    assert_payload_eq(
        Payload::form(
            "sendAnimation",
            Form::from([
                ("chat_id", 1.into()),
                ("animation", InputFile::file_id("file-id").into()),
                ("parse_mode", "Markdown".into()),
            ]),
        ),
        SendAnimation::new(InputFile::file_id("file-id"), 1)
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .with_caption_parse_mode(ParseMode::Markdown),
    );
}

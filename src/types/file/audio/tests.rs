use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        Audio,
        ForceReply,
        InputFile,
        ParseMode,
        PhotoSize,
        ReplyParameters,
        SendAudio,
        SendAudioError,
        TextEntity,
    },
};

#[test]
fn audio() {
    assert_json_eq(
        Audio::new(243, "file-id", "file-unique-id")
            .with_file_name("File Name")
            .with_file_size(10240)
            .with_mime_type("audio/mpeg")
            .with_performer("Performer")
            .with_title("Title")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-unique-file-id", 24, 24).with_file_size(1024)),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "duration": 243,
            "performer": "Performer",
            "title": "Title",
            "file_name": "File Name",
            "mime_type": "audio/mpeg",
            "file_size": 10240,
            "thumbnail": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-unique-file-id",
                "width": 24,
                "height": 24,
                "file_size": 1024
            }
        }),
    );
    assert_json_eq(
        Audio::new(243, "file-id", "file-unique-id"),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "duration": 243,
        }),
    );
}

#[test]
fn send_audio() {
    assert_payload_eq(
        Payload::form(
            "sendAudio",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("audio", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendAudio::new(1, InputFile::file_id("file-id")),
    );
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendAudio",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("audio", InputFile::file_id("file-id").into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
                ("duration", 100.into()),
                ("performer", "Performer".into()),
                ("title", "Title".into()),
                ("thumbnail", InputFile::url("https://google.com/favicon.ico").into()),
                ("business_connection_id", "id".into()),
                ("disable_notification", true.into()),
                ("message_effect_id", "effect-id".into()),
                ("message_thread_id", 1.into()),
                ("protect_content", true.into()),
                (
                    "reply_markup",
                    serde_json::to_string(&ForceReply::new(true)).unwrap().into(),
                ),
                ("reply_parameters", reply_parameters.serialize().unwrap().into()),
            ]),
        ),
        SendAudio::new(1, InputFile::file_id("file-id"))
            .with_business_connection_id("id")
            .with_caption("Caption")
            .with_disable_notification(true)
            .with_duration(100)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_performer("Performer")
            .with_protect_content(true)
            .with_title("Title")
            .with_thumbnail(InputFile::url("https://google.com/favicon.ico"))
            .unwrap()
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_parameters(reply_parameters)
            .unwrap(),
    );
}

#[test]
fn send_audio_with_thumbnail() {
    let err = SendAudio::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("thumb-id"))
        .unwrap_err();
    assert!(matches!(err, SendAudioError::InvalidThumbnail));
}

#[test]
fn send_audio_entities_vs_parse_mode() {
    assert_payload_eq(
        Payload::form(
            "sendAudio",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("audio", InputFile::file_id("file-id").into()),
                ("parse_mode", "Markdown".into()),
            ]),
        ),
        SendAudio::new(1, InputFile::file_id("file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .with_caption_parse_mode(ParseMode::Markdown),
    );

    assert_payload_eq(
        Payload::form(
            "sendAudio",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("audio", InputFile::file_id("file-id").into()),
                ("caption_entities", r#"[{"offset":0,"length":10,"type":"bold"}]"#.into()),
            ]),
        ),
        SendAudio::new(1, InputFile::file_id("file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
}

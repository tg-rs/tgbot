use crate::{
    api::{Form, FormValue, Payload, assert_payload_eq},
    types::*,
};

#[test]
fn file() {
    insta::assert_json_snapshot!(
        File::new("file-id", "file-unique-id")
            .with_file_path("file-path")
            .with_file_size(1024)
    );
    insta::assert_json_snapshot!(File::new("file-id", "file-unique-id"));
}

#[test]
fn get_file() {
    assert_payload_eq(
        Payload::json(
            "getFile",
            serde_json::json!({
                "file_id": "file-id"
            }),
        ),
        GetFile::new("file-id"),
    );
}

#[test]
fn animation() {
    insta::assert_json_snapshot!(
        Animation::new(243, "file-id", "file-unique-id", 200, 200)
            .with_file_name("File Name")
            .with_file_size(20480)
            .with_mime_type("image/gif")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024)),
    );
    insta::assert_json_snapshot!(Animation::new(30, "file-id", "file-unique-id", 200, 200));
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
                ("allow_paid_broadcast", true.into()),
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
            .with_allow_paid_broadcast(true)
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

#[test]
fn audio() {
    insta::assert_json_snapshot!(
        Audio::new(243, "file-id", "file-unique-id")
            .with_file_name("File Name")
            .with_file_size(10240)
            .with_mime_type("audio/mpeg")
            .with_performer("Performer")
            .with_title("Title")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-unique-file-id", 24, 24).with_file_size(1024)),
    );
    insta::assert_json_snapshot!(Audio::new(243, "file-id", "file-unique-id"));
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
                ("allow_paid_broadcast", true.into()),
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
            .with_allow_paid_broadcast(true)
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

#[test]
fn document() {
    insta::assert_json_snapshot!(
        Document::new("file-id", "file-unique-id")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_name("File Name")
            .with_mime_type("image/jpeg")
            .with_file_size(10240),
    );
    insta::assert_json_snapshot!(Document::new("file-id", "file-unique-id"));
}

#[test]
fn send_document() {
    assert_payload_eq(
        Payload::form(
            "sendDocument",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("document", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendDocument::new(1, InputFile::file_id("file-id")),
    );
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendDocument",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("document", InputFile::file_id("file-id").into()),
                ("thumbnail", InputFile::url("https://example.com/image.jpg").into()),
                ("caption", "Caption".into()),
                ("allow_paid_broadcast", true.into()),
                ("business_connection_id", "id".into()),
                ("disable_content_type_detection", true.into()),
                ("parse_mode", ParseMode::Markdown.into()),
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
        SendDocument::new(1, InputFile::file_id("file-id"))
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_caption("Caption")
            .with_disable_content_type_detection(true)
            .with_disable_notification(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_parameters(reply_parameters)
            .unwrap()
            .with_thumbnail(InputFile::url("https://example.com/image.jpg"))
            .unwrap(),
    );
}

#[test]
fn send_document_with_thumbnail() {
    let err = SendDocument::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("file-id"))
        .unwrap_err();
    assert!(matches!(err, SendDocumentError::InvalidThumbnail));
}

#[test]
fn send_document_entities_vs_parse_mode() {
    assert_payload_eq(
        Payload::form(
            "sendDocument",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("document", InputFile::file_id("file-id").into()),
                ("parse_mode", "Markdown".into()),
            ]),
        ),
        SendDocument::new(1, InputFile::file_id("file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .with_caption_parse_mode(ParseMode::Markdown),
    );

    assert_payload_eq(
        Payload::form(
            "sendDocument",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("document", InputFile::file_id("file-id").into()),
                ("caption_entities", r#"[{"offset":0,"length":10,"type":"bold"}]"#.into()),
            ]),
        ),
        SendDocument::new(1, InputFile::file_id("file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
}

#[test]
fn photo_size() {
    insta::assert_json_snapshot!(PhotoSize::new("file-id", "file-unique-id", 200, 200).with_file_size(1024));
    insta::assert_json_snapshot!(PhotoSize::new("file-id", "file-unique-id", 200, 200));
}

#[test]
fn send_photo() {
    assert_payload_eq(
        Payload::form(
            "sendPhoto",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("photo", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendPhoto::new(1, InputFile::file_id("file-id")),
    );
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendPhoto",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("photo", InputFile::file_id("file-id").into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
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
            ]),
        ),
        SendPhoto::new(1, InputFile::file_id("file-id"))
            .with_allow_paid_broadcast(true)
            .with_caption("Caption")
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_has_spoiler(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_parameters(reply_parameters)
            .unwrap()
            .with_show_caption_above_media(true),
    );
}

#[test]
fn send_photo_entities_vs_parse_mode() {
    assert_payload_eq(
        Payload::form(
            "sendPhoto",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("photo", InputFile::file_id("file-id").into()),
                ("parse_mode", "Markdown".into()),
            ]),
        ),
        SendPhoto::new(1, InputFile::file_id("file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_payload_eq(
        Payload::form(
            "sendPhoto",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("photo", InputFile::file_id("file-id").into()),
                ("caption_entities", r#"[{"offset":0,"length":10,"type":"bold"}]"#.into()),
            ]),
        ),
        SendPhoto::new(1, InputFile::file_id("file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
}

#[test]
fn video() {
    insta::assert_json_snapshot!(
        Video::new(3, "file-id", "file-unique-id", 2, 1)
            .with_cover([PhotoSize::new("cover-file-id", "cover-file-unique-id", 24, 24)])
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_name("File Name")
            .with_mime_type("video/mpeg")
            .with_file_size(10240)
            .with_start_timestamp(20),
    );
    insta::assert_json_snapshot!(Video::new(3, "file-id", "file-unique-id", 2, 1));
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

#[test]
fn video_note() {
    insta::assert_json_snapshot!(
        VideoNote::new(1234, "file-id", "file-unique-id", 124)
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_size(10240)
    );
    insta::assert_json_snapshot!(VideoNote::new(1234, "file-id", "file-unique-id", 124));
}

#[test]
fn send_video_note() {
    assert_payload_eq(
        Payload::form(
            "sendVideoNote",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video_note", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendVideoNote::new(1, InputFile::file_id("file-id")),
    );
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendVideoNote",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video_note", InputFile::file_id("file-id").into()),
                ("duration", 50.into()),
                ("length", 100.into()),
                ("thumbnail", InputFile::url("https://example.com/image.jpg").into()),
                ("allow_paid_broadcast", true.into()),
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
        SendVideoNote::new(1, InputFile::file_id("file-id"))
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_duration(50)
            .with_length(100)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_parameters(reply_parameters)
            .unwrap()
            .with_thumbnail(InputFile::url("https://example.com/image.jpg"))
            .unwrap(),
    );
}

#[test]
fn send_video_note_with_thumbnail() {
    let err = SendVideoNote::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("thumb-file-id"))
        .unwrap_err();
    assert!(matches!(err, SendVideoNoteError::InvalidThumbnail));
}

#[test]
fn voice() {
    insta::assert_json_snapshot!(
        Voice::new(500, "file-id", "file-unique-id")
            .with_mime_type("audio/ogg")
            .with_file_size(40960)
    );
    insta::assert_json_snapshot!(Voice::new(500, "file-id", "file-unique-id"));
}

#[test]
fn send_voice() {
    assert_payload_eq(
        Payload::form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendVoice::new(1, InputFile::file_id("file-id")),
    );
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
                ("duration", 100.into()),
                ("allow_paid_broadcast", true.into()),
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
        SendVoice::new(1, InputFile::file_id("file-id"))
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_caption("Caption")
            .with_disable_notification(true)
            .with_duration(100)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_parameters(reply_parameters)
            .unwrap(),
    );
}

#[test]
fn send_voice_entities_vs_parse_mode() {
    assert_payload_eq(
        Payload::form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
                ("parse_mode", "Markdown".into()),
            ]),
        ),
        SendVoice::new(1, InputFile::file_id("file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_payload_eq(
        Payload::form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
                ("caption_entities", r#"[{"offset":0,"length":10,"type":"bold"}]"#.into()),
            ]),
        ),
        SendVoice::new(1, InputFile::file_id("file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
}

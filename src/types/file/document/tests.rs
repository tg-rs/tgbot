use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        Document,
        ForceReply,
        InputFile,
        ParseMode,
        PhotoSize,
        SendDocument,
        SendDocumentError,
        TextEntity,
    },
};

#[test]
fn document() {
    assert_json_eq(
        Document::new("file-id", "file-unique-id")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_name("File Name")
            .with_mime_type("image/jpeg")
            .with_file_size(10240),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "thumbnail": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-file-unique-id",
                "width": 24,
                "height": 24,
                "file_size": 1024
            },
            "file_name": "File Name",
            "mime_type": "image/jpeg",
            "file_size": 10240
        }),
    );
    assert_json_eq(
        Document::new("file-id", "file-unique-id"),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id"
        }),
    );
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
    assert_payload_eq(
        Payload::form(
            "sendDocument",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("document", InputFile::file_id("file-id").into()),
                ("thumbnail", InputFile::url("https://example.com/image.jpg").into()),
                ("caption", "Caption".into()),
                ("disable_content_type_detection", true.into()),
                ("parse_mode", ParseMode::Markdown.into()),
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
        SendDocument::new(1, InputFile::file_id("file-id"))
            .with_allow_sending_without_reply(true)
            .with_caption("Caption")
            .with_disable_content_type_detection(true)
            .with_disable_notification(true)
            .with_message_thread_id(1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_to_message_id(1)
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

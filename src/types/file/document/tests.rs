use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{tests::assert_json_eq, Document, ForceReply, InputFile, ParseMode, PhotoSize, SendDocument, TextEntity},
};

#[test]
fn document() {
    assert_json_eq(
        Document {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            thumbnail: Some(PhotoSize {
                file_id: String::from("thumb-file-id"),
                file_unique_id: String::from("thumb-file-unique-id"),
                width: 24,
                height: 24,
                file_size: Some(1024),
            }),
            file_name: Some(String::from("File Name")),
            mime_type: Some(String::from("image/jpeg")),
            file_size: Some(10240),
        },
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
        Document {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            thumbnail: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        },
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
                ("thumbnail", InputFile::file_id("file-id").into()),
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
            .thumbnail(InputFile::file_id("file-id"))
            .caption("Caption")
            .parse_mode(ParseMode::Markdown)
            .disable_content_type_detection(true)
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
fn send_document_caption_entities_vs_parse_mode() {
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
            .caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .parse_mode(ParseMode::Markdown),
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
            .parse_mode(ParseMode::Markdown)
            .caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
}

use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{tests::assert_json_eq, ForceReply, InputFile, ParseMode, PhotoSize, SendPhoto, TextEntity},
};

#[test]
fn photo_size() {
    assert_json_eq(
        PhotoSize {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            width: 200,
            height: 200,
            file_size: Some(1024),
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 200,
            "height": 200,
            "file_size": 1024
        }),
    );
    assert_json_eq(
        PhotoSize {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            width: 200,
            height: 200,
            file_size: None,
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 200,
            "height": 200,
        }),
    );
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
    assert_payload_eq(
        Payload::form(
            "sendPhoto",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("photo", InputFile::file_id("file-id").into()),
                ("caption", "Caption".into()),
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
                ("has_spoiler", true.into()),
            ]),
        ),
        SendPhoto::new(1, InputFile::file_id("file-id"))
            .caption("Caption")
            .parse_mode(ParseMode::Markdown)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .unwrap()
            .message_thread_id(1)
            .has_spoiler(true),
    );
}

#[test]
fn send_photo_caption_entities_vs_parse_mode() {
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
            .caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .parse_mode(ParseMode::Markdown),
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
            .parse_mode(ParseMode::Markdown)
            .caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
}

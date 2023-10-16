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
            .message_thread_id(1),
    );
}

#[test]
fn send_photo_caption_entities_vs_parse_mode() {
    let mut method = SendPhoto::new(1, InputFile::file_id("file-id"));

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

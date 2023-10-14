use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{tests::assert_json_eq, Audio, ForceReply, InputFile, ParseMode, PhotoSize, SendAudio, TextEntity},
};

#[test]
fn audio() {
    assert_json_eq(
        Audio {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            duration: 243,
            performer: Some(String::from("Performer")),
            title: Some(String::from("Title")),
            file_name: Some(String::from("File Name")),
            mime_type: Some(String::from("audio/mpeg")),
            file_size: Some(10240),
            thumb: Some(PhotoSize {
                file_id: String::from("thumb-file-id"),
                file_unique_id: String::from("thumb-unique-file-id"),
                width: 24,
                height: 24,
                file_size: Some(1024),
            }),
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "duration": 243,
            "performer": "Performer",
            "title": "Title",
            "file_name": "File Name",
            "mime_type": "audio/mpeg",
            "file_size": 10240,
            "thumb": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-unique-file-id",
                "width": 24,
                "height": 24,
                "file_size": 1024
            }
        }),
    );
    assert_json_eq(
        Audio {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            duration: 243,
            performer: None,
            title: None,
            file_name: None,
            mime_type: None,
            file_size: None,
            thumb: None,
        },
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
                ("thumb", InputFile::file_id("thumb-id").into()),
                ("disable_notification", true.into()),
                ("protect_content", true.into()),
                ("reply_to_message_id", 1.into()),
                ("allow_sending_without_reply", true.into()),
                (
                    "reply_markup",
                    serde_json::to_string(&ForceReply::new(true)).unwrap().into(),
                ),
            ]),
        ),
        SendAudio::new(1, InputFile::file_id("file-id"))
            .caption("Caption")
            .parse_mode(ParseMode::Markdown)
            .duration(100)
            .performer("Performer")
            .title("Title")
            .thumb(InputFile::file_id("thumb-id"))
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .unwrap(),
    );
}

#[test]
fn send_audio_caption_entities_vs_parse_mode() {
    let mut method = SendAudio::new(1, InputFile::file_id("file-id"));

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

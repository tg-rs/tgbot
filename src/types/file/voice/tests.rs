use crate::{
    form::{Form, FormValue},
    tests::{assert_json_eq, assert_request_eq, ExpectedRequest},
    types::{ForceReply, InputFile, ParseMode, SendVoice, TextEntity, Voice},
};

#[test]
fn voice() {
    assert_json_eq(
        Voice {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            duration: 500,
            mime_type: Some(String::from("audio/ogg")),
            file_size: Some(40960),
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "duration": 500,
            "mime_type": "audio/ogg",
            "file_size": 40960
        }),
    );
    assert_json_eq(
        Voice {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            duration: 500,
            mime_type: None,
            file_size: None,
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "duration": 500,
        }),
    );
}

#[test]
fn send_voice() {
    assert_request_eq(
        ExpectedRequest::post_form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendVoice::new(1, InputFile::file_id("file-id")),
    );
    assert_request_eq(
        ExpectedRequest::post_form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
                ("duration", 100.into()),
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
        SendVoice::new(1, InputFile::file_id("file-id"))
            .caption("Caption")
            .parse_mode(ParseMode::Markdown)
            .duration(100)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .unwrap(),
    );
}

#[test]
fn send_voice_caption_entities_vs_parse_mode() {
    let mut method = SendVoice::new(1, InputFile::file_id("file-id"));

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

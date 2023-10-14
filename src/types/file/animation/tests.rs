use crate::{
    form::{Form, FormValue},
    tests::{assert_json_eq, assert_request_eq, ExpectedRequest},
    types::{Animation, ForceReply, InputFile, ParseMode, PhotoSize, SendAnimation, TextEntity},
};

#[test]
fn animation() {
    assert_json_eq(
        Animation {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            width: 200,
            height: 200,
            duration: 243,
            thumb: Some(PhotoSize {
                file_id: String::from("thumb-file-id"),
                file_unique_id: String::from("thumb-file-unique-id"),
                width: 24,
                height: 24,
                file_size: Some(1024),
            }),
            file_name: Some(String::from("File Name")),
            mime_type: Some(String::from("image/gif")),
            file_size: Some(20480),
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 200,
            "height": 200,
            "duration": 243,
            "thumb": {
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
        Animation {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            width: 200,
            height: 200,
            duration: 30,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        },
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
    assert_request_eq(
        ExpectedRequest::post_form(
            "sendAnimation",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("animation", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendAnimation::new(1, InputFile::file_id("file-id")),
    );
    assert_request_eq(
        ExpectedRequest::post_form(
            "sendAnimation",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("animation", InputFile::file_id("file-id").into()),
                ("duration", 100.into()),
                ("width", 200.into()),
                ("height", 300.into()),
                ("thumb", InputFile::file_id("thumb-file-id").into()),
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
            ]),
        ),
        SendAnimation::new(1, InputFile::file_id("file-id"))
            .duration(100)
            .width(200)
            .height(300)
            .thumb(InputFile::file_id("thumb-file-id"))
            .caption("Caption")
            .parse_mode(ParseMode::Markdown)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .unwrap(),
    );
}

#[test]
fn send_animation_caption_entities_vs_parse_mode() {
    let mut method = SendAnimation::new(1, InputFile::file_id("file-id"));

    method = method.parse_mode(ParseMode::Markdown);
    assert_eq!(method.form.fields["parse_mode"].get_text().unwrap(), "Markdown");

    method = method.caption_entities(vec![TextEntity::bold(0..10)]).unwrap();
    assert!(!method.form.fields.contains_key("parse_mode"));

    let caption_entities = method.form.fields["caption_entities"].get_text().unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(caption_entities).unwrap(),
        serde_json::json!([{"type": "bold", "offset":0, "length": 10}])
    );

    method = method.parse_mode(ParseMode::Markdown);
    assert!(!method.form.fields.contains_key("caption_entities"));
}

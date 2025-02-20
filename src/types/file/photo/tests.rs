use crate::{
    api::{Form, FormValue, Payload, assert_payload_eq},
    types::{
        ForceReply,
        InputFile,
        ParseMode,
        PhotoSize,
        ReplyParameters,
        SendPhoto,
        TextEntity,
        tests::assert_json_eq,
    },
};

#[test]
fn photo_size() {
    assert_json_eq(
        PhotoSize::new("file-id", "file-unique-id", 200, 200).with_file_size(1024),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 200,
            "height": 200,
            "file_size": 1024
        }),
    );
    assert_json_eq(
        PhotoSize::new("file-id", "file-unique-id", 200, 200),
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

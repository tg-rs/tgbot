use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ForceReply, InputFile, ParseMode, PhotoSize, SendPhoto, TextEntity},
};

#[test]
fn photo_size_deserialize_full() {
    let data: PhotoSize = serde_json::from_value(serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id",
        "width": 200,
        "height": 200,
        "file_size": 1234
    }))
    .unwrap();
    assert_eq!(data.file_id, "file-id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.width, 200);
    assert_eq!(data.height, 200);
    assert_eq!(data.file_size.unwrap(), 1234);
}

#[test]
fn photo_size_deserialize_partial() {
    let data: PhotoSize = serde_json::from_value(serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id",
        "width": 200,
        "height": 200
    }))
    .unwrap();
    assert_eq!(data.file_id, "file-id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.width, 200);
    assert_eq!(data.height, 200);
    assert!(data.file_size.is_none());
}

#[test]
fn send_photo() {
    let request = SendPhoto::new(1, InputFile::file_id("file-id"))
        .caption("caption")
        .parse_mode(ParseMode::Markdown)
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .unwrap()
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendPhoto");
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert!(form.fields["photo"].get_file().is_some());
        assert_eq!(form.fields["caption"].get_text().unwrap(), "caption");
        assert_eq!(form.fields["parse_mode"].get_text().unwrap(), "Markdown");
        assert_eq!(form.fields["disable_notification"].get_text().unwrap(), "true");
        assert_eq!(form.fields["protect_content"].get_text().unwrap(), "true");
        assert_eq!(form.fields["reply_to_message_id"].get_text().unwrap(), "1");
        assert_eq!(form.fields["allow_sending_without_reply"].get_text().unwrap(), "true");
        assert_eq!(
            form.fields["reply_markup"].get_text().unwrap(),
            r#"{"force_reply":true}"#
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn send_photo_caption() {
    let mut method = SendPhoto::new(1, InputFile::file_id("file-id"));
    method = method.parse_mode(ParseMode::Markdown);
    assert_eq!(method.form.fields["parse_mode"].get_text().unwrap(), "Markdown");
    method = method.caption_entities(vec![TextEntity::bold(0..10)]).unwrap();
    assert!(!method.form.fields.contains_key("parse_mode"));
    let caption_entities = method.form.fields["caption_entities"].get_text().unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(caption_entities).unwrap(),
        serde_json::json!([{"type": "bold", "offset":0, "length": 10}])
    );
}

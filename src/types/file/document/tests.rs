use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{Document, ForceReply, InputFile, ParseMode, SendDocument, TextEntity},
};

#[test]
fn document_deserialize_full() {
    let data: Document = serde_json::from_value(serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id",
        "thumb": {
            "file_id": "file-thumb-id",
            "file_unique_id": "unique-thumb-id",
            "width": 24,
            "height": 24,
            "file_size": 12324
        },
        "file_name": "Test file name",
        "mime_type": "image/jpeg",
        "file_size": 1234
    }))
    .unwrap();

    assert_eq!(data.file_id, "file-id");
    assert_eq!(data.file_unique_id, "unique-id");

    let thumb = data.thumb.unwrap();
    assert_eq!(thumb.file_id, "file-thumb-id");
    assert_eq!(thumb.file_unique_id, "unique-thumb-id");
    assert_eq!(thumb.width, 24);
    assert_eq!(thumb.height, 24);
    assert_eq!(thumb.file_size.unwrap(), 12324);

    assert_eq!(data.file_name.unwrap(), "Test file name");
    assert_eq!(data.mime_type.unwrap(), "image/jpeg");
    assert_eq!(data.file_size.unwrap(), 1234);
}

#[test]
fn document_deserialize_partial() {
    let data: Document = serde_json::from_value(serde_json::json!({
        "file_id": "file-id",
        "file_unique_id": "unique-id"
    }))
    .unwrap();
    assert_eq!(data.file_id, "file-id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert!(data.file_name.is_none());
    assert!(data.thumb.is_none());
    assert!(data.mime_type.is_none());
    assert!(data.file_size.is_none());
}

#[test]
fn send_document() {
    let request = SendDocument::new(1, InputFile::file_id("file-id"))
        .thumb(InputFile::file_id("file-id"))
        .caption("caption")
        .parse_mode(ParseMode::Markdown)
        .disable_content_type_detection(true)
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .unwrap()
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendDocument");
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert!(form.fields["document"].get_file().is_some());
        assert!(form.fields["thumb"].get_file().is_some());
        assert_eq!(form.fields["caption"].get_text().unwrap(), "caption");
        assert_eq!(
            form.fields["disable_content_type_detection"].get_text().unwrap(),
            "true"
        );
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
fn send_document_caption() {
    let mut method = SendDocument::new(1, InputFile::file_id("file-id"));
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

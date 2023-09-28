use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ForceReply, InputFile, MaskPositionPoint, NewSticker, SendSticker, Sticker, UploadStickerFile},
};

#[test]
fn sticker_deserialize_full() {
    let data: Sticker = serde_json::from_value(serde_json::json!({
        "file_id": "test file id",
        "file_unique_id": "unique-id",
        "width": 512,
        "height": 512,
        "thumb": {
            "file_id": "AdddddUuUUUUccccUUmm_PPP",
            "file_unique_id": "unique-thumb-id",
            "width": 24,
            "height": 24,
            "file_size": 12324
        },
        "emoji": ":D",
        "set_name": "sticker set name",
        "mask_position": {
            "point": "forehead",
            "x_shift": 3.0,
            "y_shift": 2.0,
            "scale": 3.0,
        },
        "file_size": 1234,
        "is_animated": false,
        "is_video": false
    }))
    .unwrap();

    assert_eq!(data.file_id, "test file id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.width, 512);
    assert_eq!(data.height, 512);
    assert!(!data.is_animated);
    assert!(!data.is_video);

    let thumb = data.thumb.unwrap();
    assert_eq!(thumb.file_id, "AdddddUuUUUUccccUUmm_PPP");
    assert_eq!(thumb.file_unique_id, "unique-thumb-id");
    assert_eq!(thumb.width, 24);
    assert_eq!(thumb.height, 24);
    assert_eq!(thumb.file_size.unwrap(), 12324);

    assert_eq!(data.emoji.unwrap(), ":D");
    assert_eq!(data.set_name.unwrap(), "sticker set name");

    let mask_position = data.mask_position.unwrap();
    assert_eq!(mask_position.point, MaskPositionPoint::Forehead);
    assert_eq!(mask_position.x_shift, 3.0);
    assert_eq!(mask_position.y_shift, 2.0);
    assert_eq!(mask_position.scale, 3.0);

    assert_eq!(data.file_size.unwrap(), 1234);
}

#[test]
fn sticker_deserialize_partial() {
    let data: Sticker = serde_json::from_value(serde_json::json!({
        "file_id": "test file id",
        "file_unique_id": "unique-id",
        "width": 512,
        "height": 512,
        "is_animated": true,
        "is_video": false
    }))
    .unwrap();

    assert_eq!(data.file_id, "test file id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.width, 512);
    assert_eq!(data.height, 512);
    assert!(data.is_animated);
    assert!(!data.is_video);
    assert!(data.thumb.is_none());
    assert!(data.emoji.is_none());
    assert!(data.set_name.is_none());
    assert!(data.file_size.is_none());
}

#[test]
fn new_sticker_png() {
    let data = NewSticker::png(InputFile::file_id("id"));
    let repr = format!("{:?}", data);
    assert_eq!(
        repr,
        "NewSticker { kind: Png(InputFile { kind: InputFileKind::Id(\"id\") }) }"
    );
}

#[test]
fn new_sticker_tgs() {
    let data = NewSticker::tgs(InputFile::file_id("id"));
    let repr = format!("{:?}", data);
    assert_eq!(
        repr,
        "NewSticker { kind: Tgs(InputFile { kind: InputFileKind::Id(\"id\") }) }"
    );
}

#[test]
fn new_sticker_video() {
    let data = NewSticker::video(InputFile::file_id("id"));
    let repr = format!("{:?}", data);
    assert_eq!(
        repr,
        "NewSticker { kind: Video(InputFile { kind: InputFileKind::Id(\"id\") }) }"
    );
}

#[test]
fn send_sticker() {
    let request = SendSticker::new(1, InputFile::file_id("sticker-id"))
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .unwrap()
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendSticker");
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert!(form.fields["sticker"].get_file().is_some());
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
fn upload_sticker_file() {
    let request = UploadStickerFile::new(1, InputFile::file_id("sticker-id")).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/uploadStickerFile"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
        assert!(form.fields["png_sticker"].get_file().is_some());
    } else {
        panic!("Unexpected request body");
    }
}

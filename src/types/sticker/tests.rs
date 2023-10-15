use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        File,
        ForceReply,
        InputFile,
        MaskPosition,
        MaskPositionPoint,
        NewSticker,
        PhotoSize,
        ReplyMarkup,
        SendSticker,
        Sticker,
        UploadStickerFile,
    },
};

#[test]
fn sticker() {
    assert_json_eq(
        Sticker {
            file_id: String::from("test file id"),
            file_unique_id: String::from("unique-id"),
            width: 512,
            height: 512,
            thumb: Some(PhotoSize {
                file_id: String::from("file-id"),
                file_unique_id: String::from("unique-thumb-id"),
                width: 24,
                height: 24,
                file_size: Some(12324),
            }),
            emoji: Some(String::from(":D")),
            set_name: Some(String::from("sticker set name")),
            mask_position: Some(MaskPosition {
                point: MaskPositionPoint::Forehead,
                x_shift: 3.0,
                y_shift: 2.0,
                scale: 3.0,
            }),
            file_size: Some(1234),
            is_animated: false,
            is_video: false,
            premium_animation: Some(File {
                file_id: String::from("file-id"),
                file_unique_id: String::from("file-unique-id"),
                file_size: None,
                file_path: None,
            }),
        },
        serde_json::json!({
            "file_id": "test file id",
            "file_unique_id": "unique-id",
            "width": 512,
            "height": 512,
            "thumb": {
                "file_id": "file-id",
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
            "is_video": false,
            "premium_animation": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id"
            }
        }),
    );
    assert_json_eq(
        Sticker {
            file_id: String::from("test file id"),
            file_unique_id: String::from("unique-id"),
            width: 512,
            height: 512,
            thumb: None,
            emoji: None,
            set_name: None,
            mask_position: None,
            file_size: None,
            is_animated: false,
            is_video: false,
            premium_animation: None,
        },
        serde_json::json!({
            "file_id": "test file id",
            "file_unique_id": "unique-id",
            "width": 512,
            "height": 512,
            "is_animated": false,
            "is_video": false
        }),
    );
}

#[test]
fn new_sticker_png() {
    let data = NewSticker::png(InputFile::file_id("id"));
    let repr = format!("{:?}", data);
    assert_eq!(repr, "NewSticker { kind: Png(InputFile { kind: Id(\"id\") }) }");
}

#[test]
fn new_sticker_tgs() {
    let data = NewSticker::tgs(InputFile::file_id("id"));
    let repr = format!("{:?}", data);
    assert_eq!(repr, "NewSticker { kind: Tgs(InputFile { kind: Id(\"id\") }) }");
}

#[test]
fn new_sticker_video() {
    let data = NewSticker::video(InputFile::file_id("id"));
    let repr = format!("{:?}", data);
    assert_eq!(repr, "NewSticker { kind: Video(InputFile { kind: Id(\"id\") }) }");
}

#[test]
fn send_sticker() {
    let reply_markup = ReplyMarkup::from(ForceReply::new(true));
    assert_payload_eq(
        Payload::form(
            "sendSticker",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("sticker", InputFile::file_id("sticker-id").into()),
                ("disable_notification", true.into()),
                ("protect_content", true.into()),
                ("reply_to_message_id", 1.into()),
                ("allow_sending_without_reply", true.into()),
                ("reply_markup", reply_markup.serialize().unwrap().into()),
            ]),
        ),
        SendSticker::new(1, InputFile::file_id("sticker-id"))
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(reply_markup)
            .unwrap(),
    );
    assert_payload_eq(
        Payload::form(
            "sendSticker",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("sticker", InputFile::file_id("sticker-id").into()),
            ]),
        ),
        SendSticker::new(1, InputFile::file_id("sticker-id")),
    );
}

#[test]
fn upload_sticker_file() {
    assert_payload_eq(
        Payload::form(
            "uploadStickerFile",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("png_sticker", InputFile::file_id("sticker-id").into()),
            ]),
        ),
        UploadStickerFile::new(1, InputFile::file_id("sticker-id")),
    );
}

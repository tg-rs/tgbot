use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        AddStickerToSet,
        CreateNewStickerSet,
        DeleteStickerFromSet,
        GetStickerSet,
        InputFile,
        InputSticker,
        InputStickers,
        PhotoSize,
        SetStickerPositionInSet,
        SetStickerSetThumb,
        StickerFormat,
        StickerSet,
        StickerType,
    },
};

#[test]
fn sticker_set() {
    assert_json_eq(
        StickerSet {
            name: String::from("test"),
            title: String::from("test"),
            stickers: vec![],
            sticker_type: StickerType::Regular,
            is_animated: false,
            is_video: false,
            thumb: Some(PhotoSize {
                file_id: String::from("thumb-file-id"),
                file_unique_id: String::from("thumb-file-unique-id"),
                width: 512,
                height: 512,
                file_size: Some(2048),
            }),
        },
        serde_json::json!({
            "name": "test",
            "title": "test",
            "stickers": [],
            "sticker_type": "regular",
            "is_animated": false,
            "is_video": false,
            "thumb": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-file-unique-id",
                "width": 512,
                "height": 512,
                "file_size": 2048,
            }
        }),
    );
}

#[test]
fn add_sticker_to_set() {
    assert_payload_eq(
        Payload::form(
            "addStickerToSet",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("name", "name".into()),
                ("sticker", r#"{"sticker":"sticker-id","emoji_list":["😻"]}"#.into()),
            ]),
        ),
        AddStickerToSet::new(1, "name", InputSticker::new(InputFile::file_id("sticker-id"), ["😻"])).unwrap(),
    );
}

fn create_input_stickers() -> InputStickers {
    InputStickers::default().with(InputSticker::new(InputFile::file_id("sticker-file-id"), ["😻"]))
}

#[test]
fn create_new_sticker_set() {
    assert_payload_eq(
        Payload::form(
            "createNewStickerSet",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("name", "name".into()),
                ("title", "title".into()),
                (
                    "stickers",
                    r#"[{"sticker":"sticker-file-id","emoji_list":["😻"]}]"#.into(),
                ),
                ("needs_repainting", true.into()),
                ("sticker_type", "regular".into()),
                ("sticker_format", "static".into()),
            ]),
        ),
        CreateNewStickerSet::new(1, "name", "title", create_input_stickers(), StickerFormat::Static)
            .unwrap()
            .needs_repainting(true)
            .sticker_type(StickerType::Regular),
    );
    assert_payload_eq(
        Payload::form(
            "createNewStickerSet",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("name", "name".into()),
                ("title", "title".into()),
                (
                    "stickers",
                    r#"[{"sticker":"sticker-file-id","emoji_list":["😻"]}]"#.into(),
                ),
                ("sticker_format", "static".into()),
            ]),
        ),
        CreateNewStickerSet::new(1, "name", "title", create_input_stickers(), StickerFormat::Static).unwrap(),
    );
}

#[test]
fn delete_sticker_from_set() {
    assert_payload_eq(
        Payload::json(
            "deleteStickerFromSet",
            serde_json::json!({
                "sticker": "sticker"
            }),
        ),
        DeleteStickerFromSet::new("sticker"),
    );
}

#[test]
fn get_sticker_set() {
    assert_payload_eq(
        Payload::json(
            "getStickerSet",
            serde_json::json!({
                "name": "name"
            }),
        ),
        GetStickerSet::new("name"),
    );
}

#[test]
fn set_sticker_position_in_set() {
    assert_payload_eq(
        Payload::json(
            "setStickerPositionInSet",
            serde_json::json!({
                "sticker": "sticker",
                "position": 1
            }),
        ),
        SetStickerPositionInSet::new("sticker", 1),
    );
}

#[test]
fn set_sticker_set_thumb() {
    assert_payload_eq(
        Payload::form(
            "setStickerSetThumb",
            Form::from([
                ("name", FormValue::from("name")),
                ("user_id", 1.into()),
                ("thumb", InputFile::file_id("file-id").into()),
            ]),
        ),
        SetStickerSetThumb::new("name", 1).thumb(InputFile::file_id("file-id")),
    );
}

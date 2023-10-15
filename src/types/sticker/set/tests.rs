use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        AddStickerToSet,
        CreateNewStickerSet,
        DeleteStickerFromSet,
        GetStickerSet,
        InputFile,
        MaskPosition,
        MaskPositionPoint,
        NewSticker,
        PhotoSize,
        SetStickerPositionInSet,
        SetStickerSetThumb,
        StickerSet,
        StickerType,
    },
};

#[test]
fn sticker_type() {
    for (expected_struct, expected_value) in [
        (StickerType::CustomEmoji, serde_json::json!("custom_emoji")),
        (StickerType::Mask, serde_json::json!("mask")),
        (StickerType::Regular, serde_json::json!("regular")),
    ] {
        assert_json_eq(expected_struct, expected_value);
    }
}

#[test]
fn sticker_set() {
    assert_json_eq(
        StickerSet {
            name: String::from("test"),
            title: String::from("test"),
            contains_masks: false,
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
            "contains_masks": false,
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
    let mask_position = MaskPosition {
        point: MaskPositionPoint::Forehead,
        x_shift: 1.0,
        y_shift: 2.0,
        scale: 3.0,
    };
    assert_payload_eq(
        Payload::form(
            "addStickerToSet",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("name", "name".into()),
                ("png_sticker", InputFile::file_id("sticker-id").into()),
                ("emojis", "^_^".into()),
                ("mask_position", mask_position.serialize().unwrap().into()),
            ]),
        ),
        AddStickerToSet::new(1, "name", NewSticker::png(InputFile::file_id("sticker-id")), "^_^")
            .mask_position(mask_position)
            .unwrap(),
    );
    assert_payload_eq(
        Payload::form(
            "addStickerToSet",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("name", "name".into()),
                ("tgs_sticker", InputFile::file_id("sticker-id").into()),
                ("emojis", "^_^".into()),
            ]),
        ),
        AddStickerToSet::new(1, "name", NewSticker::tgs(InputFile::file_id("sticker-id")), "^_^"),
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
fn create_new_sticker_set() {
    let mask_position = MaskPosition {
        point: MaskPositionPoint::Forehead,
        x_shift: 1.0,
        y_shift: 2.0,
        scale: 3.0,
    };
    assert_payload_eq(
        Payload::form(
            "createNewStickerSet",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("name", "name".into()),
                ("title", "title".into()),
                ("png_sticker", InputFile::file_id("sticker-id").into()),
                ("emojis", "^_^".into()),
                ("mask_position", mask_position.serialize().unwrap().into()),
                ("contains_masks", true.into()),
            ]),
        ),
        CreateNewStickerSet::new(
            1,
            "name",
            "title",
            NewSticker::png(InputFile::file_id("sticker-id")),
            "^_^",
        )
        .contains_masks(true)
        .mask_position(mask_position)
        .unwrap(),
    );
    assert_payload_eq(
        Payload::form(
            "createNewStickerSet",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("name", "name".into()),
                ("title", "title".into()),
                ("tgs_sticker", InputFile::file_id("sticker-id").into()),
                ("emojis", "^_^".into()),
            ]),
        ),
        CreateNewStickerSet::new(
            1,
            "name",
            "title",
            NewSticker::tgs(InputFile::file_id("sticker-id")),
            "^_^",
        ),
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

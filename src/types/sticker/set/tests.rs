use crate::{
    api::{Form, FormValue, Payload, assert_payload_eq},
    types::{
        AddStickerToSet,
        CreateNewStickerSet,
        DeleteStickerFromSet,
        DeleteStickerSet,
        GetStickerSet,
        InputFile,
        InputSticker,
        InputStickers,
        PhotoSize,
        ReplaceStickerInSet,
        SetCustomEmojiStickerSetThumbnail,
        SetStickerPositionInSet,
        SetStickerSetThumbnail,
        SetStickerSetTitle,
        StickerFormat,
        StickerSet,
        StickerType,
        tests::assert_json_eq,
    },
};

#[test]
fn sticker_set() {
    assert_json_eq(
        StickerSet::new("test", StickerType::Regular, vec![], "test")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 512, 512).with_file_size(2048)),
        serde_json::json!({
            "name": "test",
            "title": "test",
            "stickers": [],
            "sticker_type": "regular",
            "thumbnail": {
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
                (
                    "sticker",
                    r#"{"sticker":"sticker-id","emoji_list":["😻"],"format":"static"}"#.into(),
                ),
            ]),
        ),
        AddStickerToSet::new(
            1,
            "name",
            InputSticker::new(InputFile::file_id("sticker-id"), ["😻"], StickerFormat::Static),
        )
        .unwrap(),
    );
}

fn create_input_stickers() -> InputStickers {
    InputStickers::default().add_sticker(InputSticker::new(
        InputFile::file_id("sticker-file-id"),
        ["😻"],
        StickerFormat::Static,
    ))
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
                    r#"[{"sticker":"sticker-file-id","emoji_list":["😻"],"format":"static"}]"#.into(),
                ),
                ("needs_repainting", true.into()),
                ("sticker_type", "regular".into()),
            ]),
        ),
        CreateNewStickerSet::new(1, "name", "title", create_input_stickers())
            .unwrap()
            .with_needs_repainting(true)
            .with_sticker_type(StickerType::Regular),
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
                    r#"[{"sticker":"sticker-file-id","emoji_list":["😻"],"format":"static"}]"#.into(),
                ),
            ]),
        ),
        CreateNewStickerSet::new(1, "name", "title", create_input_stickers()).unwrap(),
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
fn delete_sticker_set() {
    assert_payload_eq(
        Payload::json(
            "deleteStickerSet",
            serde_json::json!({
                "name": "test"
            }),
        ),
        DeleteStickerSet::new("test"),
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
fn replace_sticker_in_set() {
    let method = ReplaceStickerInSet::new(
        "test",
        "old-sticker",
        InputSticker::new(InputFile::file_id("test"), ["😻"], StickerFormat::Static),
        1,
    )
    .unwrap();
    assert_payload_eq(
        Payload::form(
            "replaceStickerInSet",
            Form::from([
                ("name", "test".into()),
                ("old_sticker", "old-sticker".into()),
                (
                    "sticker",
                    r#"{"sticker":"test","emoji_list":["😻"],"format":"static"}"#.into(),
                ),
                ("user_id", FormValue::from(1)),
            ]),
        ),
        method,
    );
}

#[test]
fn set_custom_emoji_sticker_set_thumbnail() {
    let method = SetCustomEmojiStickerSetThumbnail::new("test");
    assert_payload_eq(
        Payload::json(
            "setCustomEmojiStickerSetThumbnail",
            serde_json::json!({
                "name": "test"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setCustomEmojiStickerSetThumbnail",
            serde_json::json!({
                "name": "test",
                "custom_emoji_id": "emoji-id"
            }),
        ),
        method.with_custom_emoji_id("emoji-id"),
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
        SetStickerPositionInSet::new(1, "sticker"),
    );
}

#[test]
fn set_sticker_set_title() {
    assert_payload_eq(
        Payload::json(
            "setStickerSetTitle",
            serde_json::json!({
                "name": "test-name",
                "title": "test-title"
            }),
        ),
        SetStickerSetTitle::new("test-name", "test-title"),
    );
}

#[test]
fn set_sticker_set_thumbnail() {
    assert_payload_eq(
        Payload::form(
            "setStickerSetThumbnail",
            Form::from([
                ("name", FormValue::from("name")),
                ("user_id", 1.into()),
                ("format", "static".into()),
                ("thumbnail", InputFile::file_id("file-id").into()),
            ]),
        ),
        SetStickerSetThumbnail::new("name", 1, StickerFormat::Static).with_thumbnail(InputFile::file_id("file-id")),
    );
}

use std::io::Cursor;

use crate::{
    api::{Form, FormValue, Payload, assert_payload_eq},
    types::*,
};

#[test]
fn input_sticker() {
    let value = InputSticker::new(InputFile::file_id("file-id"), ["😻"], StickerFormat::Static);
    let form: Form = value.try_into().unwrap();
    assert_eq!(
        Form::from([(
            "sticker",
            r#"{"sticker":"file-id","emoji_list":["😻"],"format":"static"}"#.into()
        )]),
        form
    );

    let value = InputSticker::new(
        InputFile::url("https://google.com/favicon.ico"),
        ["😻"],
        StickerFormat::Static,
    )
    .with_keywords(["kw"])
    .with_mask_position(MaskPosition::new(MaskPositionPoint::Forehead, 3.0, 1.0, 2.0));
    let form: Form = value.try_into().unwrap();
    assert_eq!(
        Form::from([(
            "sticker",
            concat!(
                r#"{"sticker":"https://google.com/favicon.ico","emoji_list":["😻"],"format":"static","#,
                r#""mask_position":{"point":"forehead","scale":3.0,"x_shift":1.0,"y_shift":2.0},"#,
                r#""keywords":["kw"]}"#
            )
            .into()
        )]),
        form
    );

    let value = InputSticker::new(Cursor::new("test"), ["😻"], StickerFormat::Static);
    let form: Form = value.try_into().unwrap();
    assert_eq!(
        Form::from([
            ("tgbot_input_sticker", InputFile::from(Cursor::new("test")).into()),
            (
                "sticker",
                r#"{"sticker":"attach://tgbot_input_sticker","emoji_list":["😻"],"format":"static"}"#.into()
            )
        ]),
        form
    );
}

#[test]
fn input_stickers() {
    let value = InputStickers::default()
        .add_sticker(InputSticker::new(
            InputFile::file_id("file-id"),
            ["😻"],
            StickerFormat::Static,
        ))
        .add_sticker(InputSticker::new(
            InputFile::url("https://google.com/favicon.ico"),
            ["😻"],
            StickerFormat::Static,
        ));
    let form: Form = value.try_into().unwrap();
    assert_eq!(
        Form::from([(
            "stickers",
            concat!(
                r#"[{"sticker":"file-id","emoji_list":["😻"],"format":"static"},"#,
                r#"{"sticker":"https://google.com/favicon.ico","emoji_list":["😻"],"format":"static"}]"#
            )
            .into()
        )]),
        form
    );
}

#[test]
fn mask_position() {
    insta::assert_json_snapshot!(MaskPosition::new(MaskPositionPoint::Forehead, 1.0, 0.0, 1.0));
}

#[test]
fn mask_position_point() {
    use crate::types::MaskPositionPoint::*;
    for value in [Forehead, Eyes, Mouth, Chin] {
        insta::assert_json_snapshot!(value);
    }
}

#[test]
fn sticker_set() {
    insta::assert_json_snapshot!(
        StickerSet::new("test", StickerType::Regular, vec![], "test")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 512, 512).with_file_size(2048))
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

#[test]
fn sticker() {
    insta::assert_json_snapshot!(
        Sticker::new("test file id", "unique-id", StickerType::Regular, 512, 512)
            .with_thumbnail(PhotoSize::new("file-id", "unique-thumb-id", 24, 24).with_file_size(12324))
            .with_emoji(":D")
            .with_set_name("sticker set name")
            .with_mask_position(MaskPosition::new(MaskPositionPoint::Forehead, 3.0, 3.0, 2.0))
            .with_file_size(1234)
            .with_is_animated(false)
            .with_is_video(false)
            .with_premium_animation(File::new("file-id", "file-unique-id"))
            .with_custom_emoji_id(String::from("emoji-id"))
            .with_needs_repainting(true)
    );
    insta::assert_json_snapshot!(Sticker::new(
        "test file id",
        "unique-id",
        StickerType::Regular,
        512,
        512
    ));
}

#[test]
fn sticker_format() {
    for (expected_struct, str_value) in [
        (StickerFormat::Animated, "animated"),
        (StickerFormat::Static, "static"),
        (StickerFormat::Video, "video"),
    ] {
        insta::assert_json_snapshot!(expected_struct);
        assert_eq!(expected_struct.as_ref(), str_value);
    }
}

#[test]
fn sticker_type() {
    for (expected_struct, str_value) in [
        (StickerType::CustomEmoji, "custom_emoji"),
        (StickerType::Mask, "mask"),
        (StickerType::Regular, "regular"),
    ] {
        insta::assert_json_snapshot!(expected_struct);
        assert_eq!(expected_struct.as_ref(), str_value);
    }
}

#[test]
fn get_custom_emoji_stickers() {
    assert_payload_eq(
        Payload::json(
            "getCustomEmojiStickers",
            serde_json::json!({
                "custom_emoji_ids": ["emoji-id"]
            }),
        ),
        GetCustomEmojiStickers::new(["emoji-id"]),
    )
}

#[test]
fn send_sticker() {
    let reply_markup = ReplyMarkup::from(ForceReply::new(true));
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendSticker",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("sticker", InputFile::file_id("sticker-id").into()),
                ("allow_paid_broadcast", true.into()),
                ("business_connection_id", "id".into()),
                ("disable_notification", true.into()),
                ("emoji", "😱".into()),
                ("message_effect_id", "effect-id".into()),
                ("message_thread_id", 1.into()),
                ("protect_content", true.into()),
                ("reply_markup", reply_markup.serialize().unwrap().into()),
                ("reply_parameters", reply_parameters.serialize().unwrap().into()),
            ]),
        ),
        SendSticker::new(1, InputFile::file_id("sticker-id"))
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_emoji("😱")
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(reply_markup)
            .unwrap()
            .with_reply_parameters(reply_parameters)
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
fn set_sticker_emoji_list() {
    assert_payload_eq(
        Payload::json(
            "setStickerEmojiList",
            serde_json::json!({
                "sticker": "file-id",
                "emoji_list": ["✌️"]
            }),
        ),
        SetStickerEmojiList::new("file-id", ["✌️"]),
    );
}

#[test]
fn set_sticker_keywords() {
    assert_payload_eq(
        Payload::json(
            "setStickerKeywords",
            serde_json::json!({
                "sticker": "file-id",
                "keywords": ["kw"]
            }),
        ),
        SetStickerKeywords::new("file-id", ["kw"]),
    );
}

#[test]
fn set_sticker_mask_position() {
    let method = SetStickerMaskPosition::new("file-id");
    assert_payload_eq(
        Payload::json(
            "setStickerMaskPosition",
            serde_json::json!({
                "sticker": "file-id",
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setStickerMaskPosition",
            serde_json::json!({
                "sticker": "file-id",
                "mask_position": {
                    "point": "forehead",
                    "x_shift": 0.0,
                    "y_shift": 0.0,
                    "scale": 0.0
                }
            }),
        ),
        method.with_mask_position(MaskPosition::new(MaskPositionPoint::Forehead, 0.0, 0.0, 0.0)),
    )
}

#[test]
fn upload_sticker_file() {
    assert_payload_eq(
        Payload::form(
            "uploadStickerFile",
            Form::from([
                ("user_id", FormValue::from(1)),
                ("sticker", InputFile::file_id("sticker-id").into()),
                ("sticker_format", "static".into()),
            ]),
        ),
        UploadStickerFile::new(1, InputFile::file_id("sticker-id"), StickerFormat::Static),
    );
}

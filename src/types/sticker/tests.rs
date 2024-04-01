use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        File,
        ForceReply,
        GetCustomEmojiStickers,
        InputFile,
        MaskPosition,
        MaskPositionPoint,
        PhotoSize,
        ReplyMarkup,
        ReplyParameters,
        SendSticker,
        SetStickerEmojiList,
        SetStickerKeywords,
        SetStickerMaskPosition,
        Sticker,
        StickerFormat,
        StickerType,
        UploadStickerFile,
    },
};

#[test]
fn sticker() {
    assert_json_eq(
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
            .with_needs_repainting(true),
        serde_json::json!({
            "file_id": "test file id",
            "file_unique_id": "unique-id",
            "type": "regular",
            "width": 512,
            "height": 512,
            "thumbnail": {
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
            },
            "custom_emoji_id": "emoji-id",
            "needs_repainting": true,
        }),
    );
    assert_json_eq(
        Sticker::new("test file id", "unique-id", StickerType::Regular, 512, 512),
        serde_json::json!({
            "file_id": "test file id",
            "file_unique_id": "unique-id",
            "type": "regular",
            "width": 512,
            "height": 512,
            "is_animated": false,
            "is_video": false
        }),
    );
}

#[test]
fn sticker_format() {
    for (expected_struct, str_value) in [
        (StickerFormat::Animated, "animated"),
        (StickerFormat::Static, "static"),
        (StickerFormat::Video, "video"),
    ] {
        let expected_value = serde_json::json!(str_value);
        assert_json_eq(expected_struct, expected_value);
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
        let expected_value = serde_json::json!(str_value);
        assert_json_eq(expected_struct, expected_value);
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
                ("business_connection_id", "id".into()),
                ("disable_notification", true.into()),
                ("emoji", "😱".into()),
                ("message_thread_id", 1.into()),
                ("protect_content", true.into()),
                ("reply_markup", reply_markup.serialize().unwrap().into()),
                ("reply_parameters", reply_parameters.serialize().unwrap().into()),
            ]),
        ),
        SendSticker::new(1, InputFile::file_id("sticker-id"))
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_emoji("😱")
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

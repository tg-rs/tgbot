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
        SendSticker,
        Sticker,
        StickerFormat,
        StickerType,
        UploadStickerFile,
    },
};

#[test]
fn sticker() {
    assert_json_eq(
        Sticker {
            file_id: String::from("test file id"),
            file_unique_id: String::from("unique-id"),
            sticker_type: StickerType::Regular,
            width: 512,
            height: 512,
            thumbnail: Some(PhotoSize {
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
            custom_emoji_id: Some(String::from("emoji-id")),
            needs_repainting: Some(true),
        },
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
        Sticker {
            file_id: String::from("test file id"),
            file_unique_id: String::from("unique-id"),
            sticker_type: StickerType::Regular,
            width: 512,
            height: 512,
            thumbnail: None,
            emoji: None,
            set_name: None,
            mask_position: None,
            file_size: None,
            is_animated: false,
            is_video: false,
            premium_animation: None,
            custom_emoji_id: None,
            needs_repainting: None,
        },
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
    assert_payload_eq(
        Payload::form(
            "sendSticker",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("sticker", InputFile::file_id("sticker-id").into()),
                ("allow_sending_without_reply", true.into()),
                ("disable_notification", true.into()),
                ("emoji", "😱".into()),
                ("message_thread_id", 1.into()),
                ("protect_content", true.into()),
                ("reply_markup", reply_markup.serialize().unwrap().into()),
                ("reply_to_message_id", 1.into()),
            ]),
        ),
        SendSticker::new(1, InputFile::file_id("sticker-id"))
            .allow_sending_without_reply(true)
            .disable_notification(true)
            .emoji("😱")
            .message_thread_id(1)
            .protect_content(true)
            .reply_markup(reply_markup)
            .unwrap()
            .reply_to_message_id(1),
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
                ("sticker", InputFile::file_id("sticker-id").into()),
                ("sticker_format", "static".into()),
            ]),
        ),
        UploadStickerFile::new(1, InputFile::file_id("sticker-id"), StickerFormat::Static),
    );
}

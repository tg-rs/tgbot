use std::io::Cursor;

use crate::types::*;

#[test]
fn input_sticker() {
    let value = InputSticker::new(InputFile::file_id("file-id"), ["😻"], StickerFormat::Static);
    assert_write_form_eq!(value; serialize);

    let value = InputSticker::new(
        InputFile::url("https://google.com/favicon.ico"),
        ["😻"],
        StickerFormat::Static,
    )
    .with_keywords(["kw"])
    .with_mask_position(MaskPosition::new(MaskPositionPoint::Forehead, 3.0, 1.0, 2.0));
    assert_write_form_eq!(value; serialize);

    let value = InputSticker::new(Cursor::new("test"), ["😻"], StickerFormat::Static);
    assert_write_form_eq!(value; serialize);
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
    let method = AddStickerToSet::new(
        1,
        "name",
        InputSticker::new(InputFile::file_id("sticker-id"), ["😻"], StickerFormat::Static),
    );
    assert_payload_eq!(POST FORM "addStickerToSet" => method);
}

fn create_input_stickers() -> Vec<InputSticker> {
    vec![InputSticker::new(
        InputFile::file_id("sticker-file-id"),
        ["😻"],
        StickerFormat::Static,
    )]
}

#[test]
fn create_new_sticker_set() {
    let method = CreateNewStickerSet::new(1, "name", "title", create_input_stickers())
        .with_needs_repainting(true)
        .with_sticker_type(StickerType::Regular);
    assert_payload_eq!(POST FORM "createNewStickerSet" => method);
    let method = CreateNewStickerSet::new(1, "name", "title", create_input_stickers());
    assert_payload_eq!(POST FORM "createNewStickerSet" => method);
}

#[test]
fn delete_sticker_from_set() {
    let method = DeleteStickerFromSet::new("sticker");
    assert_payload_eq!(POST JSON "deleteStickerFromSet" => method);
}

#[test]
fn delete_sticker_set() {
    let method = DeleteStickerSet::new("test");
    assert_payload_eq!(POST JSON "deleteStickerSet" => method);
}

#[test]
fn get_sticker_set() {
    let method = GetStickerSet::new("name");
    assert_payload_eq!(POST JSON "getStickerSet" => method);
}

#[test]
fn replace_sticker_in_set() {
    let method = ReplaceStickerInSet::new(
        "test",
        "old-sticker",
        InputSticker::new(InputFile::file_id("test"), ["😻"], StickerFormat::Static),
        1,
    );
    assert_payload_eq!(POST FORM "replaceStickerInSet" => method);
}

#[test]
fn set_custom_emoji_sticker_set_thumbnail() {
    let method = SetCustomEmojiStickerSetThumbnail::new("test");
    assert_payload_eq!(POST JSON "setCustomEmojiStickerSetThumbnail" => method.clone());
    let method = method.with_custom_emoji_id("emoji-id");
    assert_payload_eq!(POST JSON "setCustomEmojiStickerSetThumbnail" => method);
}

#[test]
fn set_sticker_position_in_set() {
    let method = SetStickerPositionInSet::new(1, "sticker");
    assert_payload_eq!(POST JSON "setStickerPositionInSet" => method);
}

#[test]
fn set_sticker_set_title() {
    let method = SetStickerSetTitle::new("test-name", "test-title");
    assert_payload_eq!(POST JSON "setStickerSetTitle" => method);
}

#[test]
fn set_sticker_set_thumbnail() {
    let method =
        SetStickerSetThumbnail::new("name", 1, StickerFormat::Static).with_thumbnail(InputFile::file_id("file-id"));
    assert_payload_eq!(POST FORM "setStickerSetThumbnail" => method);
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
    let method = GetCustomEmojiStickers::new(["emoji-id"]);
    assert_payload_eq!(POST JSON "getCustomEmojiStickers" => method);
}

#[test]
fn send_sticker() {
    let method = SendSticker::new(1, InputFile::file_id("sticker-id"))
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_callback_query_id("cqid")
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_emoji("😱")
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_receiver_user_id(999)
        .with_reply_markup(ReplyMarkup::from(ForceReply::new(true)))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_suggested_post_parameters(SuggestedPostParameters::default());
    assert_payload_eq!(POST FORM "sendSticker" => method);
    let method = SendSticker::new(1, InputFile::file_id("sticker-id"));
    assert_payload_eq!(POST FORM "sendSticker" => method);
}

#[test]
fn set_sticker_emoji_list() {
    let method = SetStickerEmojiList::new("file-id", ["✌️"]);
    assert_payload_eq!(POST JSON "setStickerEmojiList" => method);
}

#[test]
fn set_sticker_keywords() {
    let method = SetStickerKeywords::new("file-id", ["kw"]);
    assert_payload_eq!(POST JSON "setStickerKeywords" => method);
}

#[test]
fn set_sticker_mask_position() {
    let method = SetStickerMaskPosition::new("file-id");
    assert_payload_eq!(POST JSON "setStickerMaskPosition" => method.clone());
    let method = method.with_mask_position(MaskPosition::new(MaskPositionPoint::Forehead, 0.0, 0.0, 0.0));
    assert_payload_eq!(POST JSON "setStickerMaskPosition" => method);
}

#[test]
fn upload_sticker_file() {
    let method = UploadStickerFile::new(1, InputFile::file_id("sticker-id"), StickerFormat::Static);
    assert_payload_eq!(POST FORM "uploadStickerFile" => method);
}

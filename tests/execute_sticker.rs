#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-sticker-base",
            SendSticker::new(1, InputFile::file_id("sticker-id")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Sticker(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "file-unique-id");
                    assert_eq!(data.sticker_type, StickerType::Regular);
                    assert_eq!(data.width, 512);
                    assert_eq!(data.height, 512);
                    assert!(!data.is_animated);
                    assert!(!data.is_video);
                    assert!(data.thumbnail.is_none());
                    assert!(data.emoji.is_none());
                    assert!(data.set_name.is_none());
                    assert!(data.premium_animation.is_none());
                    assert!(data.mask_position.is_none());
                    assert!(data.custom_emoji_id.is_none());
                    assert!(data.needs_repainting.is_none());
                    assert!(data.file_size.is_none());
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-sticker-all",
            SendSticker::new(1, InputFile::file_id("sticker-id"))
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
                .with_suggested_post_parameters(SuggestedPostParameters::default()),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Sticker(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "file-unique-id");
                    assert_eq!(data.sticker_type, StickerType::Regular);
                    assert_eq!(data.width, 512);
                    assert_eq!(data.height, 512);
                    assert!(!data.is_animated);
                    assert!(!data.is_video);
                    assert!(data.thumbnail.is_some());
                    assert_eq!(data.emoji.unwrap(), "test");
                    assert_eq!(data.set_name.unwrap(), "test");
                    let file = data.premium_animation.unwrap();
                    assert_eq!(file.file_id, "fid");
                    let position = data.mask_position.unwrap();
                    assert_eq!(position.point, MaskPositionPoint::Forehead);
                    assert_eq!(position.x_shift, 0.0);
                    assert_eq!(position.y_shift, 0.0);
                    assert_eq!(position.scale, 1.0);
                    assert_eq!(data.custom_emoji_id.unwrap(), "test");
                    assert!(data.needs_repainting.unwrap());
                    assert_eq!(data.file_size.unwrap(), 10000);
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
    ])
    .await;

    cx.execute_batch([(
        "add-sticker-to-set",
        AddStickerToSet::new(
            1,
            "name",
            InputSticker::new(InputFile::file_id("sticker-id"), ["😻"], StickerFormat::Static),
        ),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([
        (
            "create-new-sticker-set-base",
            CreateNewStickerSet::new(
                1,
                "name",
                "title",
                vec![InputSticker::new(
                    InputFile::file_id("sticker-file-id"),
                    ["😻"],
                    StickerFormat::Static,
                )],
            ),
            |x| assert!(x),
        ),
        (
            "create-new-sticker-set-all",
            CreateNewStickerSet::new(
                1,
                "name",
                "title",
                [
                    InputSticker::new(InputFile::file_id("sticker-file-id"), ["😻"], StickerFormat::Static),
                    InputSticker::new(InputFile::file_id("sticker-file-id"), ["😻"], StickerFormat::Static)
                        .with_keywords(["test"])
                        .with_mask_position(MaskPosition::new(MaskPositionPoint::Chin, 1.0, 0.0, 3.0)),
                ],
            )
            .with_needs_repainting(true)
            .with_sticker_type(StickerType::Regular),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([("delete-sticker-from-set", DeleteStickerFromSet::new("sticker"), |x| {
        assert!(x)
    })])
    .await;

    cx.execute_batch([("delete-sticker-set", DeleteStickerSet::new("test"), |x| assert!(x))])
        .await;

    cx.execute_batch([
        ("get-sticker-set-regular", GetStickerSet::new("name"), |x| {
            assert_eq!(x.name, "name");
            assert_eq!(x.title, "test");
            assert!(matches!(x.sticker_type, StickerType::Regular));
            assert_eq!(x.stickers.len(), 0);
            assert!(x.thumbnail.is_none());
        }),
        ("get-sticker-set-mask", GetStickerSet::new("name"), |x| {
            assert_eq!(x.name, "name");
            assert_eq!(x.title, "test");
            assert!(matches!(x.sticker_type, StickerType::Mask));
            assert_eq!(x.stickers.len(), 0);
            assert!(x.thumbnail.is_none());
        }),
        ("get-sticker-set-custom-emoji", GetStickerSet::new("name"), |x| {
            assert_eq!(x.name, "name");
            assert_eq!(x.title, "test");
            assert!(matches!(x.sticker_type, StickerType::CustomEmoji));
            assert_eq!(x.stickers.len(), 1);
            let thumb = x.thumbnail.unwrap();
            assert_eq!(thumb.file_id, "fid");
        }),
    ])
    .await;

    cx.execute_batch([(
        "replace-sticker-in-set",
        {
            ReplaceStickerInSet::new(
                "test",
                "old-sticker",
                InputSticker::new(InputFile::file_id("test"), ["😻"], StickerFormat::Static),
                1,
            )
        },
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "set-custom-emoji-sticker-set-thumbnail-base",
        SetCustomEmojiStickerSetThumbnail::new("test"),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "set-custom-emoji-sticker-set-thumbnail-all",
        SetCustomEmojiStickerSetThumbnail::new("test").with_custom_emoji_id("emoji-id"),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "set-sticker-position-in-set",
        SetStickerPositionInSet::new(1, "sticker"),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "set-sticker-set-title",
        SetStickerSetTitle::new("test-name", "test-title"),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "set-sticker-set-thumbnail",
        SetStickerSetThumbnail::new("name", 1, StickerFormat::Static).with_thumbnail(InputFile::file_id("file-id")),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "get-custom-emoji-stickers",
        GetCustomEmojiStickers::new(["emoji-id"]),
        |x| assert_eq!(x.len(), 1),
    )])
    .await;

    cx.execute_batch([(
        "set-sticker-emoji-list",
        SetStickerEmojiList::new("file-id", ["✌️"]),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "set-sticker-keywords",
        SetStickerKeywords::new("file-id", ["kw"]),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([
        (
            "set-sticker-mask-position-base",
            SetStickerMaskPosition::new("file-id"),
            |x| assert!(x),
        ),
        (
            "set-sticker-mask-position-all",
            SetStickerMaskPosition::new("file-id").with_mask_position(MaskPosition::new(
                MaskPositionPoint::Forehead,
                0.0,
                0.0,
                0.0,
            )),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([(
        "upload-sticker-file",
        UploadStickerFile::new(1, InputFile::file_id("sticker-id"), StickerFormat::Static),
        |x| {
            assert_eq!(x.file_id, "fid");
            assert_eq!(x.file_unique_id, "fuid");
            assert_eq!(x.file_size.unwrap(), 10000);
            assert_eq!(x.file_path.unwrap(), "fpath");
        },
    )])
    .await;
}

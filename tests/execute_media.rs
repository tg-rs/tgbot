#![allow(missing_docs)]
use std::io::Cursor;

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-media-group-base",
            SendMediaGroup::new(
                1,
                MediaGroup::new(vec![
                    InputMediaDocument::from(InputFileReader::from(Cursor::new("document-1"))),
                    InputMediaDocument::from(InputFileReader::from(Cursor::new("document-2"))),
                ])
                .unwrap(),
            ),
            |x| {
                assert_eq!(x.len(), 2);
                for i in x {
                    assert!(matches!(i.data, MessageData::Document(_)));
                }
            },
        ),
        (
            "send-media-group-all",
            SendMediaGroup::new(
                1,
                MediaGroup::new(vec![
                    MediaGroupItem::from(InputMediaAudio::from(InputFileReader::from(Cursor::new("test-audio")))),
                    MediaGroupItem::from(InputMediaDocument::from(InputFileReader::from(Cursor::new(
                        "test-document",
                    )))),
                    MediaGroupItem::from(InputMediaLivePhoto::from((
                        InputFile::file_id("file-id"),
                        InputFile::file_id("photo-id"),
                    ))),
                    MediaGroupItem::from(InputMediaPhoto::from(InputFileReader::from(Cursor::new(
                        "test-photo-1",
                    )))),
                    MediaGroupItem::from(InputMediaPhoto::from(Cursor::new("test-photo-2")).with_caption("caption")),
                    MediaGroupItem::from(
                        InputMediaVideo::from(InputFileReader::from(Cursor::new("test-video")))
                            .with_cover(InputFile::url("cover-url")),
                    ),
                    MediaGroupItem::from(
                        InputMediaAudio::from(InputFile::file_id("file-id"))
                            .with_thumbnail(InputFile::url("thumb-url")),
                    ),
                    MediaGroupItem::from(
                        InputMediaDocument::from(InputFile::file_id("file-id"))
                            .with_thumbnail(InputFile::url("thumb-url")),
                    ),
                    MediaGroupItem::from(
                        InputMediaVideo::from(InputFile::file_id("file-id"))
                            .with_thumbnail(InputFile::url("thumb-url")),
                    ),
                ])
                .unwrap(),
            )
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_direct_messages_topic_id(1)
            .with_disable_notification(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_parameters(ReplyParameters::new(1)),
            |x| {
                assert_eq!(x.len(), 9);
                assert!(matches!(x[0].data, MessageData::Audio(_)));
                assert!(matches!(x[1].data, MessageData::Document(_)));
                assert!(matches!(x[2].data, MessageData::LivePhoto(_)));
                assert!(matches!(x[3].data, MessageData::Photo(_)));
                assert!(matches!(x[4].data, MessageData::Photo(_)));
                assert!(matches!(x[5].data, MessageData::Video(_)));
                assert!(matches!(x[6].data, MessageData::Audio(_)));
                assert!(matches!(x[7].data, MessageData::Document(_)));
                assert!(matches!(x[8].data, MessageData::Video(_)));
            },
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "send-paid-media-base",
            SendPaidMedia::new(
                1,
                InputPaidMediaGroup::new([InputPaidMediaPhoto::from(InputFile::file_id("file-id"))]).unwrap(),
                100,
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::PaidMedia(data) = x.data {
                    assert_eq!(data.star_count, 100);
                    assert_eq!(data.paid_media.len(), 1);
                    assert!(matches!(data.paid_media[0], PaidMedia::Photo(_)));
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-paid-media-all",
            SendPaidMedia::new(
                1,
                InputPaidMediaGroup::new([
                    InputPaidMedia::from(InputPaidMediaPhoto::from(InputFile::file_id("file-id"))),
                    InputPaidMedia::from(InputPaidMediaLivePhoto::from((
                        InputFile::file_id("media-id"),
                        InputFile::file_id("photo-id"),
                    ))),
                    InputPaidMedia::from(InputPaidMediaVideo::from(Cursor::new(b"test-video-1"))),
                    InputPaidMedia::from(
                        InputPaidMediaVideo::from(Cursor::new(b"test-video-2"))
                            .with_cover(InputFile::file_id("test-cover"))
                            .with_duration(10)
                            .with_height(200)
                            .with_start_timestamp(0)
                            .with_supports_streaming(true)
                            .with_thumbnail(InputFile::file_id("test-thumbnail"))
                            .with_width(200),
                    ),
                ])
                .unwrap(),
                100,
            )
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("c-id")
            .with_caption(("caption", [TextEntity::bold(0..1)]))
            .with_direct_messages_topic_id(1)
            .with_disable_notification(true)
            .with_message_thread_id(1)
            .with_payload("payload")
            .with_protect_content(true)
            .with_reply_parameters(ReplyParameters::new(1))
            .with_reply_markup(ForceReply::new(true))
            .with_suggested_post_parameters(SuggestedPostParameters::default())
            .with_show_caption_above_media(true),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::PaidMedia(data) = x.data {
                    assert_eq!(data.star_count, 100);
                    assert_eq!(data.paid_media.len(), 4);
                    assert!(matches!(data.paid_media[0], PaidMedia::Photo(_)));
                    assert!(matches!(data.paid_media[1], PaidMedia::LivePhoto(_)));
                    assert!(matches!(data.paid_media[2], PaidMedia::Video(_)));
                    assert!(matches!(data.paid_media[3], PaidMedia::Preview(_)));
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;
}

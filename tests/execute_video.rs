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
            "send-video-base",
            SendVideo::new(1, InputFile::file_id("file-id")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Video(video) = x.data {
                    assert_eq!(video.data.file_id, "file-id");
                    assert_eq!(video.data.file_unique_id, "file-unique-id");
                    assert_eq!(video.data.width, 200);
                    assert_eq!(video.data.height, 200);
                    assert_eq!(video.data.duration, 10);
                    assert!(video.data.thumbnail.is_none());
                    assert!(video.data.cover.is_none());
                    assert!(video.data.start_timestamp.is_none());
                    assert!(video.data.qualities.is_none());
                    assert!(video.data.file_name.is_none());
                    assert!(video.data.mime_type.is_none());
                    assert!(video.data.file_size.is_none());
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-video-all",
            SendVideo::new(1, InputFile::file_id("file-id"))
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_caption(("Caption", ParseMode::Markdown))
                .with_cover(InputFile::file_id("cover-id"))
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_ephemeral_message_parameters(999)
                .with_duration(100)
                .with_has_spoiler(true)
                .with_height(300)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_reply_markup(
                    ForceReply::from(true)
                        .with_input_field_placeholder("test")
                        .with_selective(true),
                )
                .with_reply_parameters(
                    ReplyParameters::new(1).with_quote(ReplyQuote::new(0, ("test", ParseMode::Markdown))),
                )
                .with_show_caption_above_media(true)
                .with_start_timestamp(20)
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_supports_streaming(true)
                .with_thumbnail_file(Cursor::new(b"file-data"))
                .with_thumbnail_url("https://example.com/image.jpg")
                .with_width(200),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Video(video) = x.data {
                    assert_eq!(video.data.file_id, "file-id");
                    assert_eq!(video.data.file_unique_id, "file-unique-id");
                    assert_eq!(video.data.width, 200);
                    assert_eq!(video.data.height, 200);
                    assert_eq!(video.data.duration, 10);
                    assert_eq!(video.data.start_timestamp.unwrap(), 0);
                    assert_eq!(video.data.file_name.unwrap(), "test");
                    assert_eq!(video.data.mime_type.unwrap(), "video/mp4");
                    assert_eq!(video.data.file_size.unwrap(), 10000);
                    let thumbnail = video.data.thumbnail.unwrap();
                    assert_eq!(thumbnail.file_id, "tfid");
                    let cover = video.data.cover.unwrap();
                    assert_eq!(cover.len(), 1);
                    let qualities = video.data.qualities.unwrap();
                    assert_eq!(qualities.len(), 1);
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-video-thumbnail-file",
            SendVideo::new(1, InputFile::file_id("file-id"))
                .with_thumbnail_url("https://example.com/image.jpg")
                .with_thumbnail_file(Cursor::new(b"file-data")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Video(video) = x.data {
                    assert_eq!(video.data.file_id, "file-id");
                    assert_eq!(video.data.file_unique_id, "file-unique-id");
                    assert_eq!(video.data.width, 200);
                    assert_eq!(video.data.height, 200);
                    assert_eq!(video.data.duration, 10);
                    assert!(video.data.thumbnail.is_some());
                    assert!(video.data.cover.is_none());
                    assert!(video.data.start_timestamp.is_none());
                    assert!(video.data.qualities.is_none());
                    assert!(video.data.file_name.is_none());
                    assert!(video.data.mime_type.is_none());
                    assert!(video.data.file_size.is_none());
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-video-caption-entities",
            SendVideo::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from((String::from("caption"), ParseMode::Html))
                    .with_format(ParseMode::Markdown)
                    .with_format([TextEntity::bold(0..10)]),
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Video(video) = x.data {
                    assert_eq!(video.data.file_id, "file-id");
                    assert_eq!(video.data.file_unique_id, "file-unique-id");
                    assert_eq!(video.data.width, 200);
                    assert_eq!(video.data.height, 200);
                    assert_eq!(video.data.duration, 10);
                    assert!(video.data.thumbnail.is_none());
                    assert!(video.data.cover.is_none());
                    assert!(video.data.start_timestamp.is_none());
                    assert!(video.data.qualities.is_none());
                    assert!(video.data.file_name.is_none());
                    assert!(video.data.mime_type.is_none());
                    assert!(video.data.file_size.is_none());
                    let caption = video.caption.unwrap();
                    assert_eq!(caption.data, "caption");
                    assert_eq!(caption.entities.unwrap().into_iter().len(), 1);
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-video-caption-parse-mode",
            SendVideo::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format([TextEntity::bold(0..10)])
                    .with_format(ParseMode::Markdown),
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Video(video) = x.data {
                    assert_eq!(video.data.file_id, "file-id");
                    assert_eq!(video.data.file_unique_id, "file-unique-id");
                    assert_eq!(video.data.width, 200);
                    assert_eq!(video.data.height, 200);
                    assert_eq!(video.data.duration, 10);
                    assert!(video.data.thumbnail.is_none());
                    assert!(video.data.cover.is_none());
                    assert!(video.data.start_timestamp.is_none());
                    assert!(video.data.qualities.is_none());
                    assert!(video.data.file_name.is_none());
                    assert!(video.data.mime_type.is_none());
                    assert!(video.data.file_size.is_none());
                    let caption = video.caption.unwrap();
                    assert_eq!(caption.data, "caption");
                    assert_eq!(caption.entities.unwrap().into_iter().len(), 1);
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
    ])
    .await;
}

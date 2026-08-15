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
            "send-animation-base",
            SendAnimation::new(InputFile::file_id("file-id"), 1),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.date, 0);
                assert_eq!(x.chat.get_id(), 1);
                assert!(x.chat.get_username().is_none());
                assert!(matches!(x.chat, Chat::Group(_)));
                assert_eq!(x.sender.get_user_id().unwrap(), 1);
                assert!(x.sender.get_user_username().is_none());
                assert!(matches!(x.sender, MessageSender::User(_)));
                if let MessageData::Animation(data) = x.data {
                    assert_eq!(data.duration, 20);
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "uid");
                    assert_eq!(data.height, 200);
                    assert_eq!(data.width, 200);
                    assert!(data.file_name.is_none());
                    assert!(data.mime_type.is_none());
                    assert!(data.file_size.is_none());
                    assert!(data.thumbnail.is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-animation-full",
            SendAnimation::new(InputFile::file_id("file-id"), 1)
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_caption(("Caption", ParseMode::Markdown))
                .with_callback_query_id("cqid")
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_duration(100)
                .with_has_spoiler(true)
                .with_height(300)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_receiver_user_id(999)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_show_caption_above_media(true)
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_thumbnail_file(Cursor::new(b"file-data"))
                .with_thumbnail_url("https://google.com/favicon.ico")
                .with_width(200),
            |x| {
                if let MessageData::Animation(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "uid");
                    assert_eq!(data.width, 200);
                    assert_eq!(data.height, 300);
                    assert_eq!(data.duration, 100);
                    assert_eq!(data.file_name.unwrap(), "test");
                    assert_eq!(data.mime_type.unwrap(), "video/mp4");
                    assert_eq!(data.file_size.unwrap(), 10000);
                    let thumb = data.thumbnail.unwrap();
                    assert_eq!(thumb.file_id, "thumb-file-id");
                    assert_eq!(thumb.file_unique_id, "tuid");
                    assert_eq!(thumb.width, 200);
                    assert_eq!(thumb.height, 300);
                    assert_eq!(thumb.file_size.unwrap(), 100);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-animation-thumbnail-file",
            SendAnimation::new(InputFile::file_id("file-id"), 1)
                .with_thumbnail_url("https://google.com/favicon.ico")
                .with_thumbnail_file(Cursor::new(b"file-data")),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.date, 0);
                if let MessageData::Animation(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-animation-caption-entities",
            SendAnimation::new(InputFile::file_id("file-id"), 1).with_caption(
                InputText::from("caption")
                    .with_format(ParseMode::Markdown)
                    .with_format([TextEntity::bold(0..10)]),
            ),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.date, 0);
                if let MessageData::Animation(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-animation-caption-parse-mode",
            SendAnimation::new(InputFile::file_id("file-id"), 1).with_caption(
                InputText::from("caption")
                    .with_format([TextEntity::bold(0..10)])
                    .with_format(ParseMode::MarkdownV2),
            ),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.date, 0);
                if let MessageData::Animation(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;
}

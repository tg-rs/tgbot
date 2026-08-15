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
            "send-video-note-base",
            SendVideoNote::new(1, InputFile::file_id("file-id")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::VideoNote(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "file-unique-id");
                    assert_eq!(data.length, 100);
                    assert_eq!(data.duration, 200);
                    assert!(data.thumbnail.is_none());
                    assert!(data.file_size.is_none());
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-video-note-all",
            SendVideoNote::new(1, InputFile::file_id("file-id"))
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_callback_query_id("cqid")
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_duration(50)
                .with_length(100)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_receiver_user_id(999)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_thumbnail_file(Cursor::new(b"file-data"))
                .with_thumbnail_url("https://example.com/image.jpg"),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::VideoNote(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "file-unique-id");
                    assert_eq!(data.length, 100);
                    assert_eq!(data.duration, 200);
                    assert_eq!(data.file_size.unwrap(), 10000);
                    let thumbnail = data.thumbnail.unwrap();
                    assert_eq!(thumbnail.file_id, "tfid");
                    assert_eq!(thumbnail.file_unique_id, "tfuid");
                    assert_eq!(thumbnail.width, 200);
                    assert_eq!(thumbnail.height, 200);
                    assert!(thumbnail.file_size.is_none());
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-video-note-thumbnail-file",
            SendVideoNote::new(1, InputFile::file_id("file-id"))
                .with_thumbnail_url("https://example.com/image.jpg")
                .with_thumbnail_file(Cursor::new(b"file-data")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::VideoNote(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "file-unique-id");
                    assert_eq!(data.length, 100);
                    assert_eq!(data.duration, 200);
                    assert!(data.thumbnail.is_some());
                    assert_eq!(data.file_size.unwrap(), 10000);
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
    ])
    .await;
}

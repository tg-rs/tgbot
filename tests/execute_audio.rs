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
            "send-audio-base",
            SendAudio::new(1, InputFile::file_id("file-id")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Audio(audio) = x.data {
                    assert_eq!(audio.data.file_id, "file-id");
                    assert_eq!(audio.data.file_unique_id, "uid");
                    assert_eq!(audio.data.duration, 20);
                    assert!(audio.data.performer.is_none());
                    assert!(audio.data.title.is_none());
                    assert!(audio.data.file_name.is_none());
                    assert!(audio.data.mime_type.is_none());
                    assert!(audio.data.file_size.is_none());
                    assert!(audio.data.thumbnail.is_none());
                    assert!(audio.caption.is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-audio-full",
            SendAudio::new(1, InputFile::file_id("file-id"))
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_callback_query_id("cqid")
                .with_caption(("Caption", ParseMode::Markdown))
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_duration(100)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_performer("Performer")
                .with_protect_content(true)
                .with_receiver_user_id(999)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_thumbnail_file(Cursor::new(b"thumbnail-file-data"))
                .with_thumbnail_url("https://google.com/favicon.ico")
                .with_title("Title"),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Audio(audio) = x.data {
                    assert_eq!(audio.data.file_id, "file-id");
                    assert_eq!(audio.data.file_unique_id, "uid");
                    assert_eq!(audio.data.duration, 100);
                    assert_eq!(audio.data.performer.unwrap(), "Performer");
                    assert_eq!(audio.data.title.unwrap(), "Title");
                    assert_eq!(audio.data.file_name.unwrap(), "test");
                    assert_eq!(audio.data.mime_type.unwrap(), "audio/mpeg");
                    assert_eq!(audio.data.file_size.unwrap(), 10000);
                    let caption = audio.caption.unwrap();
                    assert_eq!(caption.data, "Caption");
                    assert!(caption.entities.is_none());
                    let thumb = audio.data.thumbnail.unwrap();
                    assert_eq!(thumb.file_id, "thumb-file-id");
                    assert_eq!(thumb.file_unique_id, "tuid");
                    assert_eq!(thumb.width, 200);
                    assert_eq!(thumb.height, 200);
                    assert_eq!(thumb.file_size.unwrap(), 100);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-audio-thumbnail-file",
            SendAudio::new(1, InputFile::file_id("file-id"))
                .with_thumbnail_url("https://google.com/favicon.ico")
                .with_thumbnail_file(Cursor::new(b"thumbnail-file-data")),
            |x| {
                assert!(matches!(x.data, MessageData::Audio(_)));
            },
        ),
        (
            "send-audio-caption-entities",
            SendAudio::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format(ParseMode::Markdown)
                    .with_format([TextEntity::bold(0..2)]),
            ),
            |x| {
                if let MessageData::Audio(audio) = x.data {
                    let caption = audio.caption.unwrap();
                    assert_eq!(caption.data, "caption");
                    let mut entities = caption.entities.unwrap().into_iter();
                    let entity = entities.next().unwrap();
                    if let TextEntity::Bold(pos) = entity {
                        assert_eq!(pos.offset, 0);
                        assert_eq!(pos.length, 2);
                    } else {
                        panic!("Got an unexpected text entity: {entity:?}");
                    }
                    assert!(entities.next().is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-audio-caption-parse-mode",
            SendAudio::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format([TextEntity::bold(0..10)])
                    .with_format(ParseMode::Html),
            ),
            |x| {
                assert!(matches!(x.data, MessageData::Audio(_)));
            },
        ),
    ])
    .await;
}

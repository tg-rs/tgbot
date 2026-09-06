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
            "send-document-base",
            SendDocument::new(1, InputFile::file_id("file-id")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Document(document) = x.data {
                    assert_eq!(document.data.file_id, "file-id");
                    assert_eq!(document.data.file_unique_id, "fuid");
                    assert!(document.data.thumbnail.is_none());
                    assert!(document.data.file_name.is_none());
                    assert!(document.data.mime_type.is_none());
                    assert!(document.data.file_size.is_none());
                    assert!(document.caption.is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-document-full",
            SendDocument::new(1, InputFile::file_id("file-id"))
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_caption(("Caption", ParseMode::Markdown))
                .with_disable_content_type_detection(true)
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_ephemeral_message_parameters(999)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_thumbnail_file(Cursor::new(b"file-data"))
                .with_thumbnail_url("https://example.com/image.jpg"),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Document(document) = x.data {
                    assert_eq!(document.data.file_id, "file-id");
                    assert_eq!(document.data.file_unique_id, "fuid");
                    assert_eq!(document.data.file_name.unwrap(), "test");
                    assert_eq!(document.data.mime_type.unwrap(), "application/pdf");
                    assert_eq!(document.data.file_size.unwrap(), 10000);
                    let thumb = document.data.thumbnail.unwrap();
                    assert_eq!(thumb.file_id, "tfid");
                    assert_eq!(thumb.file_unique_id, "tfuid");
                    assert_eq!(thumb.width, 200);
                    assert_eq!(thumb.height, 200);
                    assert!(thumb.file_size.is_none());
                    let caption = document.caption.unwrap();
                    assert_eq!(caption.data, "test");
                    assert!(caption.entities.is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-document-thumbnail-file",
            SendDocument::new(1, InputFile::file_id("file-id"))
                .with_thumbnail_url("https://example.com/image.jpg")
                .with_thumbnail_file(Cursor::new(b"file-data")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Document(document) = x.data {
                    assert_eq!(document.data.file_id, "file-id");
                    assert_eq!(document.data.file_unique_id, "fuid");
                    assert!(document.data.thumbnail.is_none());
                    assert!(document.data.file_name.is_none());
                    assert!(document.data.mime_type.is_none());
                    assert!(document.data.file_size.is_none());
                    assert!(document.caption.is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-document-caption-entities",
            SendDocument::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format(ParseMode::Markdown)
                    .with_format([TextEntity::bold(0..10)]),
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Document(document) = x.data {
                    assert_eq!(document.data.file_id, "file-id");
                    assert_eq!(document.data.file_unique_id, "fuid");
                    assert!(document.data.thumbnail.is_none());
                    assert!(document.data.file_name.is_none());
                    assert!(document.data.mime_type.is_none());
                    assert!(document.data.file_size.is_none());
                    assert!(document.caption.is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-document-caption-parse-mode",
            SendDocument::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format([TextEntity::bold(0..10)])
                    .with_format(ParseMode::Markdown),
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Document(document) = x.data {
                    assert_eq!(document.data.file_id, "file-id");
                    assert_eq!(document.data.file_unique_id, "fuid");
                    assert!(document.data.thumbnail.is_none());
                    assert!(document.data.file_name.is_none());
                    assert!(document.data.mime_type.is_none());
                    assert!(document.data.file_size.is_none());
                    assert!(document.caption.is_none());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;
}

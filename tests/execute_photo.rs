#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-photo-base",
            SendPhoto::new(1, InputFile::file_id("file-id")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Photo(photo) = x.data {
                    assert_eq!(photo.data.len(), 1);
                    assert!(photo.caption.is_none());
                    let item = &photo.data[0];
                    assert_eq!(item.file_id, "file-id");
                    assert_eq!(item.file_unique_id, "fuid");
                    assert_eq!(item.width, 200);
                    assert_eq!(item.height, 200);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-photo-all",
            SendPhoto::new(1, InputFile::file_id("file-id"))
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_caption(("Caption", ParseMode::Markdown))
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_ephemeral_message_parameters(999)
                .with_has_spoiler(true)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_show_caption_above_media(true)
                .with_suggested_post_parameters(SuggestedPostParameters::default()),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Photo(photo) = x.data {
                    assert_eq!(photo.data.len(), 1);
                    let caption = photo.caption.unwrap();
                    assert_eq!(caption.data, "caption");
                    let item = &photo.data[0];
                    assert_eq!(item.file_id, "file-id");
                    assert_eq!(item.file_unique_id, "fuid");
                    assert_eq!(item.width, 200);
                    assert_eq!(item.height, 200);
                    assert_eq!(item.file_size.unwrap(), 100);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-photo-caption-entities",
            SendPhoto::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format(ParseMode::Markdown)
                    .with_format([TextEntity::bold(0..10)]),
            ),
            |x| {
                assert_eq!(x.id, 1);
                assert!(matches!(x.data, MessageData::Photo(_)));
            },
        ),
        (
            "send-photo-caption-parse-mode",
            SendPhoto::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format([TextEntity::bold(0..10)])
                    .with_format(ParseMode::Markdown),
            ),
            |x| {
                assert_eq!(x.id, 1);
                assert!(matches!(x.data, MessageData::Photo(_)));
            },
        ),
    ])
    .await;
}

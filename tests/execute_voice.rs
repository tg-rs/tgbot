#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-voice-base",
            SendVoice::new(1, InputFile::file_id("file-id")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Voice(voice) = x.data {
                    assert_eq!(voice.data.file_id, "file-id");
                    assert_eq!(voice.data.file_unique_id, "file-unique-id");
                    assert_eq!(voice.data.duration, 100);
                    assert!(voice.data.mime_type.is_none());
                    assert!(voice.data.file_size.is_none());
                    assert!(voice.caption.is_none());
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-voice-all",
            SendVoice::new(1, InputFile::file_id("file-id"))
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_caption(("Caption", ParseMode::Markdown))
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_duration(100)
                .with_ephemeral_message_parameters(999)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default()),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Voice(voice) = x.data {
                    assert_eq!(voice.data.file_id, "file-id");
                    assert_eq!(voice.data.file_unique_id, "file-unique-id");
                    assert_eq!(voice.data.duration, 100);
                    assert_eq!(voice.data.mime_type.unwrap(), "audio/mpeg");
                    assert_eq!(voice.data.file_size.unwrap(), 10000);
                    let caption = voice.caption.unwrap();
                    assert_eq!(caption.data, "caption");
                    assert_eq!(caption.entities.unwrap().into_iter().len(), 1);
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-voice-caption-entities",
            SendVoice::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format(ParseMode::Markdown)
                    .with_format([TextEntity::bold(0..2)]),
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Voice(voice) = x.data {
                    assert_eq!(voice.data.file_id, "file-id");
                    assert_eq!(voice.data.file_unique_id, "file-unique-id");
                    assert_eq!(voice.data.duration, 100);
                    assert_eq!(voice.data.mime_type.unwrap(), "audio/mpeg");
                    assert_eq!(voice.data.file_size.unwrap(), 10000);
                    let caption = voice.caption.unwrap();
                    assert_eq!(caption.data, "caption");
                    assert_eq!(caption.entities.unwrap().into_iter().len(), 1);
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
        (
            "send-voice-caption-parse-mode",
            SendVoice::new(1, InputFile::file_id("file-id")).with_caption(
                InputText::from("caption")
                    .with_format([TextEntity::bold(0..10)])
                    .with_format(ParseMode::Markdown),
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Voice(voice) = x.data {
                    assert_eq!(voice.data.file_id, "file-id");
                    assert_eq!(voice.data.file_unique_id, "file-unique-id");
                    assert_eq!(voice.data.duration, 100);
                    assert_eq!(voice.data.mime_type.unwrap(), "audio/mpeg");
                    assert_eq!(voice.data.file_size.unwrap(), 10000);
                    let caption = voice.caption.unwrap();
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

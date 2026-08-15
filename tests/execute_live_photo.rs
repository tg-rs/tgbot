#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-live-photo-base",
            SendLivePhoto::new(1, InputFile::file_id("file-id"), InputFile::file_id("photo-id")),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::LivePhoto(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "fuid");
                    assert_eq!(data.width, 200);
                    assert_eq!(data.height, 200);
                    assert_eq!(data.duration, 10);
                    assert!(data.mime_type.is_none());
                    assert!(data.file_size.is_none());
                } else {
                    panic!("Got an unexpected message data {:?}", x.data);
                }
            },
        ),
        (
            "send-live-photo-all",
            SendLivePhoto::new(1, InputFile::file_id("file-id"), InputFile::file_id("photo-id"))
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("bc-id")
                .with_callback_query_id("cqid")
                .with_caption(("test", ParseMode::Markdown))
                .with_direct_messages_topic_id(1)
                .with_disable_notification(false)
                .with_has_spoiler(false)
                .with_message_effect_id("eid")
                .with_message_thread_id(10)
                .with_protect_content(true)
                .with_receiver_user_id(999)
                .with_reply_markup(ReplyKeyboardRemove::default())
                .with_reply_parameters(
                    ReplyParameters::new(1)
                        .with_allow_sending_without_reply(true)
                        .with_chat_id(1)
                        .with_checklist_task_id(2)
                        .with_ephemeral_message_id(3)
                        .with_poll_option_id("test")
                        .with_quote(ReplyQuote::new(0, "test")),
                )
                .with_show_caption_above_media(true)
                .with_suggested_post_parameters(
                    SuggestedPostParameters::default()
                        .with_price(SuggestedPostPrice::new(10, "USD"))
                        .with_send_date(10),
                ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::LivePhoto(data) = x.data {
                    assert_eq!(data.file_id, "file-id");
                    assert_eq!(data.file_unique_id, "fuid");
                    assert_eq!(data.width, 200);
                    assert_eq!(data.height, 200);
                    assert_eq!(data.duration, 10);
                    assert_eq!(data.mime_type.unwrap(), "video/mp4");
                    assert_eq!(data.file_size.unwrap(), 10000);
                } else {
                    panic!("Got an unexpected message data {:?}", x.data);
                }
            },
        ),
    ])
    .await;
}

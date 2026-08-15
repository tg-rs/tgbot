#![allow(missing_docs)]

use std::io::Cursor;

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("send-message-base", SendMessage::new(1, "text"), |x| {
            assert_eq!(x.id, 1);
            assert!(matches!(x.data, MessageData::Text(_)));
        }),
        (
            "send-message-all",
            SendMessage::new(1, ("text", [TextEntity::bold(0..2)]))
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_callback_query_id("cqid")
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_link_preview_options(LinkPreviewOptions::default().with_is_disabled(true))
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_receiver_user_id(999)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default()),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Text(text) = x.data {
                    assert_eq!(text.data, "text");
                    assert!(text.entities.is_some());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;

    cx.execute_batch([
        ("send-message-draft-base", SendMessageDraft::new(1, 1, "text"), |x| {
            assert!(x);
        }),
        (
            "send-message-draft-entities",
            SendMessageDraft::new(1, 1, ("text", [TextEntity::bold(0..2)])).with_message_thread_id(1),
            |x| {
                assert!(x);
            },
        ),
        (
            "send-message-draft-parse-mode",
            SendMessageDraft::new(1, 1, ("text", ParseMode::Markdown)).with_message_thread_id(1),
            |x| {
                assert!(x);
            },
        ),
    ])
    .await;

    cx.execute_batch([
        ("copy-message-base", CopyMessage::new(1, 2, 3), |x| {
            assert_eq!(x.message_id, 4)
        }),
        (
            "copy-message-all",
            CopyMessage::new(1, 2, 3)
                .with_allow_paid_broadcast(true)
                .with_caption("caption")
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_message_effect_id("test")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_show_caption_above_media(true)
                .with_video_start_timestamp(200),
            |x| assert_eq!(x.message_id, 4),
        ),
    ])
    .await;

    cx.execute_batch([
        ("copy-messages-base", CopyMessages::new(1, 2, [3]), |x| {
            assert_eq!(x.len(), 1);
            assert_eq!(x[0].message_id, 4)
        }),
        (
            "copy-messages-all",
            CopyMessages::new(1, 2, [3])
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_remove_caption(true),
            |x| {
                assert_eq!(x.len(), 1);
                assert_eq!(x[0].message_id, 4)
            },
        ),
    ])
    .await;

    cx.execute_batch([("delete-message", DeleteMessage::new(1, 2), |x| assert!(x))])
        .await;

    cx.execute_batch([("delete-messages", DeleteMessages::new(1, [2]), |x| assert!(x))])
        .await;

    cx.execute_batch([
        (
            "edit-message-caption-chat-base",
            EditMessageCaption::for_chat_message(1, 2),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-caption-chat-all",
            EditMessageCaption::for_chat_message(1, 2)
                .with_business_connection_id("c-id")
                .with_caption("caption")
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_show_caption_above_media(true),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-caption-inline",
            EditMessageCaption::for_inline_message("msg-id"),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "edit-message-live-location-chat-base",
            EditMessageLiveLocation::for_chat_message(1, 2, 3.0, 4.0),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-live-location-chat-all",
            EditMessageLiveLocation::for_chat_message(1, 2, 3.0, 4.0)
                .with_business_connection_id("c-id")
                .with_heading(100)
                .with_horizontal_accuracy(5.0)
                .with_live_period(10)
                .with_proximity_alert_radius(200)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-live-location-inline",
            EditMessageLiveLocation::for_inline_message("msg-id", 3.0, 4.0),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "edit-message-media-chat",
            EditMessageMedia::for_chat_message(1, 2, InputMediaPhoto::from(InputFile::file_id("file-id")))
                .with_business_connection_id("c-id")
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-media-inline",
            EditMessageMedia::for_inline_message("msg-id", InputMediaPhoto::from(Cursor::new("file-data"))),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-animation-base",
            EditMessageMedia::for_inline_message("msg-id", InputMediaAnimation::from(Cursor::new(b"test-animation"))),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-animation-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaAnimation::from(Cursor::new(b"test-animation"))
                    .with_caption("test-caption")
                    .with_duration(10)
                    .with_has_spoiler(true)
                    .with_height(200)
                    .with_show_caption_above_media(true)
                    .with_thumbnail(InputFile::file_id("test-thumbnail"))
                    .with_width(200),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-audio-base",
            EditMessageMedia::for_inline_message("msg-id", InputMediaAudio::from(Cursor::new(b"test-audio"))),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-audio-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaAudio::from(Cursor::new(b"test-audio"))
                    .with_caption("test-caption")
                    .with_duration(10)
                    .with_performer("test-performer")
                    .with_thumbnail(InputFile::file_id("test-thumbnail"))
                    .with_title("test-title"),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-document-base",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaDocument::from(InputFile::file_id("document-file-id")),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-document-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaDocument::from(InputFile::file_id("document-file-id"))
                    .with_caption("test")
                    .with_disable_content_type_detection(true)
                    .with_thumbnail(InputFile::file_id("test-thumbnail")),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-link",
            EditMessageMedia::for_inline_message("msg-id", InputMedia::link("https://example.com")),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-live-photo-base",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaLivePhoto::from((
                    InputFile::url("https://example.com/video.mp4"),
                    InputFile::url("https://example.com/photo.png"),
                )),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-live-photo-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaLivePhoto::from((
                    InputFile::url("https://example.com/video.mp4"),
                    InputFile::url("https://example.com/photo.png"),
                ))
                .with_caption(InputText::from("test").with_format(ParseMode::Markdown))
                .with_show_caption_above_media(true)
                .with_has_spoiler(false),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-location-base",
            EditMessageMedia::for_inline_message("msg-id", InputMediaLocation::new(1.0, 2.0)),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-location-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaLocation::new(1.0, 2.0).with_horizontal_accuracy(3.0),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-photo-base",
            EditMessageMedia::for_inline_message("msg-id", InputMediaPhoto::from(InputFile::file_id("photo-file-id"))),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-photo-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaPhoto::from(InputFile::file_id("photo-file-id"))
                    .with_caption(InputText::from("test").with_format(ParseMode::Markdown))
                    .with_has_spoiler(true)
                    .with_show_caption_above_media(true),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-sticker-base",
            EditMessageMedia::for_inline_message("msg-id", InputMediaSticker::from(Cursor::new(b"test-sticker"))),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-sticker-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaSticker::from(Cursor::new(b"test-sticker")).with_emoji("🤡"),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-venue-base",
            EditMessageMedia::for_inline_message("msg-id", InputMediaVenue::new(1.0, 2.0, "test", "addr")),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-venue-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaVenue::new(1.0, 2.0, "test", "addr")
                    .with_foursquare_id("f-id")
                    .with_foursquare_type("f-type")
                    .with_google_place_id("g-id")
                    .with_google_place_type("g-type"),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-video-base",
            EditMessageMedia::for_inline_message("msg-id", InputMediaVideo::from(InputFile::file_id("video-file-id"))),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-video-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaVideo::from(InputFileReader::from(Cursor::new("video-file-data")))
                    .with_thumbnail(InputFileReader::from(Cursor::new("video-thumb-data")))
                    .with_cover(InputFile::file_id("cover-id"))
                    .with_caption(InputText::from("caption").with_format(ParseMode::Markdown))
                    .with_duration(100)
                    .with_has_spoiler(true)
                    .with_height(200)
                    .with_show_caption_above_media(true)
                    .with_start_timestamp(450)
                    .with_supports_streaming(true)
                    .with_width(200),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-voice-note-base",
            EditMessageMedia::for_inline_message("msg-id", InputMediaVoiceNote::from(InputFile::url("test"))),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-media-voice-note-all",
            EditMessageMedia::for_inline_message(
                "msg-id",
                InputMediaVoiceNote::from(InputFile::url("test"))
                    .with_caption(InputText::from("test").with_format([TextEntity::bold(0..2)]))
                    .with_duration(1),
            ),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "edit-message-reply-markup-chat-base",
            EditMessageReplyMarkup::for_chat_message(1, 2),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-reply-markup-chat-all",
            EditMessageReplyMarkup::for_chat_message(1, 2)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_business_connection_id("c-id"),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-reply-markup-inline",
            EditMessageReplyMarkup::for_inline_message("msg-id"),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "edit-message-text-chat-plain",
            EditMessageText::for_chat_message(1, 2, "text"),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-text-chat-rich",
            EditMessageText::for_chat_message_rich(1, 2, InputRichMessage::html("text")),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-text-chat-parse-mode",
            EditMessageText::for_chat_message(1, 2, ("text", ParseMode::Markdown))
                .with_business_connection_id("c-id")
                .with_link_preview_options(LinkPreviewOptions::default().with_is_disabled(true))
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "edit-message-text-inline-plain",
            EditMessageText::for_inline_message("msg-id", "text"),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-text-inline-rich",
            EditMessageText::for_inline_message_rich("msg-id", InputRichMessage::markdown("text")),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
        (
            "edit-message-text-inline-entities",
            EditMessageText::for_inline_message("msg-id", ("text", [TextEntity::bold(0..4)])),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
    ])
    .await;

    cx.execute_batch([
        ("forward-message-base", ForwardMessage::new(1, 2, 3), |x| {
            assert_eq!(x.id, 1)
        }),
        (
            "forward-message-all",
            ForwardMessage::new(1, 2, 3)
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_message_effect_id("test")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_video_start_timestamp(200),
            |x| assert_eq!(x.id, 1),
        ),
    ])
    .await;

    cx.execute_batch([
        ("forward-messages-base", { ForwardMessages::new(1, 2, [3]) }, |x| {
            assert_eq!(x[0].message_id, 4)
        }),
        (
            "forward-messages-all",
            {
                ForwardMessages::new(1, 2, [3])
                    .with_direct_messages_topic_id(1)
                    .with_disable_notification(true)
                    .with_message_thread_id(1)
                    .with_protect_content(true)
            },
            |x| assert_eq!(x[0].message_id, 4),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "stop-message-live-location-chat-base",
            StopMessageLiveLocation::for_chat_message(1, 2),
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "stop-message-live-location-chat-all",
            {
                StopMessageLiveLocation::for_chat_message(1, 2)
                    .with_business_connection_id("c-id")
                    .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            },
            |x| assert!(matches!(x, EditMessageResult::Message(_))),
        ),
        (
            "stop-message-live-location-inline",
            StopMessageLiveLocation::for_inline_message("msg-id"),
            |x| assert!(matches!(x, EditMessageResult::Bool(true))),
        ),
    ])
    .await;
}

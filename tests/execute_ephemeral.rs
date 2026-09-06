#![allow(missing_docs)]

use std::io::Cursor;

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;

    cx.execute_batch([(
        "delete-ephemeral-message",
        DeleteEphemeralMessage::from((1, 2, 3)),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([
        (
            "edit-ephemeral-message-caption-base",
            EditEphemeralMessageCaption::from((1, 2, 3)),
            |x| assert!(x),
        ),
        (
            "edit-ephemeral-message-caption-all",
            EditEphemeralMessageCaption::from((1, 2, 3))
                .with_caption("test")
                .with_show_caption_above_media(true)
                .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "edit-ephemeral-message-media-base",
            EditEphemeralMessageMedia::new((1, 2, 3), InputMediaAnimation::from(Cursor::new("animation-file"))),
            |x| assert!(x),
        ),
        (
            "edit-ephemeral-message-media-all",
            EditEphemeralMessageMedia::new((1, 2, 3), InputMediaAnimation::from(Cursor::new("animation-file")))
                .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "edit-ephemeral-message-reply-markup-base",
            EditEphemeralMessageReplyMarkup::from(("@chat", 2, 3)),
            |x| assert!(x),
        ),
        (
            "edit-ephemeral-message-reply-markup-all",
            EditEphemeralMessageReplyMarkup::from(("@chat", 2, 3))
                .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "edit-ephemeral-message-text-rich",
            EditEphemeralMessageText::rich_message((1, 2, 3), InputRichMessage::markdown("test")),
            |x| assert!(x),
        ),
        (
            "edit-ephemeral-message-text-base",
            EditEphemeralMessageText::text((1, 2, 3), "test"),
            |x| assert!(x),
        ),
        (
            "edit-ephemeral-message-text-parse-mode",
            EditEphemeralMessageText::text(
                (1, 2, 3),
                InputText::from("test")
                    .with_format([TextEntity::bold(0..2)])
                    .with_format(ParseMode::Markdown),
            )
            .with_link_preview_options(LinkPreviewOptions::default())
            .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]),
            |x| assert!(x),
        ),
        (
            "edit-ephemeral-message-text-entities",
            EditEphemeralMessageText::text(
                (1, 2, 3),
                InputText::from("test")
                    .with_format(ParseMode::Markdown)
                    .with_format([TextEntity::bold(0..2)]),
            )
            .with_link_preview_options(LinkPreviewOptions::default())
            .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]),
            |x| assert!(x),
        ),
    ])
    .await;
}

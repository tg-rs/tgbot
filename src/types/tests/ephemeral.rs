use std::io::Cursor;

use crate::types::*;

#[test]
fn delete_ephemeral_message() {
    let method = DeleteEphemeralMessage::from((1, 2, 3));
    assert_payload_eq!(POST JSON "deleteEphemeralMessage" => method);
}

#[test]
fn edit_ephemeral_message_caption() {
    let method = EditEphemeralMessageCaption::from((1, 2, 3));
    assert_payload_eq!(POST JSON "editEphemeralMessageCaption" => method);

    let method = EditEphemeralMessageCaption::from((1, 2, 3))
        .with_caption("test")
        .with_caption_entities([TextEntity::bold(0..2)])
        .with_parse_mode(ParseMode::Markdown)
        .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]);
    assert_payload_eq!(POST JSON "editEphemeralMessageCaption" => method);

    let method = EditEphemeralMessageCaption::from((1, 2, 3))
        .with_caption("test")
        .with_parse_mode(ParseMode::Markdown)
        .with_caption_entities([TextEntity::bold(0..2)])
        .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]);
    assert_payload_eq!(POST JSON "editEphemeralMessageCaption" => method);
}

#[test]
fn edit_ephemeral_message_media() {
    let method =
        EditEphemeralMessageMedia::new((1, 2, 3), InputMediaAnimation::from(Cursor::new("animation-file"))).unwrap();
    assert_payload_eq!(POST FORM "editEphemeralMessageMedia" => method);

    let method = EditEphemeralMessageMedia::new((1, 2, 3), InputMediaAnimation::from(Cursor::new("animation-file")))
        .unwrap()
        .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]])
        .unwrap();
    assert_payload_eq!(POST FORM "editEphemeralMessageMedia" => method);
}

#[test]
fn edit_ephemeral_message_reply_markup() {
    let method = EditEphemeralMessageReplyMarkup::from(("@chat", 2, 3));
    assert_payload_eq!(POST JSON "editEphemeralMessageReplyMarkup" => method.clone());

    let method = method.with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]);
    assert_payload_eq!(POST JSON "editEphemeralMessageReplyMarkup" => method);
}

#[test]
fn edit_ephemeral_message_text() {
    let method = EditEphemeralMessageText::new((1, 2, 3), "test");
    assert_payload_eq!(POST JSON "editEphemeralMessageText" => method.clone());

    let method = method
        .with_entities([TextEntity::bold(0..2)])
        .with_parse_mode(ParseMode::Markdown)
        .with_link_preview_options(LinkPreviewOptions::default())
        .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]);
    assert_payload_eq!(POST JSON "editEphemeralMessageText" => method.clone());

    let method = method
        .with_parse_mode(ParseMode::Markdown)
        .with_entities([TextEntity::bold(0..2)])
        .with_link_preview_options(LinkPreviewOptions::default())
        .with_reply_markup([[InlineKeyboardButton::for_url("test", "example.com")]]);
    assert_payload_eq!(POST JSON "editEphemeralMessageText" => method);
}

use crate::{
    api::{assert_payload_eq, Form, Payload},
    types::{
        CopyMessage,
        DeleteMessage,
        EditMessageCaption,
        EditMessageLiveLocation,
        EditMessageMedia,
        EditMessageReplyMarkup,
        EditMessageText,
        ForceReply,
        ForwardMessage,
        InlineKeyboardButton,
        InlineKeyboardMarkup,
        InputFile,
        InputMedia,
        InputMediaPhoto,
        ParseMode,
        SendMessage,
        StopMessageLiveLocation,
        TextEntity,
    },
};

#[test]
fn copy_message() {
    let method = CopyMessage::new(1, 2, 3);
    assert_payload_eq(
        Payload::json(
            "copyMessage",
            serde_json::json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3,
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "copyMessage",
            serde_json::json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3,
                "caption": "caption",
                "parse_mode": "Markdown",
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "reply_markup": {"force_reply": true},
                "allow_sending_without_reply": true,
                "message_thread_id": 1
            }),
        ),
        method
            .clone()
            .with_allow_sending_without_reply(true)
            .with_caption("caption")
            .with_disable_notification(true)
            .with_message_thread_id(1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_to_message_id(1),
    );
    assert_payload_eq(
        Payload::json(
            "copyMessage",
            serde_json::json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3,
                "caption_entities": [
                    {
                        "type": "bold",
                        "offset": 1,
                        "length": 1
                    }
                ]
            }),
        ),
        method.with_caption_entities([TextEntity::bold(1..2)]),
    );
}

#[test]
fn delete_message() {
    assert_payload_eq(
        Payload::json(
            "deleteMessage",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2
            }),
        ),
        DeleteMessage::new(1, 2),
    );
}

#[test]
fn edit_message_caption() {
    assert_payload_eq(
        Payload::json(
            "editMessageCaption",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2
            }),
        ),
        EditMessageCaption::for_chat_message(1, 2),
    );
    assert_payload_eq(
        Payload::json(
            "editMessageCaption",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "caption": "caption",
                "parse_mode": "Markdown",
                "reply_markup": {
                    "inline_keyboard": [
                        [
                            {"text": "text", "url": "url"}
                        ]
                    ]
                }
            }),
        ),
        EditMessageCaption::for_chat_message(1, 2)
            .with_caption("caption")
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    );

    assert_payload_eq(
        Payload::json(
            "editMessageCaption",
            serde_json::json!({
                "inline_message_id": "msg-id"
            }),
        ),
        EditMessageCaption::for_inline_message("msg-id"),
    );
    assert_payload_eq(
        Payload::json(
            "editMessageCaption",
            serde_json::json!({
                "inline_message_id": "msg-id",
                "caption_entities": [{"type": "bold", "offset": 0, "length": 10}]
            }),
        ),
        EditMessageCaption::for_inline_message("msg-id").with_caption_entities([TextEntity::bold(0..10)]),
    );
}

#[test]
fn edit_message_live_location() {
    assert_payload_eq(
        Payload::json(
            "editMessageLiveLocation",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "latitude": 3.0,
                "longitude": 4.0
            }),
        ),
        EditMessageLiveLocation::for_chat_message(1, 2, 3.0, 4.0),
    );
    assert_payload_eq(
        Payload::json(
            "editMessageLiveLocation",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "latitude": 3.0,
                "longitude": 4.0,
                "horizontal_accuracy": 5.0,
                "heading": 100,
                "proximity_alert_radius": 200,
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
            }),
        ),
        EditMessageLiveLocation::for_chat_message(1, 2, 3.0, 4.0)
            .with_heading(100)
            .with_horizontal_accuracy(5.0)
            .with_proximity_alert_radius(200)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    );

    assert_payload_eq(
        Payload::json(
            "editMessageLiveLocation",
            serde_json::json!({
                "inline_message_id": "msg-id",
                "latitude": 3.0,
                "longitude": 4.0
            }),
        ),
        EditMessageLiveLocation::for_inline_message("msg-id", 3.0, 4.0),
    );
}

#[test]
fn edit_message_media() {
    let input_media = InputMedia::new(InputFile::file_id("file-id"), InputMediaPhoto::default()).unwrap();
    let mut form: Form = InputMedia::new(InputFile::file_id("file-id"), InputMediaPhoto::default())
        .unwrap()
        .into();
    let markup: InlineKeyboardMarkup = [[InlineKeyboardButton::for_url("text", "url")]].into();
    form.insert_field("chat_id", 1);
    form.insert_field("message_id", 2);
    form.insert_field("reply_markup", markup.serialize().unwrap());
    assert_payload_eq(
        Payload::form("editMessageMedia", form),
        EditMessageMedia::for_chat_message(1, 2, input_media)
            .with_reply_markup(markup)
            .unwrap(),
    );
    let input_media = InputMedia::new(InputFile::file_id("file-id"), InputMediaPhoto::default()).unwrap();
    let mut form: Form = InputMedia::new(InputFile::file_id("file-id"), InputMediaPhoto::default())
        .unwrap()
        .into();
    form.insert_field("inline_message_id", "msg-id");
    assert_payload_eq(
        Payload::form("editMessageMedia", form),
        EditMessageMedia::for_inline_message("msg-id", input_media),
    );
}

#[test]
fn edit_message_reply_markup() {
    let markup = [[InlineKeyboardButton::for_url("text", "url")]];
    assert_payload_eq(
        Payload::json(
            "editMessageReplyMarkup",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2
            }),
        ),
        EditMessageReplyMarkup::for_chat_message(1, 2),
    );
    assert_payload_eq(
        Payload::json(
            "editMessageReplyMarkup",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
            }),
        ),
        EditMessageReplyMarkup::for_chat_message(1, 2).with_reply_markup(markup.clone()),
    );
    assert_payload_eq(
        Payload::json(
            "editMessageReplyMarkup",
            serde_json::json!({"inline_message_id": "msg-id"}),
        ),
        EditMessageReplyMarkup::for_inline_message("msg-id"),
    );
    assert_payload_eq(
        Payload::json(
            "editMessageReplyMarkup",
            serde_json::json!({
                "inline_message_id": "msg-id",
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
            }),
        ),
        EditMessageReplyMarkup::for_inline_message("msg-id").with_reply_markup(markup),
    );
}

#[test]
fn edit_message_text() {
    assert_payload_eq(
        Payload::json(
            "editMessageText",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "text": "text"
            }),
        ),
        EditMessageText::for_chat_message(1, 2, "text"),
    );
    assert_payload_eq(
        Payload::json(
            "editMessageText",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "text": "text",
                "parse_mode": "Markdown",
                "disable_web_page_preview": true,
                "reply_markup": {
                    "inline_keyboard": [
                        [
                            {
                                "text": "text",
                                "url": "url"
                            }
                        ]
                    ]
                }
            }),
        ),
        EditMessageText::for_chat_message(1, 2, "text")
            .with_disable_web_page_preview(true)
            .with_parse_mode(ParseMode::Markdown)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    );

    assert_payload_eq(
        Payload::json(
            "editMessageText",
            serde_json::json!({
                "inline_message_id": "msg-id",
                "text": "text"
            }),
        ),
        EditMessageText::for_inline_message("msg-id", "text"),
    );
    assert_payload_eq(
        Payload::json(
            "editMessageText",
            serde_json::json!({
                "inline_message_id": "msg-id",
                "text": "text",
                "entities": [
                    {
                        "type": "bold",
                        "offset": 0,
                        "length": 4
                    }
                ]
            }),
        ),
        EditMessageText::for_inline_message("msg-id", "text").with_entities([TextEntity::bold(0..4)]),
    );
}

#[test]
fn forward_message() {
    assert_payload_eq(
        Payload::json(
            "forwardMessage",
            serde_json::json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3
            }),
        ),
        ForwardMessage::new(1, 2, 3),
    );
    assert_payload_eq(
        Payload::json(
            "forwardMessage",
            serde_json::json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3,
                "disable_notification": true,
                "protect_content": true,
                "message_thread_id": 1
            }),
        ),
        ForwardMessage::new(1, 2, 3)
            .with_disable_notification(true)
            .with_message_thread_id(1)
            .with_protect_content(true),
    );
}

#[test]
fn send_message() {
    assert_payload_eq(
        Payload::json(
            "sendMessage",
            serde_json::json!({
                "chat_id": 1,
                "text": "text"
            }),
        ),
        SendMessage::new(1, "text"),
    );
    assert_payload_eq(
        Payload::json(
            "sendMessage",
            serde_json::json!({
                "chat_id": 1,
                "text": "text",
                "entities": [{
                    "type": "bold",
                    "offset": 0,
                    "length": 2
                }],
                "disable_web_page_preview": true,
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "force_reply": true
                },
                "message_thread_id": 1,
            }),
        ),
        SendMessage::new(1, "text")
            .with_allow_sending_without_reply(true)
            .with_disable_notification(true)
            .with_disable_web_page_preview(true)
            .with_message_thread_id(1)
            .with_parse_mode(ParseMode::Markdown)
            .with_entities(vec![TextEntity::bold(0..2)])
            .with_protect_content(true)
            .with_reply_to_message_id(1)
            .with_reply_markup(ForceReply::new(true)),
    );
}

#[test]
fn stop_message_live_location() {
    assert_payload_eq(
        Payload::json(
            "stopMessageLiveLocation",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2
            }),
        ),
        StopMessageLiveLocation::for_chat_message(1, 2),
    );
    assert_payload_eq(
        Payload::json(
            "stopMessageLiveLocation",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]}
            }),
        ),
        StopMessageLiveLocation::for_chat_message(1, 2)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
    );
    assert_payload_eq(
        Payload::json(
            "stopMessageLiveLocation",
            serde_json::json!({"inline_message_id": "msg-id"}),
        ),
        StopMessageLiveLocation::for_inline_message("msg-id"),
    );
}

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
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
fn copy_message_full() {
    let method = CopyMessage::new(1, 2, 3)
        .caption("caption")
        .parse_mode(ParseMode::Markdown)
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .reply_markup(ForceReply::new(true))
        .allow_sending_without_reply(true);
    let request = method.clone().into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/copyMessage");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
                "allow_sending_without_reply": true
            })
        );
    } else {
        panic!("Unexpected request body");
    }
    let request = method.caption_entities(vec![TextEntity::bold(1..2)]).into_request();
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data.get("caption_entities").unwrap(),
            &serde_json::json!([{"type": "bold", "offset": 1, "length": 1}])
        );
    }
}

#[test]
fn copy_message_partial() {
    let request = CopyMessage::new(1, 2, 3).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/copyMessage");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn delete_message() {
    let request = DeleteMessage::new(1, 2).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/deleteMessage"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["message_id"], 2);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn edit_message_caption() {
    let request = EditMessageCaption::new(1, 2)
        .caption("caption")
        .parse_mode(ParseMode::Markdown)
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageCaption"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
            })
        );
    } else {
        panic!("Unexpected request body");
    }

    let request = EditMessageCaption::with_inline_message_id("msg-id")
        .caption_entities(vec![TextEntity::bold(0..10)])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageCaption"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "inline_message_id": "msg-id",
                "caption_entities": [{"type": "bold", "offset": 0, "length": 10}]
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[allow(clippy::float_cmp)]
#[test]
fn edit_message_live_location() {
    let request = EditMessageLiveLocation::new(1, 2, 3.0, 4.0)
        .horizontal_accuracy(2.6)
        .heading(100)
        .proximity_alert_radius(200)
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageLiveLocation"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["message_id"], 2);
        assert_eq!(data["latitude"], 3.0);
        assert_eq!(data["longitude"], 4.0);
        assert_eq!(data["horizontal_accuracy"], 2.6);
        assert_eq!(data["heading"], 100);
        assert_eq!(data["proximity_alert_radius"], 200);
        assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
    } else {
        panic!("Unexpected request body");
    }

    let request = EditMessageLiveLocation::with_inline_message_id("msg-id", 3.0, 4.0).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageLiveLocation"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "inline_message_id": "msg-id",
                "latitude": 3.0,
                "longitude": 4.0
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn edit_message_media() {
    let request = EditMessageMedia::new(
        1,
        2,
        InputMedia::new(InputFile::file_id("file-id"), InputMediaPhoto::default()).unwrap(),
    )
    .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
    .unwrap()
    .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageMedia"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert_eq!(form.fields["message_id"].get_text().unwrap(), "2");
        assert!(form.fields.get("media").is_some());
        assert!(form.fields.get("reply_markup").is_some());
    } else {
        panic!("Unexpected request body");
    }

    let request = EditMessageMedia::with_inline_message_id(
        "msg-id",
        InputMedia::new(InputFile::file_id("file-id"), InputMediaPhoto::default()).unwrap(),
    )
    .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageMedia"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["inline_message_id"].get_text().unwrap(), "msg-id");
        assert!(form.fields.get("media").is_some());
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn edit_message_reply_markup() {
    let request = EditMessageReplyMarkup::new(1, 2)
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageReplyMarkup"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["message_id"], 2);
        assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
    } else {
        panic!("Unexpected request body");
    }

    let request = EditMessageReplyMarkup::with_inline_message_id("msg-id")
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageReplyMarkup"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["inline_message_id"], "msg-id");
        assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn edit_message_text() {
    let request = EditMessageText::new(1, 2, "text")
        .parse_mode(ParseMode::Markdown)
        .disable_web_page_preview(true)
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageText"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
            })
        );
    } else {
        panic!("Unexpected request body");
    }

    let request = EditMessageText::with_inline_message_id("msg-id", "text")
        .entities(vec![TextEntity::bold(0..4)])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/editMessageText"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn forward_message() {
    let request = ForwardMessage::new(1, 2, 3)
        .disable_notification(true)
        .protect_content(true)
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/forwardMessage"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3,
                "disable_notification": true,
                "protect_content": true
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn send_message() {
    let request = SendMessage::new(1, "text")
        .parse_mode(ParseMode::Markdown)
        .entities(vec![TextEntity::bold(0..2)])
        .disable_web_page_preview(true)
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendMessage");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
                }
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn stop_message_live_location() {
    let request = StopMessageLiveLocation::new(1, 2)
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/stopMessageLiveLocation"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["chat_id"], 1);
        assert_eq!(data["message_id"], 2);
        assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
    } else {
        panic!("Unexpected request body");
    }

    let request = StopMessageLiveLocation::with_inline_message_id("msg-id").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/stopMessageLiveLocation"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["inline_message_id"], "msg-id");
    } else {
        panic!("Unexpected request body");
    }
}

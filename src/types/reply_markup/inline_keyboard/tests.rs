use serde::Serialize;

use crate::types::{InlineKeyboardButton, InlineKeyboardMarkup, LoginUrl, ReplyMarkup};

#[derive(Serialize)]
struct CallbackData {
    value: String,
}

#[test]
fn serialize() {
    let callback_data = CallbackData {
        value: String::from("cdstruct"),
    };

    let markup: ReplyMarkup = vec![vec![
        InlineKeyboardButton::with_url("url", "tg://user?id=1"),
        InlineKeyboardButton::with_callback_data("cd", "cd"),
        InlineKeyboardButton::with_callback_data_struct("cd", &callback_data).unwrap(),
        InlineKeyboardButton::with_switch_inline_query("siq", "siq"),
        InlineKeyboardButton::with_switch_inline_query_current_chat("siqcc", "siqcc"),
        InlineKeyboardButton::with_callback_game("cg"),
        InlineKeyboardButton::with_pay("pay"),
        InlineKeyboardButton::with_login_url("login url", "http://example.com"),
    ]]
    .into();
    let data = serde_json::to_value(&markup).unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "inline_keyboard": [
                [
                    {"text":"url","url":"tg://user?id=1"},
                    {"text":"cd","callback_data":"cd"},
                    {"text":"cd","callback_data":"{\"value\":\"cdstruct\"}"},
                    {"text":"siq","switch_inline_query":"siq"},
                    {"text":"siqcc","switch_inline_query_current_chat":"siqcc"},
                    {"text":"cg","callback_game":{}},
                    {"text":"pay","pay":true},
                    {"text":"login url","login_url":{"url":"http://example.com"}}
                ]
            ]
        })
    );
}

#[test]
fn deserialize() {
    let buttons: Vec<InlineKeyboardButton> = serde_json::from_value(serde_json::json!(
        [
            {"text":"url","url":"tg://user?id=1"},
            {"text":"cd","callback_data":"cd"},
            {"text":"cd","callback_data":"{\"value\":\"cdstruct\"}"},
            {"text":"siq","switch_inline_query":"siq"},
            {"text":"siqcc","switch_inline_query_current_chat":"siqcc"},
            {"text":"cg","callback_game":{}},
            {"text":"pay","pay":true},
            {"text":"login url","login_url":{"url":"http://example.com"}}
        ]
    ))
    .unwrap();
    assert_eq!(buttons.len(), 8);
    assert_eq!(buttons[0].text(), "url");
    assert_eq!(buttons[1].text(), "cd");
    assert_eq!(buttons[2].text(), "cd");
    assert_eq!(buttons[3].text(), "siq");
    assert_eq!(buttons[4].text(), "siqcc");
    assert_eq!(buttons[5].text(), "cg");
    assert_eq!(buttons[6].text(), "pay");
    assert_eq!(buttons[7].text(), "login url");
}

#[test]
fn convert() {
    let a = vec![vec![InlineKeyboardButton::with_url("url", "tg://user?id=1")]];
    let b: Vec<Vec<InlineKeyboardButton>> = InlineKeyboardMarkup::from(a.clone()).into();
    assert_eq!(a.len(), b.len())
}

#[test]
fn login_url_serialize() {
    let mut url = LoginUrl::from("url");

    let data = serde_json::to_value(&url).unwrap();
    assert_eq!(data, serde_json::json!({"url": "url"}));

    url = url.forward_text("forward text");
    let data = serde_json::to_value(&url).unwrap();
    assert_eq!(data, serde_json::json!({"url": "url", "forward_text": "forward text"}));

    url = url.bot_username("botusername");
    let data = serde_json::to_value(&url).unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "url": "url",
            "forward_text": "forward text",
            "bot_username": "botusername"
        })
    );

    url = url.request_write_access(true);
    let data = serde_json::to_value(&url).unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "url": "url",
            "forward_text": "forward text",
            "bot_username": "botusername",
            "request_write_access": true
        })
    );
}

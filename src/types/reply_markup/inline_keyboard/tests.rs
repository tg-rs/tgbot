use serde::Serialize;

use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineKeyboardMarkup,
    LoginUrl,
    ReplyMarkup,
    WebAppInfo,
};

#[derive(Serialize)]
struct CallbackData {
    value: String,
}

#[test]
fn inline_keyboard() {
    let callback_data = CallbackData {
        value: String::from("cd-struct"),
    };
    assert_json_eq(
        ReplyMarkup::from(vec![vec![
            InlineKeyboardButton::with_url("url", "tg://user?id=1"),
            InlineKeyboardButton::with_web_app(
                "web app",
                WebAppInfo {
                    url: String::from("https://example.com"),
                },
            ),
            InlineKeyboardButton::with_callback_data("cd", "cd"),
            InlineKeyboardButton::with_callback_data_struct("cd", &callback_data).unwrap(),
            InlineKeyboardButton::with_switch_inline_query("siq", "siq"),
            InlineKeyboardButton::with_switch_inline_query_current_chat("siq_cc", "siq_cc"),
            InlineKeyboardButton::with_callback_game("cg"),
            InlineKeyboardButton::with_pay("pay"),
            InlineKeyboardButton::with_login_url("login url", "http://example.com"),
        ]]),
        serde_json::json!({
            "inline_keyboard": [
                [
                    {"text": "url", "url": "tg://user?id=1"},
                    {"text": "web app", "web_app": {"url": "https://example.com"}},
                    {"text": "cd", "callback_data": "cd"},
                    {"text": "cd","callback_data": "{\"value\":\"cd-struct\"}"},
                    {"text": "siq","switch_inline_query": "siq"},
                    {"text": "siq_cc","switch_inline_query_current_chat": "siq_cc"},
                    {"text": "cg","callback_game": {}},
                    {"text": "pay","pay": true},
                    {"text": "login url","login_url": {"url": "http://example.com"}}
                ]
            ]
        }),
    );
}

#[test]
fn inline_keyboard_markup_convert() {
    let a = vec![vec![InlineKeyboardButton::with_url("url", "tg://user?id=1")]];
    let b: Vec<Vec<InlineKeyboardButton>> = InlineKeyboardMarkup::from(a.clone()).into();
    assert_eq!(a.len(), b.len())
}

#[test]
fn login_url() {
    let mut url = LoginUrl::from("url");
    assert_json_eq(url.clone(), serde_json::json!({"url": "url"}));
    url = url.forward_text("forward text");
    assert_json_eq(
        url.clone(),
        serde_json::json!({
            "url": "url",
            "forward_text": "forward text"
        }),
    );
    url = url.bot_username("bot_username");
    assert_json_eq(
        url.clone(),
        serde_json::json!({
            "url": "url",
            "forward_text": "forward text",
            "bot_username": "bot_username"
        }),
    );

    url = url.request_write_access(true);
    assert_json_eq(
        url,
        serde_json::json!({
            "url": "url",
            "forward_text": "forward text",
            "bot_username": "bot_username",
            "request_write_access": true
        }),
    );
}

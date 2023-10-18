use crate::types::{
    tests::assert_json_eq,
    KeyboardButton,
    PollKind,
    ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
    ReplyMarkup,
    WebAppInfo,
};

#[test]
fn reply_keyboard_markup() {
    let row = vec![
        KeyboardButton::new("test"),
        KeyboardButton::new("request contact").request_contact(),
        KeyboardButton::new("request location").request_location(),
        KeyboardButton::new("request quiz").request_poll(PollKind::Quiz),
        KeyboardButton::new("request regular poll").request_poll(PollKind::Regular),
        KeyboardButton::new("request any poll").request_poll(None),
        KeyboardButton::new("web app").web_app(WebAppInfo {
            url: String::from("https://example.com"),
        }),
    ];

    assert_json_eq(
        ReplyMarkup::from(
            ReplyKeyboardMarkup::from(vec![row.clone()])
                .persistent(true)
                .one_time_keyboard(true)
                .selective(true)
                .resize_keyboard(true)
                .input_field_placeholder("placeholder"),
        ),
        serde_json::json!({
            "keyboard": [
                [
                    {"text": "test"},
                    {"text": "request contact", "request_contact": true},
                    {"text": "request location", "request_location": true},
                    {"text": "request quiz", "request_poll": {"type": "quiz"}},
                    {"text": "request regular poll", "request_poll": {"type": "regular"}},
                    {"text": "request any poll", "request_poll": {}},
                    {"text": "web app", "web_app": {"url": "https://example.com"}}
                ]
            ],
            "is_persistent": true,
            "resize_keyboard": true,
            "one_time_keyboard": true,
            "input_field_placeholder": "placeholder",
            "selective": true
        }),
    );

    assert_json_eq(
        ReplyMarkup::from(ReplyKeyboardMarkup::default().row(row)),
        serde_json::json!({
            "keyboard": [
                [
                    {"text": "test"},
                    {"text": "request contact","request_contact":true},
                    {"text": "request location","request_location":true},
                    {"text": "request quiz", "request_poll": {"type": "quiz"}},
                    {"text": "request regular poll", "request_poll": {"type": "regular"}},
                    {"text": "request any poll", "request_poll": {}},
                    {"text": "web app", "web_app": {"url": "https://example.com"}}
                ]
            ]
        }),
    );
}

#[test]
fn reply_keyboard_remove() {
    assert_json_eq(
        ReplyMarkup::from(ReplyKeyboardRemove::default().selective(true)),
        serde_json::json!({
            "remove_keyboard": true,
            "selective": true
        }),
    );
}

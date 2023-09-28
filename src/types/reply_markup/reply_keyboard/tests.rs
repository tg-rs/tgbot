use crate::types::{KeyboardButton, PollKind, ReplyKeyboardMarkup, ReplyKeyboardRemove, ReplyMarkup};

#[test]
fn serialize() {
    let row = vec![
        KeyboardButton::new("test"),
        KeyboardButton::new("request contact").request_contact(),
        KeyboardButton::new("request location").request_location(),
        KeyboardButton::new("request quiz").request_poll(PollKind::Quiz),
        KeyboardButton::new("request regular poll").request_poll(PollKind::Regular),
        KeyboardButton::new("request any poll").request_poll(None),
    ];

    let markup = ReplyKeyboardMarkup::from(vec![row.clone()])
        .one_time_keyboard(true)
        .selective(true)
        .resize_keyboard(true)
        .input_field_placeholder("placeholder");
    let data = serde_json::to_value(&ReplyMarkup::from(markup)).unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "keyboard": [
                [
                    {"text": "test"},
                    {"text": "request contact", "request_contact": true},
                    {"text": "request location", "request_location": true},
                    {"text": "request quiz", "request_poll": {"type": "quiz"}},
                    {"text": "request regular poll", "request_poll": {"type": "regular"}},
                    {"text": "request any poll", "request_poll": {}},
                ]
            ],
            "resize_keyboard": true,
            "one_time_keyboard": true,
            "input_field_placeholder": "placeholder",
            "selective": true
        })
    );

    let markup: ReplyMarkup = ReplyKeyboardMarkup::default().row(row).into();
    let data = serde_json::to_value(&markup).unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "keyboard": [
                [
                    {"text": "test"},
                    {"text": "request contact","request_contact":true},
                    {"text": "request location","request_location":true},
                    {"text": "request quiz", "request_poll": {"type": "quiz"}},
                    {"text": "request regular poll", "request_poll": {"type": "regular"}},
                    {"text": "request any poll", "request_poll": {}},
                ]
            ]
        })
    );

    let markup: ReplyMarkup = ReplyKeyboardRemove::default().selective(true).into();
    let j = serde_json::to_value(&markup).unwrap();
    assert_eq!(j, serde_json::json!({"remove_keyboard":true,"selective":true}));
}

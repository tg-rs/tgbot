use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{Dice, DiceKind, ForceReply, SendDice},
};

#[test]
fn dice_deserialize() {
    let dice: Dice = serde_json::from_value(serde_json::json!({
        "emoji": "🏀",
        "value": 3
    }))
    .unwrap();
    assert_eq!(dice.value(), 3);
    assert_eq!(dice.kind(), DiceKind::Basketball);

    let dice: Dice = serde_json::from_value(serde_json::json!({
        "emoji": "🎲",
        "value": 5
    }))
    .unwrap();
    assert_eq!(dice.value(), 5);
    assert_eq!(dice.kind(), DiceKind::Bones);

    let dice: Dice = serde_json::from_value(serde_json::json!({
        "emoji": "🎳",
        "value": 5
    }))
    .unwrap();
    assert_eq!(dice.value(), 5);
    assert_eq!(dice.kind(), DiceKind::Bowling);

    let dice: Dice = serde_json::from_value(serde_json::json!({
        "emoji": "🎯",
        "value": 1
    }))
    .unwrap();
    assert_eq!(dice.value(), 1);
    assert_eq!(dice.kind(), DiceKind::Darts);

    let dice: Dice = serde_json::from_value(serde_json::json!({
        "emoji": "⚽",
        "value": 3
    }))
    .unwrap();
    assert_eq!(dice.value(), 3);
    assert_eq!(dice.kind(), DiceKind::Football);

    let dice: Dice = serde_json::from_value(serde_json::json!({
        "emoji": "🎰",
        "value": 64
    }))
    .unwrap();
    assert_eq!(dice.value(), 64);
    assert_eq!(dice.kind(), DiceKind::SlotMachine);
}

#[test]
fn send_dice() {
    let request = SendDice::new(1, DiceKind::Bones)
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendDice");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "emoji": "🎲",
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

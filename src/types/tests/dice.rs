use crate::{
    api::{Payload, assert_payload_eq},
    types::*,
};

#[test]
fn dice() {
    for value in [
        serde_json::json!({
            "emoji": DiceType::Basketball.to_string(),
            "value": 1,
        }),
        serde_json::json!({
            "emoji": DiceType::Bones.to_string(),
            "value": 2,
        }),
        serde_json::json!({
            "emoji": DiceType::Bowling.to_string(),
            "value": 3,
        }),
        serde_json::json!({
            "emoji": DiceType::Darts.to_string(),
            "value": 4,
        }),
        serde_json::json!({
            "emoji": DiceType::Football.to_string(),
            "value": 5,
        }),
        serde_json::json!({
            "emoji": DiceType::SlotMachine.to_string(),
            "value": 6,
        }),
    ] {
        let value: Dice = serde_json::from_value(value).unwrap();
        insta::assert_json_snapshot!(value);
        insta::assert_snapshot!(value.dice_type().to_string());
        insta::assert_snapshot!(value.value());
    }
}

#[test]
fn send_dice() {
    let method = SendDice::new(1, DiceType::Bones);
    assert_payload_eq(
        Payload::json(
            "sendDice",
            serde_json::json!({
                "chat_id": 1,
                "emoji": "🎲"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "sendDice",
            serde_json::json!({
                "chat_id": 1,
                "emoji": "🎲",
                "allow_paid_broadcast": true,
                "business_connection_id": "id",
                "disable_notification": true,
                "protect_content": true,
                "message_effect_id": "effect-id",
                "message_thread_id": 1,
                "reply_markup": {
                    "force_reply": true
                },
                "reply_parameters": {
                    "message_id": 1
                }
            }),
        ),
        method
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_parameters(ReplyParameters::new(1)),
    );
}

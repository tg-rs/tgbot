use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, Dice, DiceType, ForceReply, SendDice},
};

#[test]
fn dice() {
    for (expected_struct, expected_value) in [
        (
            Dice {
                dice_type: DiceType::Basketball,
                value: 1,
            },
            serde_json::json!({
                "emoji": "🏀",
                "value": 1
            }),
        ),
        (
            Dice {
                dice_type: DiceType::Bones,
                value: 2,
            },
            serde_json::json!({
                "emoji": "🎲",
                "value": 2
            }),
        ),
        (
            Dice {
                dice_type: DiceType::Bowling,
                value: 3,
            },
            serde_json::json!({
                "emoji": "🎳",
                "value": 3
            }),
        ),
        (
            Dice {
                dice_type: DiceType::Darts,
                value: 4,
            },
            serde_json::json!({
                "emoji": "🎯",
                "value": 4
            }),
        ),
        (
            Dice {
                dice_type: DiceType::Football,
                value: 5,
            },
            serde_json::json!({
                "emoji": "⚽",
                "value": 5
            }),
        ),
        (
            Dice {
                dice_type: DiceType::SlotMachine,
                value: 6,
            },
            serde_json::json!({
                "emoji": "🎰",
                "value": 6
            }),
        ),
    ] {
        assert_json_eq(expected_struct, expected_value.clone());
        assert_eq!(
            expected_struct.dice_type().to_string(),
            expected_value["emoji"].as_str().unwrap()
        );
        assert_eq!(expected_value["value"], expected_struct.value());
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
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "force_reply": true
                },
                "message_thread_id": 1
            }),
        ),
        method
            .with_allow_sending_without_reply(true)
            .with_disable_notification(true)
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_to_message_id(1)
            .with_reply_markup(ForceReply::new(true)),
    );
}

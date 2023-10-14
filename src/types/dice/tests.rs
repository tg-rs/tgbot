use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, Dice, DiceKind, ForceReply, SendDice},
};

#[test]
fn dice() {
    for (expected_struct, expected_value) in [
        (
            Dice {
                kind: DiceKind::Basketball,
                value: 1,
            },
            serde_json::json!({
                "emoji": "🏀",
                "value": 1
            }),
        ),
        (
            Dice {
                kind: DiceKind::Bones,
                value: 2,
            },
            serde_json::json!({
                "emoji": "🎲",
                "value": 2
            }),
        ),
        (
            Dice {
                kind: DiceKind::Bowling,
                value: 3,
            },
            serde_json::json!({
                "emoji": "🎳",
                "value": 3
            }),
        ),
        (
            Dice {
                kind: DiceKind::Darts,
                value: 4,
            },
            serde_json::json!({
                "emoji": "🎯",
                "value": 4
            }),
        ),
        (
            Dice {
                kind: DiceKind::Football,
                value: 5,
            },
            serde_json::json!({
                "emoji": "⚽",
                "value": 5
            }),
        ),
        (
            Dice {
                kind: DiceKind::SlotMachine,
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
            expected_struct.kind().to_string(),
            expected_value["emoji"].as_str().unwrap()
        );
        assert_eq!(expected_value["value"], expected_struct.value());
    }
}

#[test]
fn send_dice() {
    let method = SendDice::new(1, DiceKind::Bones);
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
                }
            }),
        ),
        method
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true)),
    );
}

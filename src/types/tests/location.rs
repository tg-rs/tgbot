use crate::{
    api::{Payload, assert_payload_eq},
    types::*,
};

#[test]
fn location() {
    insta::assert_json_snapshot!(
        Location::new(2.0, 1.0)
            .with_heading(5)
            .with_horizontal_accuracy(3.0)
            .with_live_period(4)
            .with_proximity_alert_radius(6)
    );
    insta::assert_json_snapshot!(Location::new(2.0, 1.0));
}

#[test]
fn location_address() {
    let expected_struct = LocationAddress::new("RU");
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_state("Sakha Republic")
            .with_city("Yakutsk")
            .with_street("Lenina, 1")
    );
}

#[test]
fn send_location() {
    assert_payload_eq(
        Payload::json(
            "sendLocation",
            serde_json::json!({
                "chat_id": 1,
                "latitude": 2.0,
                "longitude": 3.0
            }),
        ),
        SendLocation::new(1, 2.0, 3.0),
    );
    assert_payload_eq(
        Payload::json(
            "sendLocation",
            serde_json::json!({
                "chat_id": 1,
                "latitude": 2.0,
                "longitude": 3.0,
                "horizontal_accuracy": 1.5,
                "live_period": 100,
                "heading": 120,
                "proximity_alert_radius": 100,
                "allow_paid_broadcast": true,
                "business_connection_id": "id",
                "disable_notification": true,
                "protect_content": true,
                "message_effect_id": "effect-id",
                "message_thread_id": 1,
                "reply_markup": {"force_reply": true},
                "reply_parameters": {"message_id": 1}
            }),
        ),
        SendLocation::new(1, 2.0, 3.0)
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_heading(120)
            .with_horizontal_accuracy(1.5)
            .with_live_period(100)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_proximity_alert_radius(100)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_parameters(ReplyParameters::new(1)),
    );
}

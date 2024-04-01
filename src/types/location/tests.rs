use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, ForceReply, Location, ReplyParameters, SendLocation},
};

#[test]
fn location() {
    assert_json_eq(
        Location::new(2.0, 1.0)
            .with_heading(5)
            .with_horizontal_accuracy(3.0)
            .with_live_period(4)
            .with_proximity_alert_radius(6),
        serde_json::json!({
            "longitude": 1.0,
            "latitude": 2.0,
            "horizontal_accuracy": 3.0,
            "live_period": 4,
            "heading": 5,
            "proximity_alert_radius": 6
        }),
    );
    assert_json_eq(
        Location::new(2.0, 1.0),
        serde_json::json!({
            "longitude": 1.0,
            "latitude": 2.0
        }),
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
                "business_connection_id": "id",
                "disable_notification": true,
                "protect_content": true,
                "message_thread_id": 1,
                "reply_markup": {"force_reply": true},
                "reply_parameters": {"message_id": 1}
            }),
        ),
        SendLocation::new(1, 2.0, 3.0)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_heading(120)
            .with_horizontal_accuracy(1.5)
            .with_live_period(100)
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_proximity_alert_radius(100)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_parameters(ReplyParameters::new(1)),
    );
}

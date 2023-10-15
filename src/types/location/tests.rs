use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, ForceReply, Location, ProximityAlertTriggered, SendLocation, User},
};

#[test]
fn location() {
    assert_json_eq(
        Location {
            longitude: 1.0,
            latitude: 2.0,
            horizontal_accuracy: Some(3.0),
            live_period: Some(4),
            heading: Some(5),
            proximity_alert_radius: Some(6),
        },
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
        Location {
            longitude: 1.0,
            latitude: 2.0,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        },
        serde_json::json!({
            "longitude": 1.0,
            "latitude": 2.0
        }),
    );
}

#[test]
fn proximity_alert_triggered() {
    assert_json_eq(
        ProximityAlertTriggered {
            traveler: User {
                id: 1,
                is_bot: false,
                first_name: String::from("1"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
            },
            watcher: User {
                id: 2,
                is_bot: false,
                first_name: String::from("2"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
            },
            distance: 10,
        },
        serde_json::json!({
            "traveler": {
                "id": 1,
                "first_name": "1",
                "is_bot": false
            },
            "watcher": {
                "id": 2,
                "first_name": "2",
                "is_bot": false
            },
            "distance": 10
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
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {"force_reply": true}
            }),
        ),
        SendLocation::new(1, 2.0, 3.0)
            .horizontal_accuracy(1.5)
            .live_period(100)
            .heading(120)
            .proximity_alert_radius(100)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true)),
    );
}

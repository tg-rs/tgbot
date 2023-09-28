use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ForceReply, Location, ProximityAlertTriggered, SendLocation},
};

#[test]
fn location_deserialize_full() {
    let data: Location = serde_json::from_value(serde_json::json!({
        "longitude": 2.5,
        "latitude": 2.6,
        "horizontal_accuracy": 0.4,
        "live_period": 1,
        "heading": 45,
        "proximity_alert_radius": 20
    }))
    .unwrap();
    assert_eq!(data.longitude, 2.5);
    assert_eq!(data.latitude, 2.6);
    assert_eq!(data.horizontal_accuracy.unwrap(), 0.4);
    assert_eq!(data.live_period.unwrap(), 1);
    assert_eq!(data.heading.unwrap(), 45);
    assert_eq!(data.proximity_alert_radius.unwrap(), 20);
}

#[test]
fn location_deserialize_partial() {
    let data: Location = serde_json::from_value(serde_json::json!({
        "longitude": 2.5,
        "latitude": 2.6,
    }))
    .unwrap();
    assert_eq!(data.longitude, 2.5);
    assert_eq!(data.latitude, 2.6);
    assert!(data.horizontal_accuracy.is_none());
    assert!(data.live_period.is_none());
    assert!(data.heading.is_none());
    assert!(data.proximity_alert_radius.is_none());
}

#[test]
fn proximity_alert_triggered_deserialize() {
    let data: ProximityAlertTriggered = serde_json::from_value(serde_json::json!({
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
    }))
    .unwrap();
    assert_eq!(data.traveler.id, 1);
    assert_eq!(data.watcher.id, 2);
    assert_eq!(data.distance, 10);
}

#[allow(clippy::float_cmp)]
#[test]
fn send_location_full() {
    let request = SendLocation::new(1, 2.0, 3.0)
        .horizontal_accuracy(1.5)
        .live_period(100)
        .heading(120)
        .proximity_alert_radius(100)
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendLocation");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
            })
        )
    } else {
        panic!("Unexpected request body");
    }
}

#[allow(clippy::float_cmp)]
#[test]
fn send_location_partial() {
    let request = SendLocation::new(1, 2.0, 3.0).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendLocation");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "chat_id": 1,
                "latitude": 2.0,
                "longitude": 3.0
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

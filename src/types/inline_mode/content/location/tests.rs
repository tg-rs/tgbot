use crate::types::{InputMessageContent, InputMessageContentLocation};

#[test]
fn input_message_content_location_serialize_full() {
    let val = serde_json::to_value(InputMessageContent::from(
        InputMessageContentLocation::new(1.0, 2.0)
            .horizontal_accuracy(1.5)
            .live_period(100)
            .heading(90)
            .proximity_alert_radius(100),
    ))
    .unwrap();
    assert_eq!(
        val,
        serde_json::json!({
            "latitude": 1.0,
            "longitude": 2.0,
            "horizontal_accuracy": 1.5,
            "live_period": 100,
            "heading": 90,
            "proximity_alert_radius": 100
        })
    );
}

#[test]
fn input_message_content_location_serialize_partial() {
    let val = serde_json::to_value(InputMessageContent::from(InputMessageContentLocation::new(1.0, 2.0))).unwrap();
    assert_eq!(
        val,
        serde_json::json!({
            "latitude": 1.0,
            "longitude": 2.0
        })
    );
}

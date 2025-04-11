use crate::types::{InputMessageContent, InputMessageContentLocation, tests::assert_json_eq};

#[test]
fn input_message_content_location() {
    assert_json_eq(
        InputMessageContent::from(
            InputMessageContentLocation::new(1.0, 2.0)
                .with_heading(90)
                .with_horizontal_accuracy(1.5)
                .with_live_period(100)
                .with_proximity_alert_radius(100),
        ),
        serde_json::json!({
            "latitude": 1.0,
            "longitude": 2.0,
            "horizontal_accuracy": 1.5,
            "live_period": 100,
            "heading": 90,
            "proximity_alert_radius": 100
        }),
    );
    assert_json_eq(
        InputMessageContent::from(InputMessageContentLocation::new(1.0, 2.0)),
        serde_json::json!({
            "latitude": 1.0,
            "longitude": 2.0
        }),
    );
}

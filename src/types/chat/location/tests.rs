use crate::types::{tests::assert_json_eq, ChatLocation, Location};

#[test]
fn chat_location() {
    assert_json_eq(
        ChatLocation {
            location: Location {
                longitude: 0.0,
                latitude: 1.0,
                horizontal_accuracy: None,
                live_period: None,
                heading: None,
                proximity_alert_radius: None,
            },
            address: String::from("Address"),
        },
        serde_json::json!({
            "location": {
                "longitude": 0.0,
                "latitude": 1.0
            },
            "address": "Address"
        }),
    );
}

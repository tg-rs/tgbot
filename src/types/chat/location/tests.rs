use crate::types::{tests::assert_json_eq, ChatLocation, Location};

#[test]
fn chat_location() {
    assert_json_eq(
        ChatLocation::new("Address", Location::new(1.0, 0.0)),
        serde_json::json!({
            "location": {
                "longitude": 0.0,
                "latitude": 1.0
            },
            "address": "Address"
        }),
    );
}

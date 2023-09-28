use crate::types::ChatLocation;

#[test]
fn chat_location_deserialize() {
    let data: ChatLocation = serde_json::from_value(serde_json::json!({
        "location": {
            "longitude": 0.0,
            "latitude": 1.0
        },
        "address": "address"
    }))
    .unwrap();
    assert_eq!(data.location.longitude, 0.0);
    assert_eq!(data.location.latitude, 1.0);
    assert_eq!(data.address, "address");
}

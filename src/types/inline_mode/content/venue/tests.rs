use crate::types::{InputMessageContent, InputMessageContentVenue};

#[test]
fn input_message_content_venue_serialize_full() {
    let val = serde_json::to_value(InputMessageContent::from(
        InputMessageContentVenue::new(1.0, 2.0, "title", "addr")
            .foursquare_id("f-id")
            .foursquare_type("f-type")
            .google_place_id("g-id")
            .google_place_type("g-type"),
    ))
    .unwrap();
    assert_eq!(
        val,
        serde_json::json!({
            "latitude": 1.0,
            "longitude": 2.0,
            "title": "title",
            "address": "addr",
            "foursquare_id": "f-id",
            "foursquare_type": "f-type",
            "google_place_id": "g-id",
            "google_place_type": "g-type"
        }),
    );
}

#[test]
fn input_message_content_venue_serialize_partial() {
    let val = serde_json::to_value(InputMessageContent::from(InputMessageContentVenue::new(
        1.0, 2.0, "title", "addr",
    )))
    .unwrap();
    assert_eq!(
        val,
        serde_json::json!({
            "latitude": 1.0,
            "longitude": 2.0,
            "title": "title",
            "address": "addr"
        }),
    );
}

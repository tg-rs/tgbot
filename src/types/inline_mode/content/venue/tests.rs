use crate::{
    tests::assert_json_eq,
    types::{InputMessageContent, InputMessageContentVenue},
};

#[test]
fn input_message_content_venue() {
    let content = InputMessageContentVenue::new(1.0, 2.0, "title", "addr");
    assert_json_eq(
        InputMessageContent::from(
            content
                .clone()
                .foursquare_id("f-id")
                .foursquare_type("f-type")
                .google_place_id("g-id")
                .google_place_type("g-type"),
        ),
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
    assert_json_eq(
        InputMessageContent::from(content),
        serde_json::json!({
            "latitude": 1.0,
            "longitude": 2.0,
            "title": "title",
            "address": "addr"
        }),
    );
}

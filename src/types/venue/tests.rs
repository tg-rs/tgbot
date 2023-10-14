use crate::{
    tests::{assert_json_eq, assert_request_eq, ExpectedRequest},
    types::{ForceReply, Location, SendVenue, Venue},
};

#[test]
fn venue() {
    assert_json_eq(
        Venue {
            location: Location {
                latitude: 1.0,
                longitude: 2.0,
                heading: None,
                horizontal_accuracy: None,
                live_period: None,
                proximity_alert_radius: None,
            },
            title: String::from("venue title"),
            address: String::from("venue address"),
            foursquare_id: Some(String::from("f-id")),
            foursquare_type: Some(String::from("f-type")),
            google_place_id: Some(String::from("g-id")),
            google_place_type: Some(String::from("g-type")),
        },
        serde_json::json!({
            "location": {
                "latitude": 1.0,
                "longitude": 2.0
            },
            "title": "venue title",
            "address": "venue address",
            "foursquare_id": "f-id",
            "foursquare_type": "f-type",
            "google_place_id": "g-id",
            "google_place_type": "g-type"
        }),
    );
    assert_json_eq(
        Venue {
            location: Location {
                latitude: 1.0,
                longitude: 2.0,
                heading: None,
                horizontal_accuracy: None,
                live_period: None,
                proximity_alert_radius: None,
            },
            title: String::from("venue title"),
            address: String::from("venue address"),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        },
        serde_json::json!({
            "location": {
                "latitude": 1.0,
                "longitude": 2.0
            },
            "title": "venue title",
            "address": "venue address",
        }),
    );
}

#[test]
fn send_venue() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "sendVenue",
            serde_json::json!({
                "chat_id": 1,
                "latitude": 2.0,
                "longitude": 3.0,
                "title": "title",
                "address": "addr",
                "foursquare_id": "f-id",
                "foursquare_type": "f-type",
                "google_place_id": "g-id",
                "google_place_type": "g-type",
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {"force_reply": true}
            }),
        ),
        SendVenue::new(1, 2.0, 3.0, "title", "addr")
            .foursquare_id("f-id")
            .foursquare_type("f-type")
            .google_place_id("g-id")
            .google_place_type("g-type")
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true)),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "sendVenue",
            serde_json::json!({
                "chat_id": 1,
                "latitude": 2.0,
                "longitude": 3.0,
                "title": "title",
                "address": "addr",
            }),
        ),
        SendVenue::new(1, 2.0, 3.0, "title", "addr"),
    );
}

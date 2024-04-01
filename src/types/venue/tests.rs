use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, ForceReply, Location, ReplyParameters, SendVenue, Venue},
};

#[test]
fn venue() {
    assert_json_eq(
        Venue::new("venue title", "venue address", Location::new(1.0, 2.0))
            .with_foursquare_id("f-id")
            .with_foursquare_type("f-type")
            .with_google_place_id("g-id")
            .with_google_place_type("g-type"),
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
        Venue::new("venue title", "venue address", Location::new(1.0, 2.0)),
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
    assert_payload_eq(
        Payload::json(
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
                "business_connection_id": "id",
                "disable_notification": true,
                "protect_content": true,
                "message_thread_id": 1,
                "reply_markup": {"force_reply": true},
                "reply_parameters": {
                    "message_id": 1
                }
            }),
        ),
        SendVenue::new(1, 2.0, 3.0, "title", "addr")
            .with_foursquare_id("f-id")
            .with_foursquare_type("f-type")
            .with_google_place_id("g-id")
            .with_google_place_type("g-type")
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_protect_content(true)
            .with_message_thread_id(1)
            .with_reply_markup(ForceReply::new(true))
            .with_reply_parameters(ReplyParameters::new(1)),
    );
    assert_payload_eq(
        Payload::json(
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

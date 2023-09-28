#![allow(clippy::float_cmp)]

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{ForceReply, SendVenue, Venue},
};

#[test]
fn venue_deserialize_full() {
    let data: Venue = serde_json::from_value(serde_json::json!({
        "location": {
            "latitude": 1.1,
            "longitude": 2.0
        },
        "title": "venue title",
        "address": "venue address",
        "foursquare_id": "f-id",
        "foursquare_type": "f-type",
        "google_place_id": "g-id",
        "google_place_type": "g-type"
    }))
    .unwrap();

    assert_eq!(data.location.latitude, 1.1);
    assert_eq!(data.location.longitude, 2.0);
    assert_eq!(data.title, "venue title");
    assert_eq!(data.address, "venue address");
    assert_eq!(data.foursquare_id.unwrap(), "f-id");
    assert_eq!(data.foursquare_type.unwrap(), "f-type");
    assert_eq!(data.google_place_id.unwrap(), "g-id");
    assert_eq!(data.google_place_type.unwrap(), "g-type");
}

#[test]
fn venue_deserialize_partial() {
    let data: Venue = serde_json::from_value(serde_json::json!({
        "location": {
            "latitude": 1.1,
            "longitude": 2.0
        },
        "title": "venue title",
        "address": "venue address"
    }))
    .unwrap();

    assert_eq!(data.location.latitude, 1.1);
    assert_eq!(data.location.longitude, 2.0);
    assert_eq!(data.title, "venue title");
    assert_eq!(data.address, "venue address");
    assert!(data.foursquare_id.is_none());
    assert!(data.foursquare_type.is_none());
    assert!(data.google_place_id.is_none());
    assert!(data.google_place_type.is_none());
}

#[test]
fn send_venue() {
    let request = SendVenue::new(1, 2.0, 3.0, "title", "addr")
        .foursquare_id("f-id")
        .foursquare_type("f-type")
        .google_place_id("g-id")
        .google_place_type("g-type")
        .disable_notification(true)
        .protect_content(true)
        .reply_to_message_id(1)
        .allow_sending_without_reply(true)
        .reply_markup(ForceReply::new(true))
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendVenue");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

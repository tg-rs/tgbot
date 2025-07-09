use crate::types::*;

#[test]
fn venue() {
    insta::assert_json_snapshot!(
        Venue::new("venue title", "venue address", Location::new(1.0, 2.0))
            .with_foursquare_id("f-id")
            .with_foursquare_type("f-type")
            .with_google_place_id("g-id")
            .with_google_place_type("g-type")
    );
    insta::assert_json_snapshot!(Venue::new("venue title", "venue address", Location::new(1.0, 2.0)));
}

#[test]
fn send_venue() {
    let method = SendVenue::new(1, 2.0, 3.0, "title", "addr")
        .with_foursquare_id("f-id")
        .with_foursquare_type("f-type")
        .with_google_place_id("g-id")
        .with_google_place_type("g-type")
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_disable_notification(true)
        .with_protect_content(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1));
    assert_payload_eq!(POST JSON "sendVenue" => method);
    let method = SendVenue::new(1, 2.0, 3.0, "title", "addr");
    assert_payload_eq!(POST JSON "sendVenue" => method);
}

#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("send-venue-base", SendVenue::new(1, 2.0, 3.0, "title", "addr"), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Venue(data) = x.data {
                assert_eq!(data.location.latitude, 1.0);
                assert_eq!(data.location.longitude, 2.0);
                assert_eq!(data.title, "title");
                assert_eq!(data.address, "addr");
            } else {
                panic!("Got an unexpected message data");
            }
        }),
        (
            "send-venue-all",
            SendVenue::new(1, 2.0, 3.0, "title", "addr")
                .with_foursquare_id("f-id")
                .with_foursquare_type("f-type")
                .with_google_place_id("g-id")
                .with_google_place_type("g-type")
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_ephemeral_message_parameters(EphemeralMessageParameters::from(999).with_callback_query_id("cqid"))
                .with_protect_content(true)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default()),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Venue(data) = x.data {
                    assert_eq!(data.location.latitude, 1.0);
                    assert_eq!(data.location.longitude, 2.0);
                    assert_eq!(data.title, "title");
                    assert_eq!(data.address, "addr");
                    assert_eq!(data.foursquare_id.unwrap(), "f-id");
                    assert_eq!(data.foursquare_type.unwrap(), "f-type");
                    assert_eq!(data.google_place_id.unwrap(), "g-id");
                    assert_eq!(data.google_place_type.unwrap(), "g-type");
                } else {
                    panic!("Got an unexpected message data");
                }
            },
        ),
    ])
    .await;
}

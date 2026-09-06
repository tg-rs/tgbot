#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("send-location-base", SendLocation::new(1, 2.0, 3.0), |x| {
            assert_eq!(x.id, 1);
            if let MessageData::Location(data) = x.data {
                assert_eq!(data.latitude, 2.0);
                assert_eq!(data.longitude, 3.0);
                assert!(data.horizontal_accuracy.is_none());
                assert!(data.live_period.is_none());
                assert!(data.heading.is_none());
                assert!(data.proximity_alert_radius.is_none());
            } else {
                panic!("Got an unexpected message data: {:?}", x.data);
            }
        }),
        (
            "send-location-all",
            SendLocation::new(1, 2.0, 3.0)
                .with_allow_paid_broadcast(true)
                .with_business_connection_id("id")
                .with_direct_messages_topic_id(1)
                .with_disable_notification(true)
                .with_ephemeral_message_parameters(EphemeralMessageParameters::from(999).with_callback_query_id("cqid"))
                .with_heading(120)
                .with_horizontal_accuracy(1.5)
                .with_live_period(100)
                .with_message_effect_id("effect-id")
                .with_message_thread_id(1)
                .with_protect_content(true)
                .with_proximity_alert_radius(100)
                .with_reply_markup(ForceReply::new(true))
                .with_reply_parameters(ReplyParameters::new(1))
                .with_suggested_post_parameters(SuggestedPostParameters::default()),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Location(data) = x.data {
                    assert_eq!(data.latitude, 2.0);
                    assert_eq!(data.longitude, 3.0);
                    assert_eq!(data.horizontal_accuracy.unwrap(), 1.5);
                    assert_eq!(data.live_period.unwrap(), 100);
                    assert_eq!(data.heading.unwrap(), 120);
                    assert_eq!(data.proximity_alert_radius.unwrap(), 100);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;
}

#![allow(missing_docs)]
use std::io::Cursor;

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("delete-webhook-base", DeleteWebhook::default(), |x| assert!(x)),
        (
            "delete-webhook-full",
            DeleteWebhook::default().with_drop_pending_updates(true),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("get-webhook-info-base", GetWebhookInfo, |x| {
            assert_eq!(x.url, "https://example.com");
            assert!(!x.has_custom_certificate);
            assert_eq!(x.pending_update_count, 0);
            assert!(x.ip_address.is_none());
            assert!(x.last_error_date.is_none());
            assert!(x.last_error_message.is_none());
            assert!(x.last_synchronization_error_date.is_none());
            assert!(x.max_connections.is_none());
            assert!(x.allowed_updates.is_none());
        }),
        ("get-webhook-info-full", GetWebhookInfo, |x| {
            assert_eq!(x.url, "https://example.com");
            assert!(!x.has_custom_certificate);
            assert_eq!(x.pending_update_count, 0);
            assert_eq!(x.ip_address.unwrap(), "127.0.0.1");
            assert_eq!(x.last_error_date.unwrap(), 0);
            assert_eq!(x.last_error_message.unwrap(), "test");
            assert_eq!(x.last_synchronization_error_date.unwrap(), 0);
            assert_eq!(x.max_connections.unwrap(), 10);
            assert_eq!(x.allowed_updates.unwrap(), [AllowedUpdate::Message]);
        }),
    ])
    .await;
    cx.execute_batch([
        ("set-webhook-base", SetWebhook::new("https://example.com"), |x| {
            assert!(x)
        }),
        (
            "set-webhook-full",
            SetWebhook::new("https://example.com")
                .with_certificate(Cursor::new(b"cert"))
                .with_ip_address("127.0.0.1")
                .with_max_connections(10)
                .with_allowed_updates([AllowedUpdate::Message])
                .with_drop_pending_updates(true)
                .with_secret_token("secret-token"),
            |x| assert!(x),
        ),
    ])
    .await;
}

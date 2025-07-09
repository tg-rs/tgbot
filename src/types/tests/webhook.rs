use std::collections::HashSet;

use crate::types::*;

#[test]
fn webhook_info() {
    insta::assert_json_snapshot!(
        WebhookInfo::new("https://example.com/tg-webhook", true, 1)
            .with_ip_address("127.0.0.1")
            .with_last_error_date(0)
            .with_last_error_message("error")
            .with_last_synchronization_error_date(0)
            .with_max_connections(10)
            .with_allowed_updates([AllowedUpdate::Message, AllowedUpdate::Poll])
    );
    insta::assert_json_snapshot!(WebhookInfo::new("https://example.com/tg-webhook", true, 1));
}

#[test]
fn delete_webhook() {
    assert_payload_eq!(GET "deleteWebhook" => DeleteWebhook::default());
    let method = DeleteWebhook::default().with_drop_pending_updates(false);
    assert_payload_eq!(POST JSON "deleteWebhook" => method);
}

#[test]
fn get_webhook_info() {
    assert_payload_eq!(GET "getWebhookInfo" => GetWebhookInfo);
}

#[test]
fn set_webhook() {
    assert_payload_eq!(POST JSON "setWebhook" => SetWebhook::new("url"));

    let mut updates = HashSet::new();
    updates.insert(AllowedUpdate::Message);
    let method = SetWebhook::new("url")
        .with_certificate("cert")
        .with_ip_address("127.0.0.1")
        .with_max_connections(10)
        .with_allowed_updates(updates)
        .add_allowed_update(AllowedUpdate::Message)
        .with_drop_pending_updates(true)
        .with_secret_token("secret-token");
    assert_payload_eq!(POST JSON "setWebhook" => method);
}

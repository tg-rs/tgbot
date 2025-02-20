use std::collections::HashSet;

use crate::{
    api::{Payload, assert_payload_eq},
    types::{AllowedUpdate, DeleteWebhook, GetWebhookInfo, SetWebhook, WebhookInfo, tests::assert_json_eq},
};

#[test]
fn webhook_info() {
    assert_json_eq(
        WebhookInfo::new("https://example.com/tg-webhook", true, 1)
            .with_ip_address("127.0.0.1")
            .with_last_error_date(0)
            .with_last_error_message("error")
            .with_last_synchronization_error_date(0)
            .with_max_connections(10)
            .with_allowed_updates([AllowedUpdate::Message, AllowedUpdate::Poll]),
        serde_json::json!({
            "url": "https://example.com/tg-webhook",
            "has_custom_certificate": true,
            "pending_update_count": 1,
            "ip_address": "127.0.0.1",
            "last_error_date": 0,
            "last_error_message": "error",
            "last_synchronization_error_date": 0,
            "max_connections": 10,
            "allowed_updates": ["message", "poll"]
        }),
    );
    assert_json_eq(
        WebhookInfo::new("https://example.com/tg-webhook", true, 1),
        serde_json::json!({
            "url": "https://example.com/tg-webhook",
            "has_custom_certificate": true,
            "pending_update_count": 1
        }),
    );
}

#[test]
fn delete_webhook() {
    assert_payload_eq(Payload::empty("deleteWebhook"), DeleteWebhook::default());
    assert_payload_eq(
        Payload::json(
            "deleteWebhook",
            serde_json::json!({
                "drop_pending_updates": false
            }),
        ),
        DeleteWebhook::default().with_drop_pending_updates(false),
    );
}

#[test]
fn get_webhook_info() {
    assert_payload_eq(Payload::empty("getWebhookInfo"), GetWebhookInfo);
}

#[test]
fn set_webhook() {
    assert_payload_eq(
        Payload::json(
            "setWebhook",
            serde_json::json!({
                "url": "url",
            }),
        ),
        SetWebhook::new("url"),
    );

    let mut updates = HashSet::new();
    updates.insert(AllowedUpdate::Message);
    assert_payload_eq(
        Payload::json(
            "setWebhook",
            serde_json::json!({
                "url": "url",
                "certificate": "cert",
                "ip_address": "127.0.0.1",
                "max_connections": 10,
                "drop_pending_updates": true,
                "allowed_updates": ["message"],
                "secret_token": "secret-token"
            }),
        ),
        SetWebhook::new("url")
            .with_certificate("cert")
            .with_ip_address("127.0.0.1")
            .with_max_connections(10)
            .with_allowed_updates(updates)
            .add_allowed_update(AllowedUpdate::Message)
            .with_drop_pending_updates(true)
            .with_secret_token("secret-token"),
    );
}

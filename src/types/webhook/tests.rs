use std::collections::HashSet;

use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, AllowedUpdate, DeleteWebhook, GetWebhookInfo, SetWebhook, WebhookInfo},
};

#[test]
fn webhook_info() {
    assert_json_eq(
        WebhookInfo {
            url: String::from("https://example.com/tg-webhook"),
            has_custom_certificate: true,
            pending_update_count: 1,
            ip_address: Some(String::from("127.0.0.1")),
            last_error_date: Some(0),
            last_error_message: Some(String::from("error")),
            last_synchronization_error_date: Some(0),
            max_connections: Some(10),
            allowed_updates: Some(vec![AllowedUpdate::Message, AllowedUpdate::Poll]),
        },
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
        WebhookInfo {
            url: String::from("https://example.com/tg-webhook"),
            has_custom_certificate: true,
            pending_update_count: 1,
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            last_synchronization_error_date: None,
            max_connections: None,
            allowed_updates: None,
        },
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
        DeleteWebhook::default().drop_pending_updates(false),
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
                "allowed_updates": ["message"]
            }),
        ),
        SetWebhook::new("url")
            .certificate("cert")
            .ip_address("127.0.0.1")
            .max_connections(10)
            .allowed_updates(updates)
            .add_allowed_update(AllowedUpdate::Message)
            .drop_pending_updates(true),
    );
}

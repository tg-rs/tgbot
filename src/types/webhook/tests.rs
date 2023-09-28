use std::collections::HashSet;

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{AllowedUpdate, DeleteWebhook, GetWebhookInfo, SetWebhook, WebhookInfo},
};

#[test]
fn webhook_info_deserialize_full() {
    let data: WebhookInfo = serde_json::from_value(serde_json::json!({
        "url": "https://example.com/tg-webhook",
        "has_custom_certificate": true,
        "pending_update_count": 1,
        "ip_address": "127.0.0.1",
        "last_error_date": 0,
        "last_error_message": "error",
        "max_connections": 10,
        "allowed_updates": ["message", "poll"]
    }))
    .unwrap();
    assert_eq!(data.url, "https://example.com/tg-webhook");
    assert!(data.has_custom_certificate);
    assert_eq!(data.pending_update_count, 1);
    assert_eq!(data.ip_address.unwrap(), "127.0.0.1");
    assert_eq!(data.last_error_date.unwrap(), 0);
    assert_eq!(data.last_error_message.unwrap(), "error");
    assert_eq!(data.max_connections.unwrap(), 10);
    let allowed = data.allowed_updates.unwrap();
    assert_eq!(allowed.len(), 2);
    assert_eq!(&allowed[0], &AllowedUpdate::Message);
    assert_eq!(&allowed[1], &AllowedUpdate::Poll);
}

#[test]
fn webhook_info_deserialize_partial() {
    let data: WebhookInfo = serde_json::from_value(serde_json::json!({
        "url": "https://example.com/tg-webhook",
        "has_custom_certificate": true,
        "pending_update_count": 1
    }))
    .unwrap();
    assert_eq!(data.url, "https://example.com/tg-webhook");
    assert!(data.has_custom_certificate);
    assert_eq!(data.pending_update_count, 1);
    assert!(data.ip_address.is_none());
    assert!(data.last_error_date.is_none());
    assert!(data.last_error_message.is_none());
    assert!(data.max_connections.is_none());
    assert!(data.allowed_updates.is_none());
}

#[test]
fn delete_webhook() {
    let request = DeleteWebhook::default().into_request();
    assert_eq!(request.get_method(), RequestMethod::Get);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/deleteWebhook"
    );
    match request.into_body() {
        RequestBody::Empty => {}
        data => panic!("Unexpected request data: {:?}", data),
    }

    let request = DeleteWebhook::default().drop_pending_updates(false).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/deleteWebhook"
    );
    match request.into_body() {
        RequestBody::Json(data) => {
            assert_eq!(data.unwrap(), r#"{"drop_pending_updates":false}"#);
        }
        data => panic!("Unexpected request data: {:?}", data),
    }
}

#[test]
fn get_webhook_info() {
    let request = GetWebhookInfo.into_request();
    assert_eq!(request.get_method(), RequestMethod::Get);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/getWebhookInfo"
    );
    match request.into_body() {
        RequestBody::Empty => {}
        data => panic!("Unexpected request data: {:?}", data),
    }
}

#[test]
fn set_webhook() {
    let request = SetWebhook::new("url").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setWebhook");
    match request.into_body() {
        RequestBody::Json(data) => {
            assert_eq!(data.unwrap(), r#"{"url":"url"}"#);
        }
        data => panic!("Unexpected request data: {:?}", data),
    }

    let mut updates = HashSet::new();
    updates.insert(AllowedUpdate::Message);
    updates.insert(AllowedUpdate::Message);
    updates.insert(AllowedUpdate::EditedMessage);
    updates.insert(AllowedUpdate::ChannelPost);
    updates.insert(AllowedUpdate::EditedChannelPost);
    updates.insert(AllowedUpdate::ChosenInlineResult);
    let request = SetWebhook::new("url")
        .certificate("cert")
        .ip_address("127.0.0.1")
        .max_connections(10)
        .allowed_updates(updates)
        .add_allowed_update(AllowedUpdate::InlineQuery)
        .add_allowed_update(AllowedUpdate::CallbackQuery)
        .add_allowed_update(AllowedUpdate::PreCheckoutQuery)
        .add_allowed_update(AllowedUpdate::ShippingQuery)
        .drop_pending_updates(true)
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setWebhook");
    match request.into_body() {
        RequestBody::Json(data) => {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["certificate"], "cert");
            assert_eq!(data["ip_address"], "127.0.0.1");
            assert_eq!(data["max_connections"], 10);
            assert!(data["drop_pending_updates"].as_bool().unwrap());
            let mut updates: Vec<&str> = data["allowed_updates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap())
                .collect();
            updates.sort_unstable();
            assert_eq!(
                updates,
                vec![
                    "callback_query",
                    "channel_post",
                    "chosen_inline_result",
                    "edited_channel_post",
                    "edited_message",
                    "inline_query",
                    "message",
                    "pre_checkout_query",
                    "shipping_query",
                ]
            );
        }
        data => panic!("Unexpected request data: {:?}", data),
    }

    let method = SetWebhook::new("url").add_allowed_update(AllowedUpdate::Message);
    assert_eq!(method.allowed_updates.unwrap().len(), 1);
}

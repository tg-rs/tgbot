use serde::Serialize;

use crate::{
    api::{Payload, assert_payload_eq},
    types::{
        CreateInvoiceLink,
        InlineKeyboardButton,
        Invoice,
        InvoiceParameters,
        LabeledPrice,
        ReplyParameters,
        SendInvoice,
        tests::assert_json_eq,
    },
};

#[test]
fn invoice() {
    assert_json_eq(
        Invoice::new(
            "RUB",
            "invoice description",
            "invoice start parameter",
            "invoice title",
            100,
        ),
        serde_json::json!({
            "title": "invoice title",
            "description": "invoice description",
            "start_parameter": "invoice start parameter",
            "currency": "RUB",
            "total_amount": 100
        }),
    );
}

#[test]
fn labeled_price() {
    assert_json_eq(
        LabeledPrice::new(145, "price label"),
        serde_json::json!({"label": "price label", "amount": 145}),
    );
}

#[derive(Serialize)]
struct ProviderData {
    key: String,
}

#[test]
fn create_invoice_link() {
    let method = CreateInvoiceLink::new(
        "product-name",
        "product-description",
        "payload",
        "GEL",
        [LabeledPrice::new(100, "price-label")],
    );
    assert_payload_eq(
        Payload::json(
            "createInvoiceLink",
            serde_json::json!({
                "title": "product-name",
                "description": "product-description",
                "payload": "payload",
                "currency": "GEL",
                "prices": [
                    {
                        "label": "price-label",
                        "amount": 100
                    }
                ]
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "createInvoiceLink",
            serde_json::json!({
                "title": "product-name",
                "description": "product-description",
                "payload": "payload",
                "currency": "GEL",
                "prices": [
                    {
                        "label": "price-label",
                        "amount": 100
                    }
                ],
                "provider_token": "provider-token",
                "max_tip_amount": 100,
                "business_connection_id": "id",
                "subscription_period": 1,
            }),
        ),
        method
            .with_business_connection_id("id")
            .with_parameters(
                InvoiceParameters::default()
                    .with_max_tip_amount(100)
                    .with_provider_token("provider-token"),
            )
            .with_subscription_period(1),
    );
}

#[test]
fn send_invoice() {
    assert_payload_eq(
        Payload::json(
            "sendInvoice",
            serde_json::json!({
                "chat_id": "@username",
                "title": "title",
                "description": "description",
                "payload": "payload",
                "currency": "RUB",
                "prices": [
                    {
                        "label": "item",
                        "amount": 100
                    }
                ],
            }),
        ),
        SendInvoice::new(
            "@username",
            "title",
            "description",
            "payload",
            "RUB",
            vec![LabeledPrice::new(100, "item")],
        ),
    );
    assert_payload_eq(
        Payload::json(
            "sendInvoice",
            serde_json::json!({
                "chat_id": 1,
                "title": "title",
                "description": "description",
                "payload": "payload",
                "provider_token": "token",
                "start_parameter": "param",
                "currency": "RUB",
                "prices": [],
                "max_tip_amount": 100,
                "suggested_tip_amounts": [10, 50, 100],
                "provider_data": "{\"key\":\"value\"}",
                "photo_url": "url",
                "photo_size": 100,
                "photo_width": 200,
                "photo_height": 300,
                "need_name": true,
                "need_phone_number": true,
                "need_email": true,
                "need_shipping_address": true,
                "send_phone_number_to_provider": true,
                "send_email_to_provider": true,
                "is_flexible": true,
                "allow_paid_broadcast": true,
                "disable_notification": true,
                "message_effect_id": "effect-id",
                "message_thread_id": 1,
                "protect_content": true,
                "reply_markup": {
                    "inline_keyboard": [[
                        {"text": "text", "url": "url"}
                    ]]
                },
                "reply_parameters": {
                    "message_id": 1
                }
            }),
        ),
        SendInvoice::new(1, "title", "description", "payload", "RUB", vec![])
            .with_allow_paid_broadcast(true)
            .with_disable_notification(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_parameters(
                InvoiceParameters::default()
                    .with_max_tip_amount(100)
                    .with_need_email(true)
                    .with_need_name(true)
                    .with_need_phone_number(true)
                    .with_need_shipping_address(true)
                    .with_photo_height(300)
                    .with_photo_size(100)
                    .with_photo_url("url")
                    .with_photo_width(200)
                    .with_provider_data(&ProviderData {
                        key: String::from("value"),
                    })
                    .unwrap()
                    .with_provider_token("token")
                    .with_send_email_to_provider(true)
                    .with_send_phone_number_to_provider(true)
                    .with_suggested_tip_amounts(vec![10, 50, 100])
                    .with_flexible(true),
            )
            .with_protect_content(true)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_reply_parameters(ReplyParameters::new(1))
            .with_start_parameter("param"),
    );
}

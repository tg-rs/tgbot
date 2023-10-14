use serde::Serialize;

use crate::{
    tests::{assert_json_eq, assert_request_eq, ExpectedRequest},
    types::{InlineKeyboardButton, Invoice, LabeledPrice, SendInvoice},
};

#[test]
fn invoice() {
    assert_json_eq(
        Invoice {
            title: String::from("invoice title"),
            description: String::from("invoice description"),
            start_parameter: String::from("invoice start parameter"),
            currency: String::from("RUB"),
            total_amount: 100,
        },
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
        LabeledPrice::new("price label", 145),
        serde_json::json!({"label": "price label", "amount": 145}),
    );
}

#[derive(Serialize)]
struct ProviderData {
    key: String,
}

#[test]
fn send_invoice() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "sendInvoice",
            serde_json::json!({
                "chat_id": "@username",
                "title": "title",
                "description": "description",
                "payload": "payload",
                "provider_token": "token",
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
            "token",
            "RUB",
            vec![LabeledPrice::new("item", 100)],
        ),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
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
                "disable_notification": true,
                "protect_content": true,
                "reply_to_message_id": 1,
                "allow_sending_without_reply": true,
                "reply_markup": {
                    "inline_keyboard": [[
                        {"text": "text", "url": "url"}
                    ]]
                }
            }),
        ),
        SendInvoice::new(1, "title", "description", "payload", "token", "RUB", vec![])
            .start_parameter("param")
            .max_tip_amount(100)
            .suggested_tip_amounts(vec![10, 50, 100])
            .provider_data(&ProviderData {
                key: String::from("value"),
            })
            .unwrap()
            .photo_url("url")
            .photo_size(100)
            .photo_width(200)
            .photo_height(300)
            .need_name(true)
            .need_phone_number(true)
            .need_email(true)
            .need_shipping_address(true)
            .send_phone_number_to_provider(true)
            .send_email_to_provider(true)
            .flexible(true)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]]),
    );
}

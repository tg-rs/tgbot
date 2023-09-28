use serde::Serialize;
use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{InlineKeyboardButton, Invoice, LabeledPrice, SendInvoice},
};

#[test]
fn invoice_deserialize() {
    let data: Invoice = serde_json::from_value(serde_json::json!({
        "title": "invoice title",
        "description": "invoice description",
        "start_parameter": "invoice start parameter",
        "currency": "RUB",
        "total_amount": 100
    }))
    .unwrap();
    assert_eq!(data.title, "invoice title");
    assert_eq!(data.description, "invoice description");
    assert_eq!(data.start_parameter, "invoice start parameter");
    assert_eq!(data.currency, "RUB");
    assert_eq!(data.total_amount, 100);
}

#[test]
fn labeled_price_serialize() {
    let price = LabeledPrice::new("price label", 145);
    let data = serde_json::to_string(&price).unwrap();
    let new_price: serde_json::Value = serde_json::from_str(&data).unwrap();
    assert_eq!(price.label(), new_price.get("label").unwrap().as_str().unwrap());
    assert_eq!(price.amount(), new_price.get("amount").unwrap().as_i64().unwrap());
}

#[derive(Serialize)]
struct ProviderData {
    key: String,
}

#[test]
fn serialize_send_invoice_partial() {
    let request = SendInvoice::new(
        "@username",
        "title",
        "description",
        "payload",
        "token",
        "RUB",
        vec![LabeledPrice::new("item", 100)],
    )
    .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendInvoice");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
            })
        );
    }
}

#[test]
fn serialize_send_invoice_full() {
    let request = SendInvoice::new(1, "title", "description", "payload", "token", "RUB", vec![])
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
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendInvoice");
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(
            data,
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
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{AnswerShippingQuery, ShippingAddress, ShippingOption, ShippingQuery, SuccessfulPayment},
};

#[test]
fn shipping_address_deserialize() {
    let data: ShippingAddress = serde_json::from_value(serde_json::json!({
        "country_code": "RU",
        "state": "Chechen Republic",
        "city": "Gudermes",
        "street_line1": "Nuradilov st., 12",
        "street_line2": "",
        "post_code": "366200"
    }))
    .unwrap();
    assert_eq!(data.country_code, "RU");
    assert_eq!(data.state, "Chechen Republic");
    assert_eq!(data.city, "Gudermes");
    assert_eq!(data.street_line1, "Nuradilov st., 12");
    assert_eq!(data.street_line2, "");
    assert_eq!(data.post_code, "366200");
}

#[test]
fn shipping_option_serialize() {
    let option = ShippingOption::new("id", "title", vec![]);
    let data = serde_json::to_string(&option).unwrap();
    let new_option: serde_json::Value = serde_json::from_str(&data).unwrap();
    assert_eq!(new_option.get("id").unwrap().as_str().unwrap(), option.id());
    assert_eq!(new_option.get("title").unwrap().as_str().unwrap(), option.title());
    assert!(new_option.get("prices").unwrap().as_array().unwrap().is_empty());
    assert!(option.prices().is_empty())
}

#[test]
fn shipping_query_deserialize() {
    let data: ShippingQuery = serde_json::from_value(serde_json::json!({
        "id": "query-id",
        "from": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "invoice_payload": "payload",
        "shipping_address": {
            "country_code": "RU",
            "state": "Chechen Republic",
            "city": "Gudermes",
            "street_line1": "Nuradilov st., 12",
            "street_line2": "",
            "post_code": "366200"
        }
    }))
    .unwrap();
    assert_eq!(data.id, "query-id");
    assert_eq!(data.from.id, 1);
    assert_eq!(data.invoice_payload, "payload");
    assert_eq!(data.shipping_address.country_code, "RU");
}

#[test]
fn successful_payment_deserialize_full() {
    let data: SuccessfulPayment = serde_json::from_value(serde_json::json!({
        "currency": "RUB",
        "total_amount": 145,
        "invoice_payload": "invoice payload",
        "shipping_option_id": "option id",
        "order_info": {},
        "telegram_payment_charge_id": "tg-charge-id",
        "provider_payment_charge_id": "provider-charge-id"
    }))
    .unwrap();
    assert_eq!(data.currency, "RUB");
    assert_eq!(data.total_amount, 145);
    assert_eq!(data.invoice_payload, "invoice payload");
    assert_eq!(data.shipping_option_id.unwrap(), "option id");
    assert!(data.order_info.unwrap().name.is_none());
    assert_eq!(data.telegram_payment_charge_id, "tg-charge-id");
    assert_eq!(data.provider_payment_charge_id, "provider-charge-id");
}

#[test]
fn successful_payment_deserialize_partial() {
    let data: SuccessfulPayment = serde_json::from_value(serde_json::json!({
        "currency": "RUB",
        "total_amount": 145,
        "invoice_payload": "invoice payload",
        "telegram_payment_charge_id": "tg-charge-id",
        "provider_payment_charge_id": "provider-charge-id"
    }))
    .unwrap();
    assert_eq!(data.currency, "RUB");
    assert_eq!(data.total_amount, 145);
    assert_eq!(data.invoice_payload, "invoice payload");
    assert!(data.shipping_option_id.is_none());
    assert!(data.order_info.is_none());
    assert_eq!(data.telegram_payment_charge_id, "tg-charge-id");
    assert_eq!(data.provider_payment_charge_id, "provider-charge-id");
}

#[test]
fn answer_shipping_query() {
    let request = AnswerShippingQuery::ok("id", vec![]).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/answerShippingQuery"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["shipping_query_id"], "id");
        assert!(data["ok"].as_bool().unwrap());
        assert!(data["shipping_options"].as_array().unwrap().is_empty());
    } else {
        panic!("Unexpected request body");
    }

    let request = AnswerShippingQuery::error("id", "msg").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/answerShippingQuery"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["shipping_query_id"], "id");
        assert!(!data["ok"].as_bool().unwrap());
        assert_eq!(data["error_message"], "msg");
    } else {
        panic!("Unexpected request body");
    }
}

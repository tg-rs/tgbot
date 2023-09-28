use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{AnswerPreCheckoutQuery, PreCheckoutQuery},
};

#[test]
fn pre_checkout_query_deserialize_full() {
    let data: PreCheckoutQuery = serde_json::from_value(serde_json::json!({
        "id": "query id",
        "from": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "currency": "GEL",
        "total_amount": 100,
        "invoice_payload": "invoice payload",
        "shipping_option_id": "option id",
        "order_info": {}
    }))
    .unwrap();
    assert_eq!(data.id, "query id");
    assert_eq!(data.from.id, 1);
    assert_eq!(data.currency, "GEL");
    assert_eq!(data.total_amount, 100);
    assert_eq!(data.invoice_payload, "invoice payload");
    assert_eq!(data.shipping_option_id.unwrap(), "option id");
    assert!(data.order_info.unwrap().name.is_none());
}

#[test]
fn pre_checkout_query_deserialize_partial() {
    let data: PreCheckoutQuery = serde_json::from_value(serde_json::json!({
        "id": "query id",
        "from": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "currency": "GEL",
        "total_amount": 100,
        "invoice_payload": "invoice payload"
    }))
    .unwrap();
    assert_eq!(data.id, "query id");
    assert_eq!(data.from.id, 1);
    assert_eq!(data.currency, "GEL");
    assert_eq!(data.total_amount, 100);
    assert_eq!(data.invoice_payload, "invoice payload");
    assert!(data.shipping_option_id.is_none());
    assert!(data.order_info.is_none());
}

#[test]
fn answer_pre_checkout_query() {
    let request = AnswerPreCheckoutQuery::ok("query-id").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/answerPreCheckoutQuery"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["pre_checkout_query_id"], "query-id");
        assert!(data["ok"].as_bool().unwrap());
    } else {
        panic!("Unexpected request body");
    }

    let request = AnswerPreCheckoutQuery::error("query-id", "msg").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/answerPreCheckoutQuery"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["pre_checkout_query_id"], "query-id");
        assert!(!data["ok"].as_bool().unwrap());
        assert_eq!(data["error_message"], "msg");
    } else {
        panic!("Unexpected request body");
    }
}

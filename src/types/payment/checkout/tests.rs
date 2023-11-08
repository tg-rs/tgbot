use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, AnswerPreCheckoutQuery, OrderInfo, PreCheckoutQuery, User},
};

#[test]
fn pre_checkout_query() {
    let expected_struct = PreCheckoutQuery::new("GEL", User::new(1, "User", false), "query id", "invoice payload", 100);
    assert_json_eq(
        expected_struct
            .clone()
            .with_shipping_option_id("option id")
            .with_order_info(OrderInfo::default()),
        serde_json::json!({
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "User",
                "is_bot": false
            },
            "currency": "GEL",
            "total_amount": 100,
            "invoice_payload": "invoice payload",
            "shipping_option_id": "option id",
            "order_info": {}
        }),
    );
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "User",
                "is_bot": false
            },
            "currency": "GEL",
            "total_amount": 100,
            "invoice_payload": "invoice payload"
        }),
    );
}

#[test]
fn answer_pre_checkout_query() {
    assert_payload_eq(
        Payload::json(
            "answerPreCheckoutQuery",
            serde_json::json!({
                "pre_checkout_query_id": "query-id",
                "ok": true
            }),
        ),
        AnswerPreCheckoutQuery::ok("query-id"),
    );
    assert_payload_eq(
        Payload::json(
            "answerPreCheckoutQuery",
            serde_json::json!({
                "pre_checkout_query_id": "query-id",
                "ok": false,
                "error_message": "msg"
            }),
        ),
        AnswerPreCheckoutQuery::error("query-id", "msg"),
    );
}

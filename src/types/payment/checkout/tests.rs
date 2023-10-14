use crate::{
    tests::{assert_json_eq, assert_request_eq, ExpectedRequest},
    types::{AnswerPreCheckoutQuery, OrderInfo, PreCheckoutQuery, User},
};

#[test]
fn pre_checkout_query() {
    assert_json_eq(
        PreCheckoutQuery {
            id: String::from("query id"),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("test"),
                last_name: None,
                username: None,
                language_code: None,
            },
            currency: String::from("GEL"),
            total_amount: 100,
            invoice_payload: String::from("invoice payload"),
            shipping_option_id: Some(String::from("option id")),
            order_info: Some(OrderInfo {
                name: None,
                phone_number: None,
                email: None,
                shipping_address: None,
            }),
        },
        serde_json::json!({
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
        }),
    );
    assert_json_eq(
        PreCheckoutQuery {
            id: String::from("query id"),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("test"),
                last_name: None,
                username: None,
                language_code: None,
            },
            currency: String::from("GEL"),
            total_amount: 100,
            invoice_payload: String::from("invoice payload"),
            shipping_option_id: None,
            order_info: None,
        },
        serde_json::json!({
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "test",
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
    assert_request_eq(
        ExpectedRequest::post_json(
            "answerPreCheckoutQuery",
            serde_json::json!({
                "pre_checkout_query_id": "query-id",
                "ok": true
            }),
        ),
        AnswerPreCheckoutQuery::ok("query-id"),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
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

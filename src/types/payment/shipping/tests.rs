use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, AnswerShippingQuery, ShippingAddress, ShippingOption, ShippingQuery, User},
};

#[test]
fn shipping_address() {
    assert_json_eq(
        ShippingAddress::new("Gudermes", "RU", "366200", "Chechen Republic", "Nuradilov st., 12", ""),
        serde_json::json!({
            "country_code": "RU",
            "state": "Chechen Republic",
            "city": "Gudermes",
            "street_line1": "Nuradilov st., 12",
            "street_line2": "",
            "post_code": "366200"
        }),
    );
}

#[test]
fn shipping_option() {
    let expected_struct = ShippingOption::new("id", "title", vec![]);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": "id",
            "title": "title",
            "prices": []
        }),
    );

    assert_eq!(expected_struct.id(), "id");
    assert_eq!(expected_struct.title(), "title");
    assert_eq!(expected_struct.prices().len(), 0);
}

#[test]
fn shipping_query() {
    assert_json_eq(
        ShippingQuery::new(
            "query-id",
            User::new(1, "User", false),
            "payload",
            ShippingAddress::new("Gudermes", "RU", "366200", "Chechen Republic", "Nuradilov st., 12", ""),
        ),
        serde_json::json!({
            "id": "query-id",
            "from": {
                "id": 1,
                "first_name": "User",
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
        }),
    );
}

#[test]
fn answer_shipping_query() {
    assert_payload_eq(
        Payload::json(
            "answerShippingQuery",
            serde_json::json!({
                "shipping_query_id": "id",
                "ok": true,
                "shipping_options": []
            }),
        ),
        AnswerShippingQuery::ok("id", vec![]),
    );
    assert_payload_eq(
        Payload::json(
            "answerShippingQuery",
            serde_json::json!({
                "shipping_query_id": "id",
                "ok": false,
                "error_message": "msg"
            }),
        ),
        AnswerShippingQuery::error("id", "msg"),
    );
}

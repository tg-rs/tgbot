use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, AnswerShippingQuery, ShippingAddress, ShippingOption, ShippingQuery, User},
};

#[test]
fn shipping_address() {
    assert_json_eq(
        ShippingAddress {
            country_code: String::from("RU"),
            state: String::from("Chechen Republic"),
            city: String::from("Gudermes"),
            street_line1: String::from("Nuradilov st., 12"),
            street_line2: String::from(""),
            post_code: String::from("366200"),
        },
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
        ShippingQuery {
            id: String::from("query-id"),
            from: User {
                id: 1,
                is_bot: false,
                first_name: String::from("test"),
                last_name: None,
                username: None,
                language_code: None,
            },
            invoice_payload: String::from("payload"),
            shipping_address: ShippingAddress {
                country_code: String::from("RU"),
                state: String::from("Chechen Republic"),
                city: String::from("Gudermes"),
                street_line1: String::from("Nuradilov st., 12"),
                street_line2: String::from(""),
                post_code: String::from("366200"),
            },
        },
        serde_json::json!({
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

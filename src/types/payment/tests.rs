use crate::types::{tests::assert_json_eq, OrderInfo, SuccessfulPayment};

#[test]
fn successful_payment() {
    assert_json_eq(
        SuccessfulPayment {
            currency: String::from("RUB"),
            total_amount: 145,
            invoice_payload: String::from("invoice payload"),
            shipping_option_id: Some(String::from("option id")),
            order_info: Some(OrderInfo {
                name: None,
                phone_number: None,
                email: None,
                shipping_address: None,
            }),
            telegram_payment_charge_id: String::from("tg-charge-id"),
            provider_payment_charge_id: String::from("provider-charge-id"),
        },
        serde_json::json!({
            "currency": "RUB",
            "total_amount": 145,
            "invoice_payload": "invoice payload",
            "shipping_option_id": "option id",
            "order_info": {},
            "telegram_payment_charge_id": "tg-charge-id",
            "provider_payment_charge_id": "provider-charge-id"
        }),
    );
    assert_json_eq(
        SuccessfulPayment {
            currency: String::from("RUB"),
            total_amount: 145,
            invoice_payload: String::from("invoice payload"),
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id: String::from("tg-charge-id"),
            provider_payment_charge_id: String::from("provider-charge-id"),
        },
        serde_json::json!({
            "currency": "RUB",
            "total_amount": 145,
            "invoice_payload": "invoice payload",
            "telegram_payment_charge_id": "tg-charge-id",
            "provider_payment_charge_id": "provider-charge-id"
        }),
    );
}

use crate::types::{OrderInfo, SuccessfulPayment, tests::assert_json_eq};

#[test]
fn successful_payment() {
    let expected_struct = SuccessfulPayment::new("RUB", "invoice payload", "provider-charge-id", "tg-charge-id", 145);
    assert_json_eq(
        expected_struct
            .clone()
            .with_is_first_recurring(true)
            .with_is_recurring(true)
            .with_order_info(OrderInfo::default())
            .with_shipping_option_id("option id")
            .with_subscription_expiration_date(0),
        serde_json::json!({
            "currency": "RUB",
            "total_amount": 145,
            "invoice_payload": "invoice payload",
            "shipping_option_id": "option id",
            "order_info": {},
            "telegram_payment_charge_id": "tg-charge-id",
            "provider_payment_charge_id": "provider-charge-id",
            "is_first_recurring": true,
            "is_recurring": true,
            "subscription_expiration_date": 0,
        }),
    );
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "currency": "RUB",
            "total_amount": 145,
            "invoice_payload": "invoice payload",
            "telegram_payment_charge_id": "tg-charge-id",
            "provider_payment_charge_id": "provider-charge-id"
        }),
    );
}

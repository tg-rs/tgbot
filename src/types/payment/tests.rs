use crate::types::{tests::assert_json_eq, OrderInfo, SuccessfulPayment};

#[test]
fn successful_payment() {
    let expected_struct = SuccessfulPayment::new("RUB", "invoice payload", "provider-charge-id", "tg-charge-id", 145);
    assert_json_eq(
        expected_struct
            .clone()
            .with_order_info(OrderInfo::default())
            .with_shipping_option_id("option id"),
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

use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, RefundStarPayment, RefundedPayment},
};

#[test]
fn refunded_payment() {
    let expected_struct = RefundedPayment::new("RUB", "payload", "charge-id", 100);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "currency": "RUB",
            "invoice_payload": "payload",
            "telegram_payment_charge_id": "charge-id",
            "total_amount": 100,
        }),
    );
    assert_json_eq(
        expected_struct.with_provider_payment_charge_id("provider-charge-id"),
        serde_json::json!({
            "currency": "RUB",
            "invoice_payload": "payload",
            "telegram_payment_charge_id": "charge-id",
            "total_amount": 100,
            "provider_payment_charge_id": "provider-charge-id",
        }),
    );
}

#[test]
fn refund_star_payment() {
    assert_payload_eq(
        Payload::json(
            "refundStarPayment",
            serde_json::json!({"user_id": 1, "telegram_payment_charge_id": "test"}),
        ),
        RefundStarPayment::new(1, "test"),
    );
}

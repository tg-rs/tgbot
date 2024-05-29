use crate::{
    api::{assert_payload_eq, Payload},
    types::RefundStarPayment,
};

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

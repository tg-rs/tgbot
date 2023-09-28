use crate::types::SuccessfulPayment;

#[test]
fn deserialize_full() {
    let data: SuccessfulPayment = serde_json::from_value(serde_json::json!({
        "currency": "RUB",
        "total_amount": 145,
        "invoice_payload": "invoice payload",
        "shipping_option_id": "option id",
        "order_info": {},
        "telegram_payment_charge_id": "tg-charge-id",
        "provider_payment_charge_id": "provider-charge-id"
    }))
    .unwrap();
    assert_eq!(data.currency, "RUB");
    assert_eq!(data.total_amount, 145);
    assert_eq!(data.invoice_payload, "invoice payload");
    assert_eq!(data.shipping_option_id.unwrap(), "option id");
    assert!(data.order_info.unwrap().name.is_none());
    assert_eq!(data.telegram_payment_charge_id, "tg-charge-id");
    assert_eq!(data.provider_payment_charge_id, "provider-charge-id");
}

#[test]
fn deserialize_partial() {
    let data: SuccessfulPayment = serde_json::from_value(serde_json::json!({
        "currency": "RUB",
        "total_amount": 145,
        "invoice_payload": "invoice payload",
        "telegram_payment_charge_id": "tg-charge-id",
        "provider_payment_charge_id": "provider-charge-id"
    }))
    .unwrap();
    assert_eq!(data.currency, "RUB");
    assert_eq!(data.total_amount, 145);
    assert_eq!(data.invoice_payload, "invoice payload");
    assert!(data.shipping_option_id.is_none());
    assert!(data.order_info.is_none());
    assert_eq!(data.telegram_payment_charge_id, "tg-charge-id");
    assert_eq!(data.provider_payment_charge_id, "provider-charge-id");
}

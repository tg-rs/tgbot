use crate::types::OrderInfo;

#[test]
fn order_info_deserialize_full() {
    let data: OrderInfo = serde_json::from_value(serde_json::json!({
        "name": "magticom",
        "phone_number": "+995 32 217 00 00",
        "email": "office@magticom.ge",
        "shipping_address": {
            "country_code": "GE",
            "state": "",
            "city": "Tbilisi",
            "street_line1": "7 A. Politkovskaya st.",
            "street_line2": "",
            "post_code": ""
        }
    }))
    .unwrap();
    assert_eq!(data.name.unwrap(), "magticom");
    assert_eq!(data.phone_number.unwrap(), "+995 32 217 00 00");
    assert_eq!(data.email.unwrap(), "office@magticom.ge");
    let addr = data.shipping_address.unwrap();
    assert_eq!(addr.country_code, "GE");
    assert_eq!(addr.state, "");
    assert_eq!(addr.city, "Tbilisi");
    assert_eq!(addr.street_line1, "7 A. Politkovskaya st.");
    assert_eq!(addr.street_line2, "");
    assert_eq!(addr.post_code, "");
}

#[test]
fn order_info_deserialize_partial() {
    let data: OrderInfo = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(data.name.is_none());
    assert!(data.phone_number.is_none());
    assert!(data.email.is_none());
    assert!(data.shipping_address.is_none());
}

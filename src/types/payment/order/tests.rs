use crate::types::{OrderInfo, ShippingAddress, tests::assert_json_eq};

#[test]
fn order_info() {
    assert_json_eq(
        OrderInfo::default()
            .with_email("gela@kobakhidze.ge")
            .with_name("Gela")
            .with_phone_number("+995 32 217 00 00")
            .with_shipping_address(ShippingAddress::new(
                "Tbilisi",
                "GE",
                "",
                "",
                "7 A. Politkovskaya st.",
                "",
            )),
        serde_json::json!({
            "name": "Gela",
            "phone_number": "+995 32 217 00 00",
            "email": "gela@kobakhidze.ge",
            "shipping_address": {
                "country_code": "GE",
                "state": "",
                "city": "Tbilisi",
                "street_line1": "7 A. Politkovskaya st.",
                "street_line2": "",
                "post_code": ""
            }
        }),
    );
    assert_json_eq(OrderInfo::default(), serde_json::json!({}));
}

use crate::{
    tests::assert_json_eq,
    types::{OrderInfo, ShippingAddress},
};

#[test]
fn order_info() {
    assert_json_eq(
        OrderInfo {
            name: Some(String::from("Gela")),
            phone_number: Some(String::from("+995 32 217 00 00")),
            email: Some(String::from("gela@kobakhidze.ge")),
            shipping_address: Some(ShippingAddress {
                country_code: String::from("GE"),
                state: String::from(""),
                city: String::from("Tbilisi"),
                street_line1: String::from("7 A. Politkovskaya st."),
                street_line2: String::from(""),
                post_code: String::from(""),
            }),
        },
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
    assert_json_eq(
        OrderInfo {
            name: None,
            phone_number: None,
            email: None,
            shipping_address: None,
        },
        serde_json::json!({}),
    );
}

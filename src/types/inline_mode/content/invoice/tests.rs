use serde::Serialize;

use crate::types::{tests::assert_json_eq, InputMessageContent, InputMessageContentInvoice, LabeledPrice};

#[derive(Serialize)]
struct InvoiceProviderData {
    key: String,
}

#[test]
fn input_message_content_invoice() {
    assert_json_eq(
        InputMessageContent::from(
            InputMessageContentInvoice::new(
                "title",
                "description",
                "payload",
                "provider-token",
                "RUB",
                [LabeledPrice::new("item", 100)],
            )
            .max_tip_amount(1)
            .suggested_tip_amounts([2])
            .provider_data(&InvoiceProviderData {
                key: String::from("value"),
            })
            .unwrap()
            .photo_url("https://google.com/favicon.ico")
            .photo_size(100)
            .photo_width(24)
            .photo_height(24)
            .need_name(true)
            .need_email(false)
            .need_phone_number(true)
            .need_shipping_address(false)
            .send_phone_number_to_provider(true)
            .send_email_to_provider(false)
            .flexible(true),
        ),
        serde_json::json!({
            "title": "title",
            "description": "description",
            "payload": "payload",
            "provider_token": "provider-token",
            "currency": "RUB",
            "prices": [{"label": "item", "amount": 100}],
            "max_tip_amount": 1,
            "suggested_tip_amounts": [2],
            "provider_data": "{\"key\":\"value\"}",
            "photo_url": "https://google.com/favicon.ico",
            "photo_size": 100,
            "photo_width": 24,
            "photo_height": 24,
            "need_name": true,
            "need_email": false,
            "need_phone_number": true,
            "need_shipping_address": false,
            "send_phone_number_to_provider": true,
            "send_email_to_provider": false,
            "is_flexible": true
        }),
    );
    assert_json_eq(
        InputMessageContent::from(InputMessageContentInvoice::new(
            "title",
            "description",
            "payload",
            "provider-token",
            "RUB",
            [LabeledPrice::new("item", 100)],
        )),
        serde_json::json!({
            "title": "title",
            "description": "description",
            "payload": "payload",
            "provider_token": "provider-token",
            "currency": "RUB",
            "prices": [{"label": "item", "amount": 100}]
        }),
    );
}

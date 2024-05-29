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
                "RUB",
                "description",
                "payload",
                [LabeledPrice::new(100, "item")],
                "title",
            )
            .with_is_flexible(true)
            .with_need_email(false)
            .with_need_name(true)
            .with_need_phone_number(true)
            .with_need_shipping_address(false)
            .with_provider_data(&InvoiceProviderData {
                key: String::from("value"),
            })
            .unwrap()
            .with_provider_token("provider-token")
            .with_photo_height(24)
            .with_photo_size(100)
            .with_photo_width(24)
            .with_photo_url("https://google.com/favicon.ico")
            .with_max_tip_amount(1)
            .with_send_email_to_provider(false)
            .with_send_phone_number_to_provider(true)
            .with_suggested_tip_amounts([2]),
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
            "RUB",
            "description",
            "payload",
            [LabeledPrice::new(100, "item")],
            "title",
        )),
        serde_json::json!({
            "title": "title",
            "description": "description",
            "payload": "payload",
            "currency": "RUB",
            "prices": [{"label": "item", "amount": 100}]
        }),
    );
}

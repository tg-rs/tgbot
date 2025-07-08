use serde::Serialize;

use crate::{
    api::{Payload, assert_payload_eq},
    types::*,
};

#[test]
fn pre_checkout_query() {
    let expected_struct = PreCheckoutQuery::new("GEL", User::new(1, "User", false), "query id", "invoice payload", 100);
    insta::assert_json_snapshot!(
        expected_struct
            .clone()
            .with_shipping_option_id("option id")
            .with_order_info(OrderInfo::default())
    );
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn answer_pre_checkout_query() {
    assert_payload_eq(
        Payload::json(
            "answerPreCheckoutQuery",
            serde_json::json!({
                "pre_checkout_query_id": "query-id",
                "ok": true
            }),
        ),
        AnswerPreCheckoutQuery::ok("query-id"),
    );
    assert_payload_eq(
        Payload::json(
            "answerPreCheckoutQuery",
            serde_json::json!({
                "pre_checkout_query_id": "query-id",
                "ok": false,
                "error_message": "msg"
            }),
        ),
        AnswerPreCheckoutQuery::error("query-id", "msg"),
    );
}

#[test]
fn invoice() {
    insta::assert_json_snapshot!(Invoice::new(
        "RUB",
        "invoice description",
        "invoice start parameter",
        "invoice title",
        100,
    ));
}

#[test]
fn labeled_price() {
    insta::assert_json_snapshot!(LabeledPrice::new(145, "price label"));
}

#[derive(Serialize)]
struct ProviderData {
    key: String,
}

#[test]
fn create_invoice_link() {
    let method = CreateInvoiceLink::new(
        "product-name",
        "product-description",
        "payload",
        "GEL",
        [LabeledPrice::new(100, "price-label")],
    );
    assert_payload_eq(
        Payload::json(
            "createInvoiceLink",
            serde_json::json!({
                "title": "product-name",
                "description": "product-description",
                "payload": "payload",
                "currency": "GEL",
                "prices": [
                    {
                        "label": "price-label",
                        "amount": 100
                    }
                ]
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "createInvoiceLink",
            serde_json::json!({
                "title": "product-name",
                "description": "product-description",
                "payload": "payload",
                "currency": "GEL",
                "prices": [
                    {
                        "label": "price-label",
                        "amount": 100
                    }
                ],
                "provider_token": "provider-token",
                "max_tip_amount": 100,
                "business_connection_id": "id",
                "subscription_period": 1,
            }),
        ),
        method
            .with_business_connection_id("id")
            .with_parameters(
                InvoiceParameters::default()
                    .with_max_tip_amount(100)
                    .with_provider_token("provider-token"),
            )
            .with_subscription_period(1),
    );
}

#[test]
fn send_invoice() {
    assert_payload_eq(
        Payload::json(
            "sendInvoice",
            serde_json::json!({
                "chat_id": "@username",
                "title": "title",
                "description": "description",
                "payload": "payload",
                "currency": "RUB",
                "prices": [
                    {
                        "label": "item",
                        "amount": 100
                    }
                ],
            }),
        ),
        SendInvoice::new(
            "@username",
            "title",
            "description",
            "payload",
            "RUB",
            vec![LabeledPrice::new(100, "item")],
        ),
    );
    assert_payload_eq(
        Payload::json(
            "sendInvoice",
            serde_json::json!({
                "chat_id": 1,
                "title": "title",
                "description": "description",
                "payload": "payload",
                "provider_token": "token",
                "start_parameter": "param",
                "currency": "RUB",
                "prices": [],
                "max_tip_amount": 100,
                "suggested_tip_amounts": [10, 50, 100],
                "provider_data": "{\"key\":\"value\"}",
                "photo_url": "url",
                "photo_size": 100,
                "photo_width": 200,
                "photo_height": 300,
                "need_name": true,
                "need_phone_number": true,
                "need_email": true,
                "need_shipping_address": true,
                "send_phone_number_to_provider": true,
                "send_email_to_provider": true,
                "is_flexible": true,
                "allow_paid_broadcast": true,
                "disable_notification": true,
                "message_effect_id": "effect-id",
                "message_thread_id": 1,
                "protect_content": true,
                "reply_markup": {
                    "inline_keyboard": [[
                        {"text": "text", "url": "url"}
                    ]]
                },
                "reply_parameters": {
                    "message_id": 1
                }
            }),
        ),
        SendInvoice::new(1, "title", "description", "payload", "RUB", vec![])
            .with_allow_paid_broadcast(true)
            .with_disable_notification(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_parameters(
                InvoiceParameters::default()
                    .with_max_tip_amount(100)
                    .with_need_email(true)
                    .with_need_name(true)
                    .with_need_phone_number(true)
                    .with_need_shipping_address(true)
                    .with_photo_height(300)
                    .with_photo_size(100)
                    .with_photo_url("url")
                    .with_photo_width(200)
                    .with_provider_data(&ProviderData {
                        key: String::from("value"),
                    })
                    .unwrap()
                    .with_provider_token("token")
                    .with_send_email_to_provider(true)
                    .with_send_phone_number_to_provider(true)
                    .with_suggested_tip_amounts(vec![10, 50, 100])
                    .with_flexible(true),
            )
            .with_protect_content(true)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_reply_parameters(ReplyParameters::new(1))
            .with_start_parameter("param"),
    );
}

#[test]
fn order_info() {
    insta::assert_json_snapshot!(
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
            ))
    );
    insta::assert_json_snapshot!(OrderInfo::default());
}

#[test]
fn refunded_payment() {
    let expected_struct = RefundedPayment::new("RUB", "payload", "charge-id", 100);
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(expected_struct.with_provider_payment_charge_id("provider-charge-id"));
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

#[test]
fn shipping_address() {
    insta::assert_json_snapshot!(ShippingAddress::new(
        "Gudermes",
        "RU",
        "366200",
        "Chechen Republic",
        "Nuradilov st., 12",
        ""
    ));
}

#[test]
fn shipping_option() {
    let expected_struct = ShippingOption::new("id", "title", vec![]);
    insta::assert_json_snapshot!(expected_struct.clone());

    assert_eq!(expected_struct.id(), "id");
    assert_eq!(expected_struct.title(), "title");
    assert_eq!(expected_struct.prices().len(), 0);
}

#[test]
fn shipping_query() {
    insta::assert_json_snapshot!(ShippingQuery::new(
        "query-id",
        User::new(1, "User", false),
        "payload",
        ShippingAddress::new("Gudermes", "RU", "366200", "Chechen Republic", "Nuradilov st., 12", ""),
    ));
}

#[test]
fn answer_shipping_query() {
    assert_payload_eq(
        Payload::json(
            "answerShippingQuery",
            serde_json::json!({
                "shipping_query_id": "id",
                "ok": true,
                "shipping_options": []
            }),
        ),
        AnswerShippingQuery::ok("id", vec![]),
    );
    assert_payload_eq(
        Payload::json(
            "answerShippingQuery",
            serde_json::json!({
                "shipping_query_id": "id",
                "ok": false,
                "error_message": "msg"
            }),
        ),
        AnswerShippingQuery::error("id", "msg"),
    );
}

#[test]
fn edit_user_star_subscription() {
    let method = EditUserStarSubscription::new(1, "id", false);
    assert_payload_eq(
        Payload::json(
            "editUserStarSubscription",
            serde_json::json!({
                "user_id": 1,
                "telegram_payment_charge_id": "id",
                "is_canceled": false,
            }),
        ),
        method,
    );
}

#[test]
fn get_star_transactions() {
    let method = GetStarTransactions::default();
    assert_payload_eq(Payload::json("getStarTransactions", serde_json::json!({})), method);
    assert_payload_eq(
        Payload::json(
            "getStarTransactions",
            serde_json::json!({
                "offset": 0,
                "limit": 5,
            }),
        ),
        method.with_offset(0).with_limit(5),
    );
}

#[test]
fn revenue_withdrawal_state() {
    insta::assert_json_snapshot!(RevenueWithdrawalState::Failed);
    insta::assert_json_snapshot!(RevenueWithdrawalState::Pending);
    insta::assert_json_snapshot!(RevenueWithdrawalState::Succeeded {
        date: 0,
        url: String::from("https://google.com"),
    });
}

#[test]
fn star_transaction() {
    let mut expected_struct = StarTransaction::new(10, 0, "tx-id");
    insta::assert_json_snapshot!(expected_struct.clone());
    expected_struct = expected_struct
        .with_nanostar_amount(233)
        .with_source(TransactionPartner::Other)
        .with_receiver(TransactionPartner::Other);
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn star_transactions() {
    insta::assert_json_snapshot!(StarTransactions::from(vec![StarTransaction::new(10, 0, "tx-id")]));
}

#[test]
fn transaction_partner() {
    insta::assert_json_snapshot!(TransactionPartner::AffiliateProgram(
        TransactionPartnerAffiliateProgramParameters::new(1)
    ));
    insta::assert_json_snapshot!(TransactionPartner::AffiliateProgram(
        TransactionPartnerAffiliateProgramParameters::new(1).with_sponsor_user(User::new(1, "John", true)),
    ));
    insta::assert_json_snapshot!(TransactionPartner::Chat(TransactionPartnerChatParameters::new(
        PrivateChat::new(1, "test")
    )));
    insta::assert_json_snapshot!(TransactionPartner::Chat(
        TransactionPartnerChatParameters::new(PrivateChat::new(1, "test")).with_gift(Gift::new(
            "gift-id",
            Sticker::new("file-id", "file-unique-id", StickerType::Mask, 512, 512),
            2,
        )),
    ));
    insta::assert_json_snapshot!(TransactionPartner::Fragment(None));
    insta::assert_json_snapshot!(TransactionPartner::Fragment(Some(RevenueWithdrawalState::Pending)));
    insta::assert_json_snapshot!(TransactionPartner::Other);
    insta::assert_json_snapshot!(TransactionPartner::TelegramAds);
    insta::assert_json_snapshot!(TransactionPartner::TelegramApi { request_count: 1 });
    insta::assert_json_snapshot!(TransactionPartner::User(TransactionPartnerUserParameters::new(
        TransactionPartnerUserType::PaidMediaPayment,
        User::new(1, "John", false),
    )));
    insta::assert_json_snapshot!(
        TransactionPartner::User(
            TransactionPartnerUserParameters::new(
                TransactionPartnerUserType::PremiumPurchase,
                User::new(1, "John", false),
            )
            .with_affiliate(AffiliateInfo::new(1, 1))
            .with_gift("test-gift")
            .with_invoice_payload(String::from("invoice-payload"))
            .with_paid_media([PaidMedia::Preview(PaidMediaPreview::default().with_duration(1))])
            .with_paid_media_payload(String::from("media-payload"))
            .with_premium_subscription_duration(5)
            .with_subscription_period(1),
        )
    );
}

#[test]
fn affiliate_info() {
    let expected_struct = AffiliateInfo::new(1, 1);
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_affiliate_chat(ChannelChat::new(1, "test"))
            .with_affiliate_user(User::new(1, "John", true))
            .with_nanostar_amount(1)
    );
}

#[test]
fn successful_payment() {
    let expected_struct = SuccessfulPayment::new("RUB", "invoice payload", "provider-charge-id", "tg-charge-id", 145);
    insta::assert_json_snapshot!(
        expected_struct
            .clone()
            .with_is_first_recurring(true)
            .with_is_recurring(true)
            .with_order_info(OrderInfo::default())
            .with_shipping_option_id("option id")
            .with_subscription_expiration_date(0)
    );
    insta::assert_json_snapshot!(expected_struct);
}

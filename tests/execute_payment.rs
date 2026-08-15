#![allow(missing_docs)]
use serde::Serialize;
use tgbot::types::*;

mod cx;

use cx::Cx;

#[derive(Serialize)]
struct ProviderData {
    key: String,
}

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-invoice-base",
            SendInvoice::new(
                "@username",
                "title",
                "description",
                "payload",
                "RUB",
                vec![LabeledPrice::new(100, "item")],
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Invoice(data) = x.data {
                    assert_eq!(data.title, "title");
                    assert_eq!(data.description, "description");
                    assert_eq!(data.start_parameter, "unique-parameter");
                    assert_eq!(data.currency, "RUB");
                    assert_eq!(data.total_amount, 100);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-invoice-full",
            SendInvoice::new(1, "title", "description", "payload", "RUB", vec![])
                .with_allow_paid_broadcast(true)
                .with_direct_messages_topic_id(1)
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
                .with_suggested_post_parameters(SuggestedPostParameters::default())
                .with_start_parameter("param"),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Invoice(data) = x.data {
                    assert_eq!(data.title, "title");
                    assert_eq!(data.description, "description");
                    assert_eq!(data.start_parameter, "unique-parameter");
                    assert_eq!(data.currency, "RUB");
                    assert_eq!(data.total_amount, 100);
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "answer-pre-checkout-query-ok",
            AnswerPreCheckoutQuery::ok("query-id"),
            |x| assert!(x),
        ),
        (
            "answer-pre-checkout-query-error",
            AnswerPreCheckoutQuery::error("query-id", "msg"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "create-invoice-link-base",
            CreateInvoiceLink::new(
                "product-name",
                "product-description",
                "payload",
                "GEL",
                [LabeledPrice::new(100, "price-label")],
            ),
            |x| assert_eq!(x, "test"),
        ),
        (
            "create-invoice-link-all",
            {
                CreateInvoiceLink::new(
                    "product-name",
                    "product-description",
                    "payload",
                    "GEL",
                    [LabeledPrice::new(100, "price-label")],
                )
                .with_business_connection_id("id")
                .with_parameters(
                    InvoiceParameters::default()
                        .with_max_tip_amount(100)
                        .with_provider_token("provider-token"),
                )
                .with_subscription_period(1)
            },
            |x| assert_eq!(x, "test"),
        ),
    ])
    .await;

    cx.execute_batch([("refund-star-payment", RefundStarPayment::new(1, "test"), |x| assert!(x))])
        .await;

    cx.execute_batch([
        ("answer-shipping-query-ok", AnswerShippingQuery::ok("id", vec![]), |x| {
            assert!(x)
        }),
    ])
    .await;

    cx.execute_batch([(
        "answer-shipping-query-err",
        { AnswerShippingQuery::error("id", "msg") },
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([(
        "edit-user-star-subscription",
        { EditUserStarSubscription::new(1, "id", false) },
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([
        ("get-star-transactions-base", GetStarTransactions::default(), |x| {
            assert_eq!(x.transactions.len(), 6);
            let tx = &x.transactions[0];
            assert_eq!(tx.id, "tx-1");
            assert_eq!(tx.amount, 200);
            assert_eq!(tx.date, 0);
            assert!(tx.nanostar_amount.is_none());
            assert!(tx.source.is_none());
            assert!(tx.receiver.is_none());

            let tx = &x.transactions[1];
            assert_eq!(tx.id, "tx-2");
            assert_eq!(tx.amount, 200);
            assert_eq!(tx.date, 0);
            assert_eq!(tx.nanostar_amount.unwrap(), 100);
            let TransactionPartner::User(source) = tx.source.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
            assert_eq!(source.user.id, 1);
            let TransactionPartner::Chat(receiver) = tx.receiver.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
            assert_eq!(receiver.chat.get_id(), 1);

            let tx = &x.transactions[2];
            assert_eq!(tx.id, "tx-3");
            assert_eq!(tx.amount, 200);
            assert_eq!(tx.date, 0);
            let TransactionPartner::AffiliateProgram(source) = tx.source.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
            assert_eq!(source.sponsor_user.as_ref().unwrap().id, 1);
            let TransactionPartner::Fragment(receiver) = tx.receiver.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
            assert!(matches!(receiver.as_ref().unwrap(), RevenueWithdrawalState::Pending));

            let tx = &x.transactions[3];
            assert_eq!(tx.id, "tx-4");
            assert_eq!(tx.amount, 200);
            assert_eq!(tx.date, 0);
            let TransactionPartner::TelegramAds = tx.source.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
            let TransactionPartner::TelegramApi { request_count } = tx.receiver.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
            assert_eq!(*request_count, 10);

            let tx = &x.transactions[4];
            assert_eq!(tx.id, "tx-5");
            assert_eq!(tx.amount, 200);
            assert_eq!(tx.date, 0);
            let TransactionPartner::Fragment(source) = tx.source.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
            assert!(matches!(
                source.as_ref().unwrap(),
                RevenueWithdrawalState::Succeeded { .. }
            ));
            let TransactionPartner::Other = tx.receiver.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };

            let tx = &x.transactions[5];
            assert_eq!(tx.id, "tx-6");
            assert_eq!(tx.amount, 200);
            assert_eq!(tx.date, 0);
            let TransactionPartner::Fragment(source) = tx.source.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
            assert!(matches!(source.as_ref().unwrap(), RevenueWithdrawalState::Failed));
            let TransactionPartner::Other = tx.receiver.as_ref().unwrap() else {
                panic!("Unexpected partner")
            };
        }),
        (
            "get-star-transactions-all",
            { GetStarTransactions::default().with_offset(0).with_limit(5) },
            |x| {
                assert_eq!(x.transactions.len(), 6);
            },
        ),
    ])
    .await;
}

use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        AffiliateInfo,
        ChannelChat,
        GetStarTransactions,
        Gift,
        PaidMedia,
        PaidMediaPreview,
        PrivateChat,
        RevenueWithdrawalState,
        StarTransaction,
        StarTransactions,
        Sticker,
        StickerType,
        TransactionPartner,
        TransactionPartnerAffiliateProgramParameters,
        TransactionPartnerChatParameters,
        TransactionPartnerUserParameters,
        User,
    },
};

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
    assert_json_eq(RevenueWithdrawalState::Failed, serde_json::json!({"type": "failed"}));
    assert_json_eq(RevenueWithdrawalState::Pending, serde_json::json!({"type": "pending"}));
    assert_json_eq(
        RevenueWithdrawalState::Succeeded {
            date: 0,
            url: String::from("https://google.com"),
        },
        serde_json::json!({
            "type": "succeeded",
            "date": 0,
            "url": "https://google.com"
        }),
    );
}

#[test]
fn star_transaction() {
    let mut expected_struct = StarTransaction::new(10, 0, "tx-id");
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "amount": 10,
            "date": 0,
            "id": "tx-id",
        }),
    );

    expected_struct = expected_struct
        .with_nanostar_amount(233)
        .with_source(TransactionPartner::Other)
        .with_receiver(TransactionPartner::Other);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "amount": 10,
            "date": 0,
            "id": "tx-id",
            "nanostar_amount": 233,
            "source": {
                "type": "other"
            },
            "receiver": {
                "type": "other"
            },
        }),
    );
}

#[test]
fn star_transactions() {
    assert_json_eq(
        StarTransactions::from(vec![StarTransaction::new(10, 0, "tx-id")]),
        serde_json::json!({
            "transactions": [
                {
                    "amount": 10,
                    "date": 0,
                    "id": "tx-id",
                }
            ]
        }),
    );
}

#[test]
fn transaction_partner() {
    assert_json_eq(
        TransactionPartner::AffiliateProgram(TransactionPartnerAffiliateProgramParameters::new(1)),
        serde_json::json!({
            "type": "affiliate_program",
            "commission_per_mille": 1,
        }),
    );
    assert_json_eq(
        TransactionPartner::AffiliateProgram(
            TransactionPartnerAffiliateProgramParameters::new(1).with_sponsor_user(User::new(1, "John", true)),
        ),
        serde_json::json!({
            "type": "affiliate_program",
            "commission_per_mille": 1,
            "sponsor_user": {
                "id": 1,
                "first_name": "John",
                "is_bot": true,
            }
        }),
    );
    assert_json_eq(
        TransactionPartner::Chat(TransactionPartnerChatParameters::new(PrivateChat::new(1, "test"))),
        serde_json::json!({"type": "chat", "chat": {"type": "private", "id": 1, "first_name": "test"}}),
    );
    assert_json_eq(
        TransactionPartner::Chat(
            TransactionPartnerChatParameters::new(PrivateChat::new(1, "test")).with_gift(Gift::new(
                "gift-id",
                Sticker::new("file-id", "file-unique-id", StickerType::Mask, 512, 512),
                2,
            )),
        ),
        serde_json::json!({
            "type": "chat",
            "chat": {"type": "private", "id": 1, "first_name": "test"},
            "gift": {
                "id": "gift-id",
                "sticker": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                    "type": "mask",
                    "width": 512,
                    "height": 512,
                    "is_animated": false,
                    "is_video": false,
                },
                "star_count": 2
            }
        }),
    );
    assert_json_eq(
        TransactionPartner::Fragment(None),
        serde_json::json!({"type": "fragment"}),
    );
    assert_json_eq(
        TransactionPartner::Fragment(Some(RevenueWithdrawalState::Pending)),
        serde_json::json!({
            "type": "fragment",
            "withdrawal_state": {
                "type": "pending"
            }
        }),
    );
    assert_json_eq(TransactionPartner::Other, serde_json::json!({"type": "other"}));
    assert_json_eq(
        TransactionPartner::TelegramAds,
        serde_json::json!({"type": "telegram_ads"}),
    );
    assert_json_eq(
        TransactionPartner::TelegramApi { request_count: 1 },
        serde_json::json!({"type": "telegram_api", "request_count": 1}),
    );
    assert_json_eq(
        TransactionPartner::User(TransactionPartnerUserParameters::new(User::new(1, "John", false))),
        serde_json::json!({
            "type": "user",
            "user" : {
                "id": 1,
                "first_name": "John",
                "is_bot": false
            }
        }),
    );
    assert_json_eq(
        TransactionPartner::User(
            TransactionPartnerUserParameters::new(User::new(1, "John", false))
                .with_affiliate(AffiliateInfo::new(1, 1))
                .with_gift("test-gift")
                .with_invoice_payload(String::from("invoice-payload"))
                .with_paid_media([PaidMedia::Preview(PaidMediaPreview::default().with_duration(1))])
                .with_paid_media_payload(String::from("media-payload"))
                .with_subscription_period(1),
        ),
        serde_json::json!({
            "type": "user",
            "user" : {
                "id": 1,
                "first_name": "John",
                "is_bot": false
            },
            "affiliate": {
                "amount": 1,
                "commission_per_mille": 1,
            },
            "gift": "test-gift",
            "invoice_payload": "invoice-payload",
            "paid_media": [
                {
                    "type": "preview",
                    "duration": 1,
                }
            ],
            "paid_media_payload": "media-payload",
            "subscription_period": 1,
        }),
    );
}

#[test]
fn affiliate_info() {
    let expected_struct = AffiliateInfo::new(1, 1);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "amount": 1,
            "commission_per_mille": 1,
        }),
    );
    assert_json_eq(
        expected_struct
            .with_affiliate_chat(ChannelChat::new(1, "test"))
            .with_affiliate_user(User::new(1, "John", true))
            .with_nanostar_amount(1),
        serde_json::json!({
            "amount": 1,
            "commission_per_mille": 1,
            "affiliate_chat": {
                "type": "channel",
                "id": 1,
                "title": "test",
            },
            "affiliate_user": {
                "id": 1,
                "first_name": "John",
                "is_bot": true,
            },
            "nanostar_amount": 1,
        }),
    );
}

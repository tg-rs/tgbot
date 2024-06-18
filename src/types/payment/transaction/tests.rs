use crate::types::{
    tests::assert_json_eq,
    RevenueWithdrawalState,
    StarTransaction,
    StarTransactions,
    TransactionPartner,
    User,
};

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
        .with_source(TransactionPartner::Other)
        .with_receiver(TransactionPartner::Other);
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "amount": 10,
            "date": 0,
            "id": "tx-id",
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
        TransactionPartner::User(User::new(1, "John", false)),
        serde_json::json!({
            "type": "user",
            "user" : {
                "id": 1,
                "first_name": "John",
                "is_bot": false
            }
        }),
    );
}

use crate::{
    api::{assert_payload_eq, Payload},
    types::EditUserStarSubscription,
};

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

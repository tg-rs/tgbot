use serde::Serialize;

use crate::{
    api::{Method, Payload},
    types::Integer,
};

/// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars.
#[derive(Clone, Debug, Serialize)]
pub struct EditUserStarSubscription {
    user_id: Integer,
    telegram_payment_charge_id: String,
    is_canceled: bool,
}

impl EditUserStarSubscription {
    /// Creates a new `EditUserStarSubscription`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Identifier of the user whose subscription will be edited.
    /// * `telegram_payment_charge_id` - Telegram payment identifier for the subscription.
    /// * `is_canceled` - Whether to cancel extension of the user subscription;
    ///   the subscription must be active up to the end of the current subscription period;
    ///   use false to allow the user to re-enable a subscription that was previously canceled by the bot.
    pub fn new<T>(user_id: Integer, telegram_payment_charge_id: T, is_canceled: bool) -> Self
    where
        T: Into<String>,
    {
        Self {
            user_id,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            is_canceled,
        }
    }
}

impl Method for EditUserStarSubscription {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("editUserStarSubscription", self)
    }
}

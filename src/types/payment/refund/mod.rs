use serde::Serialize;

use crate::{
    api::{Method, Payload},
    types::Integer,
};

#[cfg(test)]
mod tests;

/// Refunds a successful payment in Telegram Stars.
#[derive(Clone, Debug, Serialize)]
pub struct RefundStarPayment {
    user_id: Integer,
    telegram_payment_charge_id: String,
}

impl RefundStarPayment {
    /// Creates a new `RefundStarPayment`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Identifier of the user whose payment will be refunded.
    /// * `telegram_payment_charge_id` - Telegram payment identifier.
    pub fn new<T>(user_id: Integer, telegram_payment_charge_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            user_id,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
        }
    }
}

impl Method for RefundStarPayment {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("refundStarPayment", self)
    }
}

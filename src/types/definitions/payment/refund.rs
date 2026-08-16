use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload, PayloadError},
    types::Integer,
};

/// Contains basic information about a refunded payment.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 currency code, or “XTR” for payments in Telegram Stars. Currently, always “XTR”.
    pub currency: String,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// Telegram payment identifier.
    pub telegram_payment_charge_id: String,
    /// Total refunded price in the smallest units of the currency (integer, not float/double).
    ///
    /// For example, for a price of `US$ 1.45`, `total_amount = 145`.
    /// See the exp parameter in [currencies.json][1],
    /// it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    ///
    /// [1]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: Integer,
    /// Provider payment identifier.
    pub provider_payment_charge_id: Option<String>,
}

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

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("refundStarPayment", self)
    }
}

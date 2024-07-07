use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::Integer,
};

#[cfg(test)]
mod tests;

/// Contains basic information about a refunded payment.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}

impl RefundedPayment {
    /// Creates a new `RefundedPayment`.
    ///
    /// # Arguments
    ///
    /// * `currency` - Three-letter ISO 4217 currency code.
    /// * `invoice_payload` - Bot-specified invoice payload.
    /// * `telegram_payment_charge_id` - Telegram payment identifier.
    /// * `total_amount` - Total refunded price in the smallest units of the currency.
    pub fn new<A, B, C>(currency: A, invoice_payload: B, telegram_payment_charge_id: C, total_amount: Integer) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            currency: currency.into(),
            invoice_payload: invoice_payload.into(),
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            total_amount,
            provider_payment_charge_id: None,
        }
    }

    /// Sets a new provider payment identifier.
    ///
    /// # Arguments
    ///
    /// * `value` - Provider payment identifier.
    pub fn with_provider_payment_charge_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.provider_payment_charge_id = Some(value.into());
        self
    }
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

    fn into_payload(self) -> Payload {
        Payload::json("refundStarPayment", self)
    }
}

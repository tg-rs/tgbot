use serde::{Deserialize, Serialize};

use crate::types::Integer;

pub use self::{checkout::*, invoice::*, order::*, shipping::*};

#[cfg(test)]
mod tests;

mod checkout;
mod invoice;
mod order;
mod shipping;

/// Represents a successful payment
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Total price in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145.
    /// See the exp parameter in [currencies.json][1], it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies).
    ///
    /// [1]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: Integer,
    /// Order info provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    /// Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
}

impl SuccessfulPayment {
    /// Creates a new SuccessfulPayment
    ///
    /// # Arguments
    ///
    /// * currency - Three-letter ISO 4217 currency code
    /// * invoice_payload - Bot specified invoice payload
    /// * provider_payment_charge_id - Provider payment identifier
    /// * telegram_payment_charge_id - Telegram payment identifier
    /// * total_amount - Total price
    pub fn new<A, B, C, D>(
        currency: A,
        invoice_payload: B,
        provider_payment_charge_id: C,
        telegram_payment_charge_id: D,
        total_amount: Integer,
    ) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
    {
        Self {
            currency: currency.into(),
            invoice_payload: invoice_payload.into(),
            provider_payment_charge_id: provider_payment_charge_id.into(),
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            total_amount,
            order_info: None,
            shipping_option_id: None,
        }
    }

    /// Sets a new order info
    ///
    /// # Arguments
    ///
    /// * value - Order info
    pub fn with_order_info(mut self, value: OrderInfo) -> Self {
        self.order_info = Some(value);
        self
    }

    /// Sets a new shipping option ID
    ///
    /// # Arguments
    ///
    /// * value - Shipping option ID
    pub fn with_shipping_option_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.shipping_option_id = Some(value.into());
        self
    }
}

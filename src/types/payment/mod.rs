use serde::{Deserialize, Serialize};

pub use self::{checkout::*, invoice::*, order::*, refund::*, shipping::*, transaction::*};
use crate::types::Integer;

#[cfg(test)]
mod tests;

mod checkout;
mod invoice;
mod order;
mod refund;
mod shipping;
mod transaction;

/// Represents a successful payment.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code.
    pub currency: String,
    /// Bot specified invoice payload.
    pub invoice_payload: String,
    /// Provider payment identifier.
    pub provider_payment_charge_id: String,
    /// Telegram payment identifier.
    pub telegram_payment_charge_id: String,
    /// Total price in the smallest units of the currency (integer, not float/double).
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145.
    /// See the exp parameter in [currencies.json][1], it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies).
    ///
    /// [1]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: Integer,
    /// Whether the payment is the first payment for a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_recurring: Option<bool>,
    /// Whether the payment is a recurring payment for a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
    /// Order info provided by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    /// Identifier of the shipping option chosen by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Expiration date of the subscription, in Unix time; for recurring payments only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_expiration_date: Option<Integer>,
}

impl SuccessfulPayment {
    /// Creates a new `SuccessfulPayment`.
    ///
    /// # Arguments
    ///
    /// * `currency` - Three-letter ISO 4217 currency code.
    /// * `invoice_payload` - Bot specified invoice payload.
    /// * `provider_payment_charge_id` - Provider payment identifier.
    /// * `telegram_payment_charge_id` - Telegram payment identifier.
    /// * `total_amount` - Total price.
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
            is_first_recurring: None,
            is_recurring: None,
            total_amount,
            order_info: None,
            shipping_option_id: None,
            subscription_expiration_date: None,
        }
    }

    /// Sets a new value for an `is_first_recurring` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the payment is the first payment for a subscription.
    pub fn with_is_first_recurring(mut self, value: bool) -> Self {
        self.is_first_recurring = Some(value);
        self
    }

    /// Sets a new value for an `is_recurring` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the payment is a recurring payment for a subscription.
    pub fn with_is_recurring(mut self, value: bool) -> Self {
        self.is_recurring = Some(value);
        self
    }

    /// Sets a new order info.
    ///
    /// # Arguments
    ///
    /// * `value` - Order info.
    pub fn with_order_info(mut self, value: OrderInfo) -> Self {
        self.order_info = Some(value);
        self
    }

    /// Sets a new shipping option ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Shipping option ID.
    pub fn with_shipping_option_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.shipping_option_id = Some(value.into());
        self
    }

    /// Sets a new subscription expiration date
    ///
    /// # Arguments
    ///
    /// * `value` -  Expiration date of the subscription, in Unix time; for recurring payments only.
    pub fn with_subscription_expiration_date(mut self, value: Integer) -> Self {
        self.subscription_expiration_date = Some(value);
        self
    }
}

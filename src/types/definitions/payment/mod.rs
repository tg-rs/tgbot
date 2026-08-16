use serde::{Deserialize, Serialize};

pub use self::{checkout::*, invoice::*, order::*, refund::*, shipping::*, subscription::*, transaction::*};
use crate::types::Integer;

mod checkout;
mod invoice;
mod order;
mod refund;
mod shipping;
mod subscription;
mod transaction;

/// Represents a successful payment.
#[serde_with::skip_serializing_none]
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
    pub is_first_recurring: Option<bool>,
    /// Whether the payment is a recurring payment for a subscription.
    pub is_recurring: Option<bool>,
    /// Order info provided by the user.
    pub order_info: Option<OrderInfo>,
    /// Identifier of the shipping option chosen by the user.
    pub shipping_option_id: Option<String>,
    /// Expiration date of the subscription, in Unix time; for recurring payments only.
    pub subscription_expiration_date: Option<Integer>,
}

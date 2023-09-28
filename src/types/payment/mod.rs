use serde::Deserialize;

use crate::types::Integer;

pub use self::{checkout::*, invoice::*, order::*, shipping::*};

#[cfg(test)]
mod tests;

mod checkout;
mod invoice;
mod order;
mod shipping;

/// Basic information about a successful payment
#[derive(Clone, Debug, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145
    /// See the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies)
    pub total_amount: Integer,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}

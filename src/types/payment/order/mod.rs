use serde::Deserialize;

use crate::types::ShippingAddress;

#[cfg(test)]
mod tests;

/// Information about an order
#[derive(Clone, Debug, Deserialize)]
pub struct OrderInfo {
    /// User name
    pub name: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// User email
    pub email: Option<String>,
    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}

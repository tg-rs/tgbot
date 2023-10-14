use serde::{Deserialize, Serialize};

use crate::types::ShippingAddress;

#[cfg(test)]
mod tests;

/// Information about an order
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OrderInfo {
    /// User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

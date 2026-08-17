use serde::{Deserialize, Serialize};

use crate::types::ShippingAddress;

/// Represents an order.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OrderInfo {
    /// User's email.
    pub email: Option<String>,
    /// User's name.
    pub name: Option<String>,
    /// User's phone number.
    pub phone_number: Option<String>,
    /// User's shipping address.
    pub shipping_address: Option<ShippingAddress>,
}

use serde::{Deserialize, Serialize};

use crate::types::ShippingAddress;

#[cfg(test)]
mod tests;

/// Represents an order
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OrderInfo {
    /// User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

impl OrderInfo {
    /// Sets a new email
    ///
    /// # Arguments
    ///
    /// * value - Email
    pub fn with_email<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.email = Some(value.into());
        self
    }

    /// Sets a new user name
    ///
    /// # Arguments
    ///
    /// * value - Name
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }

    /// Sets a new phone number
    ///
    /// # Arguments
    ///
    /// * value - Phone number
    pub fn with_phone_number<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.phone_number = Some(value.into());
        self
    }

    /// Sets a new shipping address
    ///
    /// # Arguments
    ///
    /// * value - Shipping address
    pub fn with_shipping_address(mut self, value: ShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }
}

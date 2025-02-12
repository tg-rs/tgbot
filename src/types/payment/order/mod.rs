use serde::{Deserialize, Serialize};

use crate::types::ShippingAddress;

#[cfg(test)]
mod tests;

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

impl OrderInfo {
    /// Sets a new E-Mail
    ///
    /// # Arguments
    ///
    /// * `value` - The E-Mail of the user
    pub fn with_email<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.email = Some(value.into());
        self
    }

    /// Sets a new name of a user.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the user.
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }

    /// Sets a new phone number.
    ///
    /// # Arguments
    ///
    /// * `value` - The phone number of the user.
    pub fn with_phone_number<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.phone_number = Some(value.into());
        self
    }

    /// Sets a new shipping address.
    ///
    /// # Arguments
    ///
    /// * `value` - The shipping address of the user.
    pub fn with_shipping_address(mut self, value: ShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::types::{Integer, LabeledPrice};

#[cfg(test)]
mod tests;

/// Represents the content of an invoice message to be sent as the result of an inline query
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentInvoice {
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<Integer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
}

impl InputMessageContentInvoice {
    /// Creates a new InputMessageContentInvoice
    ///
    /// # Arguments
    ///
    /// * title - Product name, 1-32 characters
    /// * description - Product description, 1-255 characters
    /// * payload - Bot-defined invoice payload, 1-128 bytes.
    ///             This will not be displayed to the user,
    ///             use for your internal processes.
    /// * provider_token - Payment provider token, obtained via Bot Father
    /// * currency - Three-letter ISO 4217 currency code, see more on currencies
    /// * prices - Price breakdown  (e.g. product price, tax, discount,
    ///            delivery cost, delivery tax, bonus, etc.)
    pub fn new<A, B, C, D, E, F>(
        title: A,
        description: B,
        payload: C,
        provider_token: D,
        currency: E,
        prices: F,
    ) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
        F: IntoIterator<Item = LabeledPrice>,
    {
        Self {
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            currency: currency.into(),
            prices: prices.into_iter().collect(),
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }

    /// Sets the maximum accepted amount for tips in the smallest units of the currency
    ///
    /// For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145.
    /// See the exp parameter in [currencies.json], it shows the number
    // of digits past the decimal point for each currency
    /// (2 for the majority of currencies). Defaults to 0
    ///
    /// [currencies.json]: https://core.telegram.org/bots/payments/currencies.json
    pub fn max_tip_amount(mut self, value: Integer) -> Self {
        self.max_tip_amount = Some(value);
        self
    }

    /// Sets an array of suggested amounts of tip in the smallest units of the currency
    ///
    /// At most 4 suggested tip amounts can be specified.
    /// The suggested tip amounts must be positive, passed
    /// in a strictly increased order and must not exceed max_tip_amount.
    pub fn suggested_tip_amounts(mut self, value: impl IntoIterator<Item = Integer>) -> Self {
        self.suggested_tip_amounts = Some(value.into_iter().collect());
        self
    }

    /// An object for data about the invoice,
    /// which will be shared with the payment provider.
    /// A detailed description of the required fields should be provided by the payment provider.
    pub fn provider_data<T>(mut self, value: &T) -> Result<Self, JsonError>
    where
        T: Serialize,
    {
        self.provider_data = Some(serde_json::to_string(value)?);
        Ok(self)
    }

    /// Sets an URL of the product photo for the invoice
    ///
    /// Can be a photo of the goods or a marketing image for a service.
    /// People like it better when they see what they are paying for.
    pub fn photo_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.photo_url = Some(value.into());
        self
    }

    /// Sets a photo size
    pub fn photo_size(mut self, value: Integer) -> Self {
        self.photo_size = Some(value);
        self
    }

    /// Sets a photo width
    pub fn photo_width(mut self, value: Integer) -> Self {
        self.photo_width = Some(value);
        self
    }

    /// Sets a photo height
    pub fn photo_height(mut self, value: Integer) -> Self {
        self.photo_height = Some(value);
        self
    }

    /// Pass True, if you require the user's full name to complete the order
    pub fn need_name(mut self, value: bool) -> Self {
        self.need_name = Some(value);
        self
    }

    /// Pass True, if you require the user's phone number to complete the order
    pub fn need_phone_number(mut self, value: bool) -> Self {
        self.need_phone_number = Some(value);
        self
    }

    /// Pass True, if you require the user's email address to complete the order
    pub fn need_email(mut self, value: bool) -> Self {
        self.need_email = Some(value);
        self
    }

    /// Pass True, if you require the user's shipping address to complete the order
    pub fn need_shipping_address(mut self, value: bool) -> Self {
        self.need_shipping_address = Some(value);
        self
    }

    /// Pass True, if user's phone number should be sent to provider
    pub fn send_phone_number_to_provider(mut self, value: bool) -> Self {
        self.send_phone_number_to_provider = Some(value);
        self
    }

    /// Pass True, if user's email address should be sent to provider
    pub fn send_email_to_provider(mut self, value: bool) -> Self {
        self.send_email_to_provider = Some(value);
        self
    }

    /// Pass True, if the final price depends on the shipping method
    pub fn flexible(mut self, value: bool) -> Self {
        self.is_flexible = Some(value);
        self
    }
}

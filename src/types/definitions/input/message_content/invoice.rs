use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::types::{Integer, LabeledPrice};

/// Represents an invoice message to be sent as the result of an inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentInvoice {
    currency: String,
    description: String,
    payload: String,
    prices: Vec<LabeledPrice>,
    title: String,
    is_flexible: Option<bool>,
    max_tip_amount: Option<Integer>,
    need_email: Option<bool>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_shipping_address: Option<bool>,
    photo_height: Option<Integer>,
    photo_size: Option<Integer>,
    photo_width: Option<Integer>,
    photo_url: Option<String>,
    provider_data: Option<String>,
    provider_token: Option<String>,
    send_email_to_provider: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    suggested_tip_amounts: Option<Vec<Integer>>,
}

impl InputMessageContentInvoice {
    /// Creates a new `InputMessageContentInvoice`.
    ///
    /// # Arguments
    ///
    /// * `currency` - Three-letter ISO 4217 currency code.
    /// * `description` - Product description; 1-255 characters.
    /// * `payload` - Bot-defined invoice payload; 1-128 bytes;
    ///   this will not be displayed to the user,
    ///   use for your internal processes.
    /// * `prices` - Price breakdown (e.g. product price, tax, discount,
    ///   delivery cost, delivery tax, bonus, etc.).
    /// * `title` - Product name; 1-32 characters.
    pub fn new<A, B, C, D, E>(currency: A, description: B, payload: C, prices: D, title: E) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: IntoIterator<Item = LabeledPrice>,
        E: Into<String>,
    {
        Self {
            currency: currency.into(),
            description: description.into(),
            payload: payload.into(),
            prices: prices.into_iter().collect(),
            title: title.into(),
            is_flexible: None,
            max_tip_amount: None,
            need_email: None,
            need_name: None,
            need_phone_number: None,
            need_shipping_address: None,
            photo_height: None,
            photo_size: None,
            photo_width: None,
            photo_url: None,
            provider_data: None,
            provider_token: None,
            send_email_to_provider: None,
            send_phone_number_to_provider: None,
            suggested_tip_amounts: None,
        }
    }

    /// Sets a new value for the `is_flexible` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the final price depends on the shipping method.
    pub fn with_is_flexible(mut self, value: bool) -> Self {
        self.is_flexible = Some(value);
        self
    }

    /// Sets a new max tip amount.
    ///
    /// # Arguments
    ///
    /// * `value` - Maximum accepted amount for tips in the smallest units of the currency; default - 0.
    ///
    /// For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145.
    /// See the exp parameter in [currencies.json], it shows the number
    /// of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    ///
    /// [currencies.json]: https://core.telegram.org/bots/payments/currencies.json
    pub fn with_max_tip_amount(mut self, value: Integer) -> Self {
        self.max_tip_amount = Some(value);
        self
    }

    /// Sets a new value for the `need_email` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether an email address of a user is required to complete the order.
    pub fn with_need_email(mut self, value: bool) -> Self {
        self.need_email = Some(value);
        self
    }

    /// Sets a new value for the `need_name` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a full name of a user is required to complete the order.
    pub fn with_need_name(mut self, value: bool) -> Self {
        self.need_name = Some(value);
        self
    }

    /// Sets a new value for the `need_phone_number` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a phone number of a user is required to complete the order.
    pub fn with_need_phone_number(mut self, value: bool) -> Self {
        self.need_phone_number = Some(value);
        self
    }

    /// Sets a new value for the `need_shipping_address` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a shipping address of a user
    ///   is required to complete the order.
    pub fn with_need_shipping_address(mut self, value: bool) -> Self {
        self.need_shipping_address = Some(value);
        self
    }

    /// Sets a new photo height.
    ///
    /// # Arguments
    ///
    /// * `value` - Photo height.
    pub fn with_photo_height(mut self, value: Integer) -> Self {
        self.photo_height = Some(value);
        self
    }

    /// Sets a new photo size.
    ///
    /// # Arguments
    ///
    /// * `value` - Photo size in bytes.
    pub fn with_photo_size(mut self, value: Integer) -> Self {
        self.photo_size = Some(value);
        self
    }

    /// Sets a new photo width.
    ///
    /// # Arguments
    ///
    /// * `value` - Photo width.
    pub fn with_photo_width(mut self, value: Integer) -> Self {
        self.photo_width = Some(value);
        self
    }

    /// Sets a new photo URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the product photo for the invoice.
    ///
    /// Can be a photo of the goods or a marketing image for a service.
    /// People like it better when they see what they are paying for.
    pub fn with_photo_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.photo_url = Some(value.into());
        self
    }

    /// Sets a new provider data.
    ///
    /// # Arguments
    ///
    /// * `value` - An object for data about the invoice,
    ///   which will be shared with the payment provider.
    ///
    /// A detailed description of the required fields should be provided by the payment provider.
    pub fn with_provider_data<T>(mut self, value: &T) -> Result<Self, JsonError>
    where
        T: Serialize,
    {
        self.provider_data = Some(serde_json::to_string(value)?);
        Ok(self)
    }

    /// Sets a new provider token.
    ///
    /// # Arguments
    ///
    /// * `value` - Payment provider token, obtained via @BotFather;
    ///   pass an empty string for payments in Telegram Stars.
    pub fn with_provider_token<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.provider_token = Some(value.into());
        self
    }

    /// Sets a new value for the `send_email_to_provider` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether an email address of a user should be sent to provider.
    pub fn with_send_email_to_provider(mut self, value: bool) -> Self {
        self.send_email_to_provider = Some(value);
        self
    }

    /// Sets a new value for the `send_phone_number_to_provider` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a phone number of a user should be sent to provider.
    pub fn with_send_phone_number_to_provider(mut self, value: bool) -> Self {
        self.send_phone_number_to_provider = Some(value);
        self
    }

    /// Sets a new suggested tip amounts.
    ///
    /// # Arguments
    ///
    /// * `value` - Array of suggested amounts of tip in the smallest units of the currency.
    ///
    /// At most 4 suggested tip amounts can be specified.
    /// The suggested tip amounts must be positive, passed
    /// in a strictly increased order and must not exceed `max_tip_amount`.
    pub fn with_suggested_tip_amounts<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = Integer>,
    {
        self.suggested_tip_amounts = Some(value.into_iter().collect());
        self
    }
}

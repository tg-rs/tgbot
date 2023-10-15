use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::{
    api::{Method, Payload},
    types::{ChatId, InlineKeyboardMarkup, Integer, Message},
};

#[cfg(test)]
mod tests;

/// Basic information about an invoice
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145
    /// See the exp parameter in currencies.json, it shows the number of digits past
    /// the decimal point for each currency (2 for the majority of currencies)
    pub total_amount: Integer,
}

/// Portion of the price for goods or services
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct LabeledPrice {
    label: String,
    amount: Integer,
}

impl LabeledPrice {
    /// Creates a new LabeledPrice
    ///
    /// # Arguments
    ///
    /// * label - Portion label
    /// * amount - For example, for a price of US$ 1.45 pass amount = 145
    ///            See the exp parameter in currencies.json, it shows the number of digits past the
    ///            decimal point for each currency (2 for the majority of currencies)
    pub fn new<S: Into<String>>(label: S, amount: Integer) -> Self {
        Self {
            label: label.into(),
            amount,
        }
    }

    /// Returns a portion label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns an amount
    pub fn amount(&self) -> Integer {
        self.amount
    }
}

/// Invoice parameters used in CreateInvoiceLink and SendInvoice
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InvoiceParameters {
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145.
    /// See the exp parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json),
    /// it shows the number of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    ///
    /// Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<Integer>,
    /// An array of suggested amounts of tips in the smallest units of the currency
    /// (integer, not float/double)
    ///
    /// At most 4 suggested tip amounts can be specified.
    /// The suggested tip amounts must be positive,
    /// passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<Integer>>,
    /// Data about the invoice, which will be shared with the payment provider
    ///
    /// A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice
    ///
    /// Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<Integer>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<Integer>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<Integer>,
    /// Pass True if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass True if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass True if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass True if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass True if the user's phone number should be sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True if the user's email address should be sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass True if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

impl InvoiceParameters {
    /// The maximum accepted amount for tips in the smallest units of the currency
    ///
    /// For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145.
    /// See the exp parameter in currencies.json, it shows the number of digits
    /// past the decimal point for each currency
    /// (2 for the majority of currencies). Defaults to 0
    pub fn with_max_tip_amount(mut self, value: Integer) -> Self {
        self.max_tip_amount = Some(value);
        self
    }

    /// An array of suggested amounts of tips in the smallest units of the currency
    ///
    /// At most 4 suggested tip amounts can be specified. The suggested tip amounts
    /// must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    pub fn with_suggested_tip_amounts<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = Integer>,
    {
        self.suggested_tip_amounts = Some(value.into_iter().collect());
        self
    }

    /// JSON-encoded data about the invoice, which will be shared with the payment provider
    ///
    /// A detailed description of required fields should be provided by the payment provider
    pub fn with_provider_data<T>(mut self, value: &T) -> Result<Self, JsonError>
    where
        T: Serialize,
    {
        self.provider_data = Some(serde_json::to_string(value)?);
        Ok(self)
    }

    /// URL of the product photo for the invoice
    ///
    /// Can be a photo of the goods or a marketing image for a service
    /// People like it better when they see what they are paying for
    pub fn with_photo_url<S: Into<String>>(mut self, photo_url: S) -> Self {
        self.photo_url = Some(photo_url.into());
        self
    }

    /// Photo size
    pub fn with_photo_size(mut self, photo_size: Integer) -> Self {
        self.photo_size = Some(photo_size);
        self
    }

    /// Photo width
    pub fn with_photo_width(mut self, photo_width: Integer) -> Self {
        self.photo_width = Some(photo_width);
        self
    }

    /// Photo height
    pub fn with_photo_height(mut self, photo_height: Integer) -> Self {
        self.photo_height = Some(photo_height);
        self
    }

    /// Pass True, if you require the user's full name to complete the order
    pub fn with_need_name(mut self, need_name: bool) -> Self {
        self.need_name = Some(need_name);
        self
    }

    /// Pass True, if you require the user's phone number to complete the order
    pub fn with_need_phone_number(mut self, need_phone_number: bool) -> Self {
        self.need_phone_number = Some(need_phone_number);
        self
    }

    /// Pass True, if you require the user's email address to complete the order
    pub fn with_need_email(mut self, need_email: bool) -> Self {
        self.need_email = Some(need_email);
        self
    }

    /// Pass True, if you require the user's shipping address to complete the order
    pub fn with_need_shipping_address(mut self, need_shipping_address: bool) -> Self {
        self.need_shipping_address = Some(need_shipping_address);
        self
    }

    /// Pass True, if user's phone number should be sent to provider
    pub fn with_send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
        self
    }

    /// Pass True, if user's email address should be sent to provider
    pub fn with_send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
        self.send_email_to_provider = Some(send_email_to_provider);
        self
    }

    /// Pass True, if the final price depends on the shipping method
    pub fn with_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }
}

/// Create a link for an invoice
///
/// Returns the created invoice link as String on success.
#[derive(Clone, Debug, Serialize)]
pub struct CreateInvoiceLink {
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(flatten)]
    parameters: Option<InvoiceParameters>,
}

impl CreateInvoiceLink {
    /// Creates a new CreateInvoiceLink
    ///
    /// # Arguments
    ///
    /// * title - Product name, 1-32 characters
    /// * description - Product description, 1-255 characters
    /// * payload - Bot-defined invoice payload, 1-128 bytes.
    ///             This will not be displayed to the user, use for your internal processes.
    /// * provider_token - Payment provider token, obtained via BotFather
    /// * currency - Three-letter ISO 4217 currency code, see more on currencies
    /// * prices - Price breakdown (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub fn new<A, B, C, D, E>(title: A, description: B, payload: C, provider_token: D, currency: D, prices: E) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: IntoIterator<Item = LabeledPrice>,
    {
        Self {
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            currency: currency.into(),
            prices: prices.into_iter().collect(),
            parameters: None,
        }
    }

    /// Sets invoice parameters
    pub fn parameters(mut self, parameters: InvoiceParameters) -> Self {
        self.parameters = Some(parameters);
        self
    }
}

impl Method for CreateInvoiceLink {
    type Response = String;

    fn into_payload(self) -> Payload {
        Payload::json("createInvoiceLink", self)
    }
}

/// Send invoice
#[derive(Clone, Debug, Serialize)]
pub struct SendInvoice {
    chat_id: ChatId,
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(flatten)]
    parameters: Option<InvoiceParameters>,
}

impl SendInvoice {
    /// Creates a new SendInvoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat or username of the target channel (in the format @channel_username)
    /// * title - Product name, 1-32 characters
    /// * description - Product description, 1-255 characters
    /// * payload - Bot-defined invoice payload, 1-128 bytes
    ///             This will not be displayed to the user, use for your internal processes
    /// * provider_token - Payments provider token, obtained via Bot Father
    /// * currency - Three-letter ISO 4217 currency code, see more on currencies
    /// * prices - Price breakdown, a list of components
    ///            (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    #[allow(clippy::too_many_arguments)]
    pub fn new<A, B, C, D, E, F, G>(
        chat_id: A,
        title: B,
        description: C,
        payload: D,
        provider_token: E,
        currency: F,
        prices: G,
    ) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
        F: Into<String>,
        G: IntoIterator<Item = LabeledPrice>,
    {
        SendInvoice {
            chat_id: chat_id.into(),
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            currency: currency.into(),
            prices: prices.into_iter().collect(),
            start_parameter: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
            parameters: None,
        }
    }

    /// Sets an Unique deep-linking parameter
    ///
    ///  If left empty, forwarded copies of the sent message will have a Pay button,
    /// allowing multiple users to pay directly from the forwarded message, using the same invoice.
    /// If non-empty, forwarded copies of the sent message will have a URL button
    /// with a deep link to the bot (instead of a Pay button),
    /// with the value used as the start parameter
    pub fn start_parameter<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.start_parameter = Some(value.into());
        self
    }

    /// Sets invoice parameters
    pub fn parameters(mut self, parameters: InvoiceParameters) -> Self {
        self.parameters = Some(parameters);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Inline keyboard
    ///
    /// If empty, one 'Pay total price' button will be shown
    /// If not empty, the first button must be a Pay button
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendInvoice {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendInvoice", self)
    }
}

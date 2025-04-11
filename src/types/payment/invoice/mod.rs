use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::{
    api::{Method, Payload},
    types::{ChatId, InlineKeyboardMarkup, Integer, Message, ReplyParameters},
};

#[cfg(test)]
mod tests;

/// Represents an invoice.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Invoice {
    /// Three-letter ISO 4217 currency code.
    pub currency: String,
    /// Product description.
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice.
    pub start_parameter: String,
    /// Product name.
    pub title: String,
    /// Total price in the smallest units of the currency (integer, not float/double).
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145.
    /// See the exp parameter in [currencies.json][1], it shows the number of digits past
    /// the decimal point for each currency (2 for the majority of currencies).
    ///
    /// [1]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: Integer,
}

impl Invoice {
    /// Creates a new `Invoice`.
    ///
    /// # Arguments
    ///
    /// * `currency` - ISO 4217 currency code.
    /// * `description` - Product description.
    /// * `start_parameter` - Unique bot deep-linking parameter.
    /// * `title` - Product name.
    /// * `total_amount` - Total price.
    pub fn new<A, B, C, D>(currency: A, description: B, start_parameter: C, title: D, total_amount: Integer) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
    {
        Self {
            currency: currency.into(),
            description: description.into(),
            start_parameter: start_parameter.into(),
            title: title.into(),
            total_amount,
        }
    }
}

/// Represents a portion of the price for goods or services.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct LabeledPrice {
    amount: Integer,
    label: String,
}

impl LabeledPrice {
    /// Creates a new `LabeledPrice`.
    ///
    /// # Arguments
    ///
    /// * `amount` - Price of the product in the smallest units of the currency.
    /// * `label` - Portion label.
    pub fn new<T>(amount: Integer, label: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            amount,
            label: label.into(),
        }
    }

    /// Returns the amount.
    pub fn amount(&self) -> Integer {
        self.amount
    }

    /// Returns the portion label.
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Represents an invoice parameters used in [`CreateInvoiceLink`] and [`SendInvoice`].
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InvoiceParameters {
    /// Indicates whether the final price depends on the shipping method.
    pub is_flexible: Option<bool>,
    /// The maximum accepted amount for tips in the smallest units of the currency.
    ///
    /// For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145.
    /// See the exp parameter in [currencies.json][1],
    /// it shows the number of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    ///
    /// Defaults to 0.
    ///
    /// [1]: (https://core.telegram.org/bots/payments/currencies.json)
    pub max_tip_amount: Option<Integer>,
    /// Indicates whether the user's email address is required to complete the order.
    pub need_email: Option<bool>,
    /// Indicates whether the user's full name is required to complete the order.
    pub need_name: Option<bool>,
    /// Indicates whether the user's phone number is required to complete the order.
    pub need_phone_number: Option<bool>,
    /// Indicates whether the user's shipping address is required to complete the order.
    pub need_shipping_address: Option<bool>,
    /// Photo height.
    pub photo_height: Option<Integer>,
    /// Photo size in bytes.
    pub photo_size: Option<Integer>,
    /// URL of the product photo for the invoice.
    ///
    /// Can be a photo of the goods or a marketing image for a service.
    pub photo_url: Option<String>,
    /// Photo width.
    pub photo_width: Option<Integer>,
    /// Data about the invoice, which will be shared with the payment provider.
    ///
    /// A detailed description of required fields should be provided by the payment provider.
    pub provider_data: Option<String>,
    /// Payment provider token, obtained via @BotFather.
    ///
    /// Pass an empty string for payments in Telegram Stars.
    pub provider_token: Option<String>,
    /// Indicates whether the user's phone number should be sent to the provider.
    pub send_phone_number_to_provider: Option<bool>,
    /// Indicates whether the user's email address should be sent to the provider.
    pub send_email_to_provider: Option<bool>,
    /// An array of suggested amounts of tips in the smallest units of the currency.
    ///
    /// At most 4 suggested tip amounts can be specified.
    /// The suggested tip amounts must be positive,
    /// passed in a strictly increased order and must not exceed `max_tip_amount`.
    pub suggested_tip_amounts: Option<Vec<Integer>>,
}

impl InvoiceParameters {
    /// Sets a new value for the `is_flexible` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the final price depends on the shipping method.
    pub fn with_flexible(mut self, value: bool) -> Self {
        self.is_flexible = Some(value);
        self
    }

    /// Sets a new max tip amount.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum accepted amount for tips in the smallest units of the currency.
    pub fn with_max_tip_amount(mut self, value: Integer) -> Self {
        self.max_tip_amount = Some(value);
        self
    }

    /// Sets a new value for the `need_email` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user's email address is required to complete the order.
    pub fn with_need_email(mut self, value: bool) -> Self {
        self.need_email = Some(value);
        self
    }

    /// Sets a new value for the `need_name` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user's full name is required to complete the order.
    pub fn with_need_name(mut self, value: bool) -> Self {
        self.need_name = Some(value);
        self
    }

    /// Sets a new value for the `need_phone_number` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user's phone number is required to complete the order.
    pub fn with_need_phone_number(mut self, value: bool) -> Self {
        self.need_phone_number = Some(value);
        self
    }

    /// Sets a new value for the `need_shipping_address` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user's shipping address is required to complete the order.
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

    /// Sets a new photo URL.
    ///
    /// # Arguments
    ///
    /// * `value` - Photo URL.
    pub fn with_photo_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.photo_url = Some(value.into());
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

    /// Sets a new provider data.
    ///
    /// # Arguments
    ///
    /// * `value` - Data about the invoice, which will be shared with the payment provider.
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
    /// * `value` - Payment provider token, obtained via @BotFather.
    ///   Pass an empty string for payments in Telegram Stars.
    pub fn with_provider_token<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.provider_token = Some(value.into());
        self
    }

    /// Sets a new value for the `send_phone_number_to_provider` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user's phone number should be sent to the provider.
    pub fn with_send_phone_number_to_provider(mut self, value: bool) -> Self {
        self.send_phone_number_to_provider = Some(value);
        self
    }

    /// Sets a new value for the `send_email_to_provider` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user's email address should be sent to the provider.
    pub fn with_send_email_to_provider(mut self, value: bool) -> Self {
        self.send_email_to_provider = Some(value);
        self
    }

    /// Sets a new list of max tip amounts.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of suggested amounts of tips in the smallest units of the currency.
    pub fn with_suggested_tip_amounts<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = Integer>,
    {
        self.suggested_tip_amounts = Some(value.into_iter().collect());
        self
    }
}

/// Creates a link for an invoice.
///
/// Returns the created invoice link as String on success.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CreateInvoiceLink {
    currency: String,
    description: String,
    payload: String,
    prices: Vec<LabeledPrice>,
    title: String,
    business_connection_id: Option<String>,
    subscription_period: Option<Integer>,
    #[serde(flatten)]
    parameters: Option<InvoiceParameters>,
}

impl CreateInvoiceLink {
    /// Creates a new `CreateInvoiceLink`.
    ///
    /// # Arguments
    ///
    /// * `title` - Product name; 1-32 characters.
    /// * `description` - Product description; 1-255 characters.
    /// * `payload` - Bot-defined invoice payload; 1-128 bytes;
    ///   this will not be displayed to the user;
    ///   use for your internal processes.
    /// * `currency` - Three-letter ISO 4217 currency code, see more on currencies.
    /// * `prices` - Price breakdown
    ///   (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.).
    pub fn new<A, B, C, D, E>(title: A, description: B, payload: C, currency: D, prices: E) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: IntoIterator<Item = LabeledPrice>,
    {
        Self {
            currency: currency.into(),
            description: description.into(),
            payload: payload.into(),
            prices: prices.into_iter().collect(),
            title: title.into(),
            business_connection_id: None,
            subscription_period: None,
            parameters: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the link will be created.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new invoice parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Invoice parameters.
    pub fn with_parameters(mut self, value: InvoiceParameters) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Sets a new subscription period.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of seconds the subscription will be active for before the next payment.
    ///   The currency must be set to “XTR” (Telegram Stars) if the parameter is used.
    ///   Currently, it must always be 2592000 (30 days) if specified.
    pub fn with_subscription_period(mut self, value: Integer) -> Self {
        self.subscription_period = Some(value);
        self
    }
}

impl Method for CreateInvoiceLink {
    type Response = String;

    fn into_payload(self) -> Payload {
        Payload::json("createInvoiceLink", self)
    }
}

/// Sends an invoice.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendInvoice {
    chat_id: ChatId,
    currency: String,
    description: String,
    payload: String,
    prices: Vec<LabeledPrice>,
    title: String,
    allow_paid_broadcast: Option<bool>,
    disable_notification: Option<bool>,
    message_effect_id: Option<String>,
    message_thread_id: Option<Integer>,
    #[serde(flatten)]
    parameters: Option<InvoiceParameters>,
    protect_content: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
    reply_parameters: Option<ReplyParameters>,
    start_parameter: Option<String>,
}

impl SendInvoice {
    /// Creates a new `SendInvoice`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `title` - Product name; 1-32 characters.
    /// * `description` - Product description; 1-255 characters.
    /// * `payload` - Bot-defined invoice payload; 1-128 bytes
    ///   this will not be displayed to the user;
    ///   use for your internal processes.
    /// * `currency` - Three-letter ISO 4217 currency code, see more on currencies.
    /// * `prices` - Price breakdown, a list of components
    ///   (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.).
    pub fn new<A, B, C, D, E, F>(chat_id: A, title: B, description: C, payload: D, currency: E, prices: F) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
        F: IntoIterator<Item = LabeledPrice>,
    {
        SendInvoice {
            chat_id: chat_id.into(),
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            currency: currency.into(),
            prices: prices.into_iter().collect(),
            allow_paid_broadcast: None,
            disable_notification: None,
            message_effect_id: None,
            message_thread_id: None,
            parameters: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
            start_parameter: None,
        }
    }

    /// Sets a new value for the `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///   for a fee of 0.1 Telegram Stars per message.
    ///   The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.allow_paid_broadcast = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect to be added to the message; for private chats only.
    pub fn with_message_effect_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.message_effect_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new invoice parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Invoice parameters.
    pub fn with_parameters(mut self, value: InvoiceParameters) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    ///
    /// If empty, one 'Pay total price' button will be shown.
    /// If not empty, the first button must be a Pay button.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.reply_parameters = Some(value);
        self
    }

    /// Sets a new unique deep-linking parameter.
    ///
    /// # Arguments
    ///
    /// * `value` - Value of the parameter.
    ///
    /// If left empty, forwarded copies of the sent message will have a Pay button,
    /// allowing multiple users to pay directly from the forwarded message, using the same invoice.
    /// If non-empty, forwarded copies of the sent message will have a URL button
    /// with a deep link to the bot (instead of a Pay button),
    /// with the value used as the start parameter.
    pub fn with_start_parameter<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.start_parameter = Some(value.into());
        self
    }
}

impl Method for SendInvoice {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendInvoice", self)
    }
}

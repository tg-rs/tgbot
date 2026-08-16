use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload, PayloadError},
    types::{LabeledPrice, User},
};

/// Represents a shipping address.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ShippingAddress {
    /// City.
    pub city: String,
    /// ISO 3166-1 alpha-2 country code.
    pub country_code: String,
    /// Address post code.
    pub post_code: String,
    /// State, if applicable.
    pub state: String,
    /// First line for the address.
    pub street_line1: String,
    /// Second line for the address.
    pub street_line2: String,
}

/// Represents a shipping option.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

impl ShippingOption {
    /// Creates a new `ShippingOption`.
    ///
    /// # Arguments
    ///
    /// * `id` - Shipping option identifier.
    /// * `title` - Option title.
    /// * `prices` - List of price portions.
    pub fn new<A, B, C>(id: A, title: B, prices: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: IntoIterator<Item = LabeledPrice>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            prices: prices.into_iter().collect(),
        }
    }

    /// Returns the ID of the option.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the title of the option.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the list of price portions.
    pub fn prices(&self) -> &[LabeledPrice] {
        &self.prices
    }
}

/// Represents a shipping query.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ShippingQuery {
    /// Unique query identifier.
    pub id: String,
    /// User who sent the query.
    pub from: User,
    /// Bot specified invoice payload.
    pub invoice_payload: String,
    /// User specified shipping address.
    pub shipping_address: ShippingAddress,
}

/// Replies to a shipping query.
///
/// If you sent an invoice requesting a shipping address
/// and the parameter `is_flexible` was specified,
/// the Bot API will send an [`crate::types::UpdateType::ShippingQuery`] to the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct AnswerShippingQuery {
    ok: bool,
    shipping_query_id: String,
    error_message: Option<String>,
    shipping_options: Option<Vec<ShippingOption>>,
}

impl AnswerShippingQuery {
    /// Creates a new `AnswerShippingQuery` with a success answer.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the query to be answered.
    /// * `options` - Array of available shipping options.
    pub fn ok<A, B>(id: A, options: B) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = ShippingOption>,
    {
        Self {
            ok: true,
            shipping_query_id: id.into(),
            error_message: None,
            shipping_options: Some(options.into_iter().collect()),
        }
    }

    /// Creates a new `AnswerShippingQuery` with an error answer.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the query to be answered.
    /// * `message` - Error message in human readable form.
    pub fn error<A, B>(id: A, message: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            ok: false,
            shipping_query_id: id.into(),
            error_message: Some(message.into()),
            shipping_options: None,
        }
    }
}

impl Method for AnswerShippingQuery {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("answerShippingQuery", self)
    }
}

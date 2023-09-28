use serde::{Deserialize, Serialize};

use crate::{
    method::Method,
    request::Request,
    types::{LabeledPrice, User},
};

#[cfg(test)]
mod tests;

/// Shipping address
#[derive(Clone, Debug, Deserialize)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}

/// Shipping option
#[derive(Clone, Debug, Serialize)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

impl ShippingOption {
    /// Creates a new ShippingOption
    ///
    /// # Arguments
    ///
    /// * id - Shipping option identifier
    /// * title - Option title
    /// * prices - List of price portions
    pub fn new<I, T>(id: I, title: T, prices: Vec<LabeledPrice>) -> Self
    where
        I: Into<String>,
        T: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            prices,
        }
    }

    /// Returns an option id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns an option title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns price portions
    pub fn prices(&self) -> &[LabeledPrice] {
        &self.prices
    }
}

/// Information about an incoming shipping query
#[derive(Clone, Debug, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}

/// Reply to shipping query
///
/// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified,
/// the Bot API will send an Update with a shipping_query field to the bot
#[derive(Clone, Debug, Serialize)]
pub struct AnswerShippingQuery {
    shipping_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl AnswerShippingQuery {
    /// Success answer
    ///
    /// # Arguments
    ///
    /// * shipping_query_id - Unique identifier for the query to be answered
    /// * shipping_options - Array of available shipping options
    pub fn ok<S: Into<String>>(shipping_query_id: S, shipping_options: Vec<ShippingOption>) -> Self {
        AnswerShippingQuery {
            shipping_query_id: shipping_query_id.into(),
            ok: true,
            shipping_options: Some(shipping_options),
            error_message: None,
        }
    }

    /// Error answer
    ///
    /// # Arguments
    ///
    /// * shipping_query_id - Unique identifier for the query to be answered
    /// * error_message - Error message in human readable form
    pub fn error<S: Into<String>>(shipping_query_id: S, error_message: S) -> Self {
        AnswerShippingQuery {
            shipping_query_id: shipping_query_id.into(),
            ok: false,
            shipping_options: None,
            error_message: Some(error_message.into()),
        }
    }
}

impl Method for AnswerShippingQuery {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("answerShippingQuery", self)
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    method::Method,
    request::Request,
    types::{Integer, OrderInfo, User},
};

#[cfg(test)]
mod tests;

/// Information about an incoming pre-checkout query
#[derive(Clone, Debug, Deserialize, PartialOrd, PartialEq, Serialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145
    /// See the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies)
    pub total_amount: Integer,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}

/// Respond to pre-checkout query
///
/// Once the user has confirmed their payment and shipping details,
/// the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query
/// Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent
#[derive(Clone, Debug, Serialize)]
pub struct AnswerPreCheckoutQuery {
    pre_checkout_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl AnswerPreCheckoutQuery {
    /// Success answer
    ///
    /// # Arguments
    ///
    /// * pre_checkout_query_id - Unique identifier for the query to be answered
    pub fn ok<S: Into<String>>(pre_checkout_query_id: S) -> Self {
        AnswerPreCheckoutQuery {
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok: true,
            error_message: None,
        }
    }

    /// Error answer
    ///
    /// # Arguments
    ///
    /// * pre_checkout_query_id - Unique identifier for the query to be answered
    /// * error_message - Error message in human readable form
    ///                   that explains the reason for failure to proceed with the checkout
    pub fn error<S: Into<String>>(pre_checkout_query_id: S, error_message: S) -> Self {
        AnswerPreCheckoutQuery {
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok: false,
            error_message: Some(error_message.into()),
        }
    }
}

impl Method for AnswerPreCheckoutQuery {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("answerPreCheckoutQuery", self)
    }
}

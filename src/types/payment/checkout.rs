use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload, PayloadError},
    types::{Integer, OrderInfo, User},
};

/// Represents an incoming pre-checkout query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialOrd, PartialEq, Serialize)]
pub struct PreCheckoutQuery {
    /// Three-letter ISO 4217 currency code.
    pub currency: String,
    /// User who sent the query.
    pub from: User,
    /// Unique query identifier.
    pub id: String,
    /// Bot specified invoice payload.
    pub invoice_payload: String,
    /// Total price in the smallest units of the currency (integer, not float/double).
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145.
    /// See the exp parameter in [currencies.json][1], it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies).
    ///
    /// [1]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: Integer,
    /// Order info provided by the user.
    pub order_info: Option<OrderInfo>,
    /// Identifier of the shipping option chosen by the user.
    pub shipping_option_id: Option<String>,
}

/// Responds to a pre-checkout query.
///
/// Once the user has confirmed their payment and shipping details,
/// the Bot API sends the final confirmation in the form of an
/// [`crate::types::UpdateType::PreCheckoutQuery`].
///
/// # Notes
///
/// The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct AnswerPreCheckoutQuery {
    ok: bool,
    pre_checkout_query_id: String,
    error_message: Option<String>,
}

impl AnswerPreCheckoutQuery {
    /// Creates a new `AnswerPreCheckoutQuery` with a success answer.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the query to be answered.
    pub fn ok<T>(id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pre_checkout_query_id: id.into(),
            ok: true,
            error_message: None,
        }
    }

    /// Creates a new `AnswerPreCheckoutQuery` with an error answer.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the query to be answered.
    /// * `message` - Error message in human readable form
    ///   that explains the reason for failure to proceed with the checkout.
    pub fn error<A, B>(id: A, message: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            ok: false,
            pre_checkout_query_id: id.into(),
            error_message: Some(message.into()),
        }
    }
}

impl Method for AnswerPreCheckoutQuery {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("answerPreCheckoutQuery", self)
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::InlineQueryResult,
};

/// An inline message sent by a guest bot.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message
    pub inline_message_id: String,
}

impl From<&str> for SentGuestMessage {
    fn from(value: &str) -> Self {
        Self {
            inline_message_id: String::from(value),
        }
    }
}

impl From<String> for SentGuestMessage {
    fn from(value: String) -> Self {
        Self {
            inline_message_id: value,
        }
    }
}

impl From<SentGuestMessage> for String {
    fn from(value: SentGuestMessage) -> Self {
        value.inline_message_id
    }
}

/// Reply to a received guest message.
#[derive(Clone, Debug, Serialize)]
pub struct AnswerGuestQuery {
    guest_query_id: String,
    result: InlineQueryResult,
}

impl AnswerGuestQuery {
    /// Creates a new `AnswerGuestQuery`.
    ///
    /// # Arguments
    ///
    /// * `guest_query_id` - Unique identifier for the query to be answered.
    /// * `result` - The message to be sent.
    pub fn new<A, B>(guest_query_id: A, result: B) -> Self
    where
        A: Into<String>,
        B: Into<InlineQueryResult>,
    {
        Self {
            guest_query_id: guest_query_id.into(),
            result: result.into(),
        }
    }
}

impl Method for AnswerGuestQuery {
    type Response = SentGuestMessage;

    fn into_payload(self) -> Payload {
        Payload::json("answerGuestQuery", self)
    }
}

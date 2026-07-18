use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{InlineQueryResult, SerializeError},
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
#[derive(Debug)]
pub struct AnswerGuestQuery {
    form: Form,
}

impl AnswerGuestQuery {
    /// Creates a new `AnswerGuestQuery`.
    ///
    /// # Arguments
    ///
    /// * `guest_query_id` - Unique identifier for the query to be answered.
    /// * `result` - The message to be sent.
    pub fn new<A, B>(guest_query_id: A, result: B) -> Result<Self, SerializeError>
    where
        A: Into<String>,
        B: Into<InlineQueryResult>,
    {
        let (form, data) = result.into().into_parts();
        let mut form = form.unwrap_or_default();
        form.insert_field("guest_query_id", guest_query_id.into());
        form.insert_field("result", data.serialize()?);
        Ok(Self { form })
    }
}

impl Method for AnswerGuestQuery {
    type Response = SentGuestMessage;

    fn into_payload(self) -> Payload {
        Payload::form("answerGuestQuery", self.form)
    }
}

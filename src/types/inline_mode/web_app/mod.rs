use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::InlineQueryResult,
};

#[cfg(test)]
mod tests;

/// Represents an inline message sent by a Web App on behalf of a user
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message
    ///
    /// Available only if there is an inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl SentWebAppMessage {
    /// Sets a new inline message ID
    ///
    /// # Arguments
    ///
    /// * value - Inline message ID
    pub fn with_inline_message_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = Some(value.into());
        self
    }
}

/// Sets a result of an interaction with a Web App and
/// send a corresponding message on behalf of the user
/// to the chat from which the query originated
#[derive(Clone, Debug, Serialize)]
pub struct AnswerWebAppQuery {
    web_app_query_id: String,
    result: InlineQueryResult,
}

impl AnswerWebAppQuery {
    /// Creates a new AnswerWebAppQuery
    ///
    /// # Arguments
    ///
    /// * web_app_query_id - Unique identifier of the query to be answered
    /// * result - An object describing the message to be sent
    pub fn new<A, B>(result: A, web_app_query_id: B) -> Self
    where
        A: Into<InlineQueryResult>,
        B: Into<String>,
    {
        Self {
            result: result.into(),
            web_app_query_id: web_app_query_id.into(),
        }
    }
}

impl Method for AnswerWebAppQuery {
    type Response = SentWebAppMessage;

    fn into_payload(self) -> Payload {
        Payload::json("answerWebAppQuery", self)
    }
}

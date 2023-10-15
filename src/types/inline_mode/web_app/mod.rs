use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::InlineQueryResult,
};

#[cfg(test)]
mod tests;

/// Describes an inline message sent by a Web App on behalf of a user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message
    ///
    /// Available only if there is an inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// Set the result of an interaction with a Web App and
/// send a corresponding message on behalf of the user to the chat from which the query originated.
///
/// On success, a SentWebAppMessage object is returned.
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
    /// * web_app_query_id - Unique identifier for the query to be answered
    /// * result - An object describing the message to be sent
    pub fn new<I: Into<String>, R: Into<InlineQueryResult>>(web_app_query_id: I, result: R) -> Self {
        Self {
            web_app_query_id: web_app_query_id.into(),
            result: result.into(),
        }
    }
}

impl Method for AnswerWebAppQuery {
    type Response = SentWebAppMessage;

    fn into_payload(self) -> Payload {
        Payload::json("answerWebAppQuery", self)
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{InlineQueryResult, InlineQueryResultData},
};

/// Represents an inline message sent by a Web App on behalf of a user
#[serde_with::skip_serializing_none]
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message
    ///
    /// Available only if there is an inline keyboard attached to the message.
    pub inline_message_id: Option<String>,
}

impl SentWebAppMessage {
    /// Sets a new inline message ID
    ///
    /// # Arguments
    ///
    /// * `value` - Inline message ID
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
#[derive(Debug)]
pub struct AnswerWebAppQuery {
    result: InlineQueryResult,
    web_app_query_id: String,
}

impl AnswerWebAppQuery {
    /// Creates a new AnswerWebAppQuery
    ///
    /// # Arguments
    ///
    /// * `web_app_query_id` - Unique identifier of the query to be answered
    /// * `result` - An object describing the message to be sent
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

#[derive(Debug, Serialize)]
struct AnswerWebAppQueryParameters {
    result: InlineQueryResultData,
    web_app_query_id: String,
}

impl Method for AnswerWebAppQuery {
    type Response = SentWebAppMessage;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self {
            result,
            web_app_query_id,
        } = self;
        let mut form = Form::default();
        let parameters = AnswerWebAppQueryParameters {
            result: result.write(&mut form),
            web_app_query_id,
        };
        parameters.serialize(&mut form)?;
        Payload::form("answerWebAppQuery", form)
    }
}

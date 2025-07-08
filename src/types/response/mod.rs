use std::{error::Error, fmt};

use serde::Deserialize;

use crate::types::Integer;

#[cfg(test)]
mod tests;

/// Represents an API Response.
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "RawResponse<T>")]
pub enum Response<T> {
    /// Success.
    Success(T),
    /// Error.
    Error(ResponseError),
}

impl<T> Response<T> {
    /// Returns a number of seconds left to wait before the request can be repeated.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Response::Success(_) => None,
            Response::Error(err) => err.retry_after(),
        }
    }

    /// Converts the response into [`Result`].
    pub fn into_result(self) -> Result<T, ResponseError> {
        match self {
            Response::Success(obj) => Ok(obj),
            Response::Error(err) => Err(err),
        }
    }
}

impl<T> From<RawResponse<T>> for Response<T> {
    fn from(raw: RawResponse<T>) -> Self {
        if raw.ok {
            if let Some(result) = raw.result {
                Response::Success(result)
            } else {
                Response::Error(ResponseError {
                    description: String::from("response is ok, but result is not provided"),
                    error_code: None,
                    migrate_to_chat_id: None,
                    retry_after: None,
                })
            }
        } else {
            Response::Error(ResponseError {
                description: raw.description.unwrap_or_else(|| String::from("no description")),
                error_code: raw.error_code,
                migrate_to_chat_id: raw.parameters.and_then(|x| x.migrate_to_chat_id),
                retry_after: raw.parameters.and_then(|x| x.retry_after),
            })
        }
    }
}

/// Represents a response error.
#[derive(Clone, Debug)]
pub struct ResponseError {
    description: String,
    error_code: Option<Integer>,
    migrate_to_chat_id: Option<Integer>,
    retry_after: Option<Integer>,
}

impl ResponseError {
    /// Returns a human-readable description of the error.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns an error code.
    pub fn error_code(&self) -> Option<Integer> {
        self.error_code
    }

    /// Returns a flag describing whether a request can be repeated.
    pub fn can_retry(&self) -> bool {
        self.retry_after.is_some()
    }

    /// Returns a number of seconds left to wait before the request can be repeated.
    pub fn retry_after(&self) -> Option<u64> {
        self.retry_after.and_then(|x| x.try_into().ok())
    }

    /// Returns a new identifier of a group which has been migrated to a supergroup.
    pub fn migrate_to_chat_id(&self) -> Option<Integer> {
        self.migrate_to_chat_id
    }
}

impl Error for ResponseError {}

impl fmt::Display for ResponseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "a telegram error has occurred: description={}", self.description)?;
        if let Some(code) = self.error_code {
            write!(out, "; error_code={code}")?;
        }
        if let Some(chat_id) = self.migrate_to_chat_id {
            write!(out, "; migrate_to_chat_id={chat_id}")?;
        }
        if let Some(retry_after) = self.retry_after {
            write!(out, "; retry_after={retry_after}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
struct RawResponse<T> {
    ok: bool,
    result: Option<T>,
    description: Option<String>,
    error_code: Option<Integer>,
    parameters: Option<RawResponseParameters>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct RawResponseParameters {
    migrate_to_chat_id: Option<Integer>,
    retry_after: Option<Integer>,
}

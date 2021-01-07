use crate::types::primitive::{False, Integer, True};
use serde::Deserialize;
use std::{error::Error as StdError, fmt};

/// API Response
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "RawResponse<T>")]
pub enum Response<T> {
    /// Success
    Success(T),
    /// Error
    Error(ResponseError),
}

impl<T> From<RawResponse<T>> for Response<T> {
    fn from(raw: RawResponse<T>) -> Self {
        match raw {
            RawResponse::Success { result, .. } => Response::Success(result),
            RawResponse::Error {
                description,
                error_code,
                parameters,
                ..
            } => Response::Error(ResponseError {
                description,
                error_code,
                migrate_to_chat_id: parameters.and_then(|x| x.migrate_to_chat_id),
                retry_after: parameters.and_then(|x| x.retry_after),
            }),
        }
    }
}

/// Response error
#[derive(Clone, Debug)]
pub struct ResponseError {
    description: String,
    error_code: Option<Integer>,
    migrate_to_chat_id: Option<Integer>,
    retry_after: Option<Integer>,
}

impl ResponseError {
    /// Human-readable description of the error
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Error code
    pub fn error_code(&self) -> Option<Integer> {
        self.error_code
    }

    /// Whether request can be repeated
    pub fn can_retry(&self) -> bool {
        self.retry_after.is_some()
    }

    /// Number of seconds left to wait before the request can be repeated
    pub fn retry_after(&self) -> Option<Integer> {
        self.retry_after
    }

    /// The group has been migrated to a supergroup with the specified identifier
    pub fn migrate_to_chat_id(&self) -> Option<Integer> {
        self.migrate_to_chat_id
    }
}

impl StdError for ResponseError {}

impl fmt::Display for ResponseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "a telegram error has occurred: description={}", self.description)?;
        if let Some(code) = self.error_code {
            write!(out, "; error_code={}", code)?;
        }
        if let Some(chat_id) = self.migrate_to_chat_id {
            write!(out, "; migrate_to_chat_id={}", chat_id)?;
        }
        if let Some(retry_after) = self.retry_after {
            write!(out, "; retry_after={}", retry_after)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum RawResponse<T> {
    Success {
        ok: True,
        result: T,
    },
    Error {
        ok: False,
        description: String,
        error_code: Option<Integer>,
        parameters: Option<RawResponseParameters>,
    },
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct RawResponseParameters {
    migrate_to_chat_id: Option<Integer>,
    retry_after: Option<Integer>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Clone, Debug, Deserialize)]
    struct Object {
        name: String,
    }

    #[test]
    fn deserialize() {
        let success: Response<Object> = serde_json::from_value(json!({
            "ok": true,
            "result": {"name": "test" }
        }))
        .unwrap();

        if let Response::Success(ref obj) = success {
            assert_eq!(obj.name, String::from("test"));
        } else {
            panic!("Unexpected response: {:?}", success);
        }

        let error: Response<Object> = serde_json::from_value(json!({
            "ok": false,
            "description": "test err",
            "error_code": 1,
            "parameters": {
                "migrate_to_chat_id": 2,
                "retry_after": 3
            }
        }))
        .unwrap();
        if let Response::Error(err) = error {
            assert_eq!(err.description(), "test err");
            assert_eq!(err.error_code(), Some(1));
            assert!(err.can_retry());
            assert_eq!(err.retry_after(), Some(3));
            assert_eq!(err.migrate_to_chat_id(), Some(2));
        } else {
            panic!("Unexpected response: {:?}", success);
        }

        let error: Response<Object> = serde_json::from_value(json!({
            "ok": false,
            "description": "test err"
        }))
        .unwrap();
        if let Response::Error(err) = error {
            assert_eq!(err.description(), "test err");
            assert!(!err.can_retry());
            assert!(err.retry_after().is_none());
            assert!(err.error_code().is_none());
            assert!(err.migrate_to_chat_id().is_none());
        } else {
            panic!("Unexpected response: {:?}", success);
        }
    }
}

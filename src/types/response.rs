use crate::types::primitive::Integer;
use serde::{de::Error, Deserialize, Deserializer};
use std::{error::Error as StdError, fmt};

/// API Response
#[derive(Clone, Debug)]
pub enum Response<T> {
    /// Success
    Success(T),
    /// Error
    Error(ResponseError),
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Response<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawResponse<T> = Deserialize::deserialize(deserializer)?;
        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        };
        Ok(if raw.ok {
            Response::Success(required!(result))
        } else {
            Response::Error(ResponseError {
                description: required!(description),
                error_code: raw.error_code,
                migrate_to_chat_id: raw.parameters.and_then(|x| x.migrate_to_chat_id),
                retry_after: raw.parameters.and_then(|x| x.retry_after),
            })
        })
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
struct RawResponse<T> {
    ok: bool,
    description: Option<String>,
    error_code: Option<Integer>,
    result: Option<T>,
    parameters: Option<RawResponseParameters>,
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

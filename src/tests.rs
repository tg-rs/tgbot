use crate::{
    form::Form,
    method::Method,
    request::{RequestBody, RequestMethod},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Debug;

pub(crate) fn assert_json_eq<S>(expected_struct: S, expected_value: JsonValue)
where
    S: Clone + Debug + DeserializeOwned + PartialEq + Serialize,
{
    let actual_struct = serde_json::from_value(expected_value.clone()).unwrap();
    let actual_value = serde_json::to_value(&expected_struct).unwrap();
    assert_eq!(expected_struct, actual_struct);
    assert_eq!(expected_value, actual_value);
}

#[derive(Debug)]
enum ExpectedRequestBody {
    Json(JsonValue),
    Form(Form),
    Empty,
}

#[derive(Debug)]
pub(crate) struct ExpectedRequest {
    path: String,
    method: RequestMethod,
    body: ExpectedRequestBody,
}

impl ExpectedRequest {
    pub(crate) fn get<P>(path: P) -> Self
    where
        P: Into<String>,
    {
        Self {
            path: path.into(),
            method: RequestMethod::Get,
            body: ExpectedRequestBody::Empty,
        }
    }
    pub(crate) fn post_json<P>(path: P, body: JsonValue) -> Self
    where
        P: Into<String>,
    {
        Self {
            path: path.into(),
            method: RequestMethod::Post,
            body: ExpectedRequestBody::Json(body),
        }
    }

    pub(crate) fn post_form<P>(path: P, body: Form) -> Self
    where
        P: Into<String>,
    {
        Self {
            path: path.into(),
            method: RequestMethod::Post,
            body: ExpectedRequestBody::Form(body),
        }
    }
}

pub(crate) fn assert_request_eq<A>(expected_request: ExpectedRequest, actual_method: A)
where
    A: Method,
{
    let request = actual_method.into_request();
    let actual_url = request.build_url("base-url", "-token");
    let expected_url = format!("base-url/bot-token/{}", expected_request.path);
    assert_eq!(expected_url, actual_url);
    assert_eq!(expected_request.method, request.get_method());
    match (expected_request.body, request.into_body()) {
        (ExpectedRequestBody::Json(expected_body), RequestBody::Json(result)) => {
            let actual_body_raw = result.unwrap();
            let actual_body: JsonValue = serde_json::from_str(&actual_body_raw).unwrap();
            assert_eq!(expected_body, actual_body);
        }
        (ExpectedRequestBody::Form(expected_form), RequestBody::Form(actual_form)) => {
            assert_eq!(expected_form, actual_form);
        }
        (expected_body, actual_body) => {
            assert!(matches!(expected_body, ExpectedRequestBody::Empty));
            assert!(matches!(actual_body, RequestBody::Empty));
        }
    }
}

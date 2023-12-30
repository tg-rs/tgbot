use pretty_assertions::assert_eq;
use serde_json::Value as JsonValue;

use super::{Payload, PayloadData};
use crate::api::Method;

pub(crate) fn assert_payload_eq<A>(expected_payload: Payload, actual_method: A)
where
    A: Method,
{
    let payload = actual_method.into_payload();
    let actual_url = payload.build_url("base-url", "-token");
    let expected_url = format!("base-url/bot-token/{}", expected_payload.url_path);
    assert_eq!(expected_url, actual_url);
    assert_eq!(expected_payload.http_method, payload.http_method);
    match (expected_payload.payload_data, payload.payload_data) {
        (PayloadData::Json(expected_result), PayloadData::Json(actual_result)) => {
            let expected_data_raw = expected_result.unwrap();
            let expected_data: JsonValue = serde_json::from_str(&expected_data_raw).unwrap();
            let actual_data_raw = actual_result.unwrap();
            let actual_data: JsonValue = serde_json::from_str(&actual_data_raw).unwrap();
            assert_eq!(expected_data, actual_data);
        }
        (PayloadData::Form(expected_form), PayloadData::Form(actual_form)) => {
            assert_eq!(expected_form, actual_form);
        }
        (expected_data, actual_body) => {
            assert!(matches!(expected_data, PayloadData::Empty));
            assert!(matches!(actual_body, PayloadData::Empty));
        }
    }
}

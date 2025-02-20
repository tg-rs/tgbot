use std::fmt::Debug;

use pretty_assertions::assert_eq;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value as JsonValue;

pub(crate) fn assert_json_eq<S>(expected_struct: S, expected_value: JsonValue)
where
    S: Clone + Debug + DeserializeOwned + PartialEq + Serialize,
{
    let actual_struct = serde_json::from_value(expected_value.clone()).unwrap();
    let actual_value = serde_json::to_value(&expected_struct).unwrap();
    assert_eq!(expected_struct, actual_struct);
    assert_eq!(expected_value, actual_value);
}

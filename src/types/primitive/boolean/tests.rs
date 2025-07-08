use super::{False, True};
use crate::types::tests::assert_json_eq;

#[test]
fn ok() {
    assert_json_eq(True, serde_json::json!(true));
    assert_json_eq(False, serde_json::json!(false));
}

#[test]
fn err() {
    let err = serde_json::from_value::<True>(serde_json::json!(false))
        .unwrap_err()
        .to_string();
    assert_eq!(err, "invalid value: boolean `false`, expected true");
    let err = serde_json::from_value::<False>(serde_json::json!(true))
        .unwrap_err()
        .to_string();
    assert_eq!(err, "invalid value: boolean `true`, expected false");
}

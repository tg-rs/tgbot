use serde::Deserialize;

use crate::types::Response;

#[derive(Clone, Debug, Deserialize)]
struct Object {
    name: String,
}

#[test]
fn deserialize() {
    let success: Response<Object> = serde_json::from_value(serde_json::json!({
        "ok": true,
        "result": {"name": "test" }
    }))
    .unwrap();

    if let Response::Success(ref obj) = success {
        assert_eq!(obj.name, String::from("test"));
    } else {
        panic!("Unexpected response: {success:?}");
    }

    let error: Response<Object> = serde_json::from_value(serde_json::json!({
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
        panic!("Unexpected response: {success:?}");
    }

    let error: Response<Object> = serde_json::from_value(serde_json::json!({
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
        panic!("Unexpected response: {success:?}");
    }
}

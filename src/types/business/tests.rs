use crate::types::{tests::assert_json_eq, BusinessConnection, User};

#[test]
fn business_connection() {
    let expected_struct = BusinessConnection::new(0, "id", User::new(1, "test", false), 2);
    let mut expected_value = serde_json::json!({
        "can_reply": false,
        "date": 0,
        "id": "id",
        "is_enabled": false,
        "user": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "user_chat_id": 2,
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    let expected_struct = expected_struct.with_can_reply(true).with_is_enabled(true);
    expected_value["can_reply"] = serde_json::json!(true);
    expected_value["is_enabled"] = serde_json::json!(true);
    assert_json_eq(expected_struct, expected_value)
}
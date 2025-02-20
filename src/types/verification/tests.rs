use crate::{
    api::{Payload, assert_payload_eq},
    types::{RemoveChatVerification, RemoveUserVerification, VerifyChat, VerifyUser},
};

#[test]
fn remove_chat_verification() {
    assert_payload_eq(
        Payload::json(
            "removeChatVerification",
            serde_json::json!({
                "chat_id": 1,
            }),
        ),
        RemoveChatVerification::new(1),
    );
}

#[test]
fn remove_user_verification() {
    assert_payload_eq(
        Payload::json(
            "removeUserVerification",
            serde_json::json!({
                "user_id": 1,
            }),
        ),
        RemoveUserVerification::new(1),
    );
}

#[test]
fn verify_chat() {
    let method = VerifyChat::new(1);
    assert_payload_eq(
        Payload::json(
            "verifyChat",
            serde_json::json!({
                "chat_id": 1,
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "verifyChat",
            serde_json::json!({
                "chat_id": 1,
                "custom_description": "test",
            }),
        ),
        method.with_custom_description("test").clone(),
    );
}

#[test]
fn verify_user() {
    let method = VerifyUser::new(1);
    assert_payload_eq(
        Payload::json(
            "verifyUser",
            serde_json::json!({
                "user_id": 1,
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "verifyUser",
            serde_json::json!({
                "user_id": 1,
                "custom_description": "test",
            }),
        ),
        method.with_custom_description("test").clone(),
    );
}

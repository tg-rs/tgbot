use crate::{
    tests::{assert_request_eq, ExpectedRequest},
    types::{PinChatMessage, UnpinAllChatMessages, UnpinChatMessage},
};

#[test]
fn pin_chat_message() {
    let method = PinChatMessage::new(1, 2);
    assert_request_eq(
        ExpectedRequest::post_json(
            "pinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2
            }),
        ),
        method.clone(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "pinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "disable_notification": true
            }),
        ),
        method.disable_notification(true),
    );
}

#[test]
fn unpin_chat_message() {
    let method = UnpinChatMessage::new(1);
    assert_request_eq(
        ExpectedRequest::post_json(
            "unpinChatMessage",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "unpinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2
            }),
        ),
        method.message_id(2),
    );
}

#[test]
fn unpin_all_chat_messages() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "unpinAllChatMessages",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        UnpinAllChatMessages::new(1),
    );
}

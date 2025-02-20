use crate::{
    api::{Payload, assert_payload_eq},
    types::{PinChatMessage, UnpinAllChatMessages, UnpinChatMessage},
};

#[test]
fn pin_chat_message() {
    let method = PinChatMessage::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "pinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "pinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "business_connection_id": "c-id",
                "disable_notification": true
            }),
        ),
        method
            .with_business_connection_id("c-id")
            .with_disable_notification(true),
    );
}

#[test]
fn unpin_chat_message() {
    let method = UnpinChatMessage::new(1);
    assert_payload_eq(
        Payload::json(
            "unpinChatMessage",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "unpinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "business_connection_id": "c-id",
                "message_id": 2
            }),
        ),
        method.with_business_connection_id("c-id").with_message_id(2),
    );
}

#[test]
fn unpin_all_chat_messages() {
    assert_payload_eq(
        Payload::json(
            "unpinAllChatMessages",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        UnpinAllChatMessages::new(1),
    );
}

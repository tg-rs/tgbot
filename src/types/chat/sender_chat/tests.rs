use crate::{
    api::{assert_payload_eq, Payload},
    types::{BanChatSenderChat, UnbanChatSenderChat},
};

#[test]
fn ban_chat_sender_chat() {
    assert_payload_eq(
        Payload::json(
            "banChatSenderChat",
            serde_json::json!({
                "chat_id": 1,
                "sender_chat_id": 1,
            }),
        ),
        BanChatSenderChat::new(1, 1),
    );
}

#[test]
fn unban_chat_sender_chat() {
    assert_payload_eq(
        Payload::json(
            "unbanChatSenderChat",
            serde_json::json!({
                "chat_id": 1,
                "sender_chat_id": 1,
            }),
        ),
        UnbanChatSenderChat::new(1, 1),
    );
}

use crate::{
    tests::{assert_request_eq, ExpectedRequest},
    types::{BanChatSenderChat, UnbanChatSenderChat},
};

#[test]
fn ban_chat_sender_chat() {
    assert_request_eq(
        ExpectedRequest::post_json(
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
    assert_request_eq(
        ExpectedRequest::post_json(
            "unbanChatSenderChat",
            serde_json::json!({
                "chat_id": 1,
                "sender_chat_id": 1,
            }),
        ),
        UnbanChatSenderChat::new(1, 1),
    );
}

use crate::{
    api::{assert_payload_eq, Payload},
    types::{GetChat, LeaveChat, SetChatDescription, SetChatTitle},
};

#[test]
fn get_chat() {
    assert_payload_eq(
        Payload::json(
            "getChat",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChat::new(1),
    );
}

#[test]
fn leave_chat() {
    assert_payload_eq(
        Payload::json(
            "leaveChat",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        LeaveChat::new(1),
    );
}

#[test]
fn set_chat_description() {
    let method = SetChatDescription::new(1);
    assert_payload_eq(
        Payload::json(
            "setChatDescription",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setChatDescription",
            serde_json::json!({
                "chat_id": 1,
                "description": "Description"
            }),
        ),
        method.with_description("Description"),
    );
}

#[test]
fn set_chat_title() {
    assert_payload_eq(
        Payload::json(
            "setChatTitle",
            serde_json::json!({
                "chat_id": 1,
                "title": "Chat"
            }),
        ),
        SetChatTitle::new(1, "Chat"),
    );
}

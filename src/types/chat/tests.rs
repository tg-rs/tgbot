use crate::{
    tests::{assert_request_eq, ExpectedRequest},
    types::{GetChat, LeaveChat, SetChatDescription, SetChatTitle},
};

#[test]
fn get_chat() {
    assert_request_eq(
        ExpectedRequest::post_json(
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
    assert_request_eq(
        ExpectedRequest::post_json(
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
    assert_request_eq(
        ExpectedRequest::post_json(
            "setChatDescription",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "setChatDescription",
            serde_json::json!({
                "chat_id": 1,
                "description": "Description"
            }),
        ),
        method.description("Description"),
    );
}

#[test]
fn set_chat_title() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "setChatTitle",
            serde_json::json!({
                "chat_id": 1,
                "title": "Chat"
            }),
        ),
        SetChatTitle::new(1, "Chat"),
    );
}

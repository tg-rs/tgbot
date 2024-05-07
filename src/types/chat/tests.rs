use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        BackgroundType,
        ChatBackground,
        Document,
        GetChat,
        LeaveChat,
        SetChatDescription,
        SetChatTitle,
    },
};

#[test]
fn chat_background() {
    assert_json_eq(
        ChatBackground::from(BackgroundType::Wallpaper {
            dark_theme_dimming: 100,
            document: Document::new("file-id", "file-unique-id"),
            is_blurred: Some(true),
            is_moving: Some(false),
        }),
        serde_json::json!({
            "type": {
                "type": "wallpaper",
                "dark_theme_dimming": 100,
                "document": {
                    "file_id": "file-id",
                    "file_unique_id": "file-unique-id",
                },
                "is_blurred": true,
                "is_moving": false,
            }
        }),
    );
}

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

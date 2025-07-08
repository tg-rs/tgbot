use crate::{
    api::{Payload, assert_payload_eq},
    types::*,
};

#[test]
fn menu_button() {
    insta::assert_json_snapshot!(MenuButton::Commands);
    insta::assert_json_snapshot!(MenuButton::Default);
    insta::assert_json_snapshot!(MenuButton::WebApp(MenuButtonWebApp::new(
        "button-text",
        WebAppInfo::from("https://example.com"),
    )));
}

#[test]
fn get_chat_menu_button() {
    assert_payload_eq(
        Payload::json("getChatMenuButton", serde_json::json!({})),
        GetChatMenuButton::default(),
    );
    assert_payload_eq(
        Payload::json(
            "getChatMenuButton",
            serde_json::json!({
                "chat_id": 1,
            }),
        ),
        GetChatMenuButton::default().with_chat_id(1),
    );
}

#[test]
fn set_chat_menu_button() {
    assert_payload_eq(
        Payload::json("setChatMenuButton", serde_json::json!({})),
        SetChatMenuButton::default(),
    );
    assert_payload_eq(
        Payload::json(
            "setChatMenuButton",
            serde_json::json!({
                "chat_id": 1,
            }),
        ),
        SetChatMenuButton::default().chat_id(1),
    );
    assert_payload_eq(
        Payload::json(
            "setChatMenuButton",
            serde_json::json!({
                "menu_button": {
                    "type": "default"
                },
            }),
        ),
        SetChatMenuButton::default().menu_button(MenuButton::Default),
    );
    assert_payload_eq(
        Payload::json(
            "setChatMenuButton",
            serde_json::json!({
                "chat_id": 1,
                "menu_button": {
                    "type": "commands"
                },
            }),
        ),
        SetChatMenuButton::default()
            .chat_id(1)
            .menu_button(MenuButton::Commands),
    );
}

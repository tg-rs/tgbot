use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, GetChatMenuButton, MenuButton, MenuButtonWebApp, SetChatMenuButton, WebAppInfo},
};

#[test]
fn menu_button() {
    assert_json_eq(
        MenuButton::Commands,
        serde_json::json!({
            "type": "commands"
        }),
    );
    assert_json_eq(
        MenuButton::Default,
        serde_json::json!({
            "type": "default"
        }),
    );
    assert_json_eq(
        MenuButton::WebApp(MenuButtonWebApp::new(
            "button-text",
            WebAppInfo::from("https://example.com"),
        )),
        serde_json::json!({
            "type": "web_app",
            "text": "button-text",
            "web_app": {
                "url": "https://example.com"
            }
        }),
    );
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

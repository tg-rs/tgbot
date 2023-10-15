use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        GetChatMenuButton,
        MenuButton,
        MenuButtonCommands,
        MenuButtonDefault,
        MenuButtonWebApp,
        SetChatMenuButton,
        WebAppInfo,
    },
};

#[test]
fn menu_button() {
    assert_json_eq(
        MenuButton::Commands(MenuButtonCommands {}),
        serde_json::json!({
            "type": "commands"
        }),
    );
    assert_json_eq(
        MenuButton::Default(MenuButtonDefault {}),
        serde_json::json!({
            "type": "default"
        }),
    );
    assert_json_eq(
        MenuButton::WebApp(MenuButtonWebApp {
            text: String::from("button-text"),
            web_app: WebAppInfo {
                url: String::from("https://example.com"),
            },
        }),
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
        GetChatMenuButton::default().chat_id(1),
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
        SetChatMenuButton::default().menu_button(MenuButton::default()),
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
            .menu_button(MenuButton::commands()),
    );
}

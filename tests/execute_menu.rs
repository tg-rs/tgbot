#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("get-chat-menu-button-commands", GetChatMenuButton::default(), |x| {
            assert!(matches!(x, MenuButton::Commands))
        }),
        ("get-chat-menu-button-default", GetChatMenuButton::default(), |x| {
            assert!(matches!(x, MenuButton::Default))
        }),
        (
            "get-chat-menu-button-web-app",
            GetChatMenuButton::default().with_chat_id(1),
            |x| {
                if let MenuButton::WebApp(data) = x {
                    assert_eq!(data.text, "test");
                    assert_eq!(data.web_app.url, "https://example.com");
                } else {
                    panic!("Got an unexpected menu button: {x:?}");
                }
            },
        ),
    ])
    .await;
    cx.execute_batch([
        ("set-chat-menu-button-base", SetChatMenuButton::default(), |x| {
            assert!(x)
        }),
        (
            "set-chat-menu-button-commands",
            SetChatMenuButton::default()
                .with_chat_id(1)
                .with_menu_button(MenuButton::Commands),
            |x| assert!(x),
        ),
        (
            "set-chat-menu-button-default",
            SetChatMenuButton::default()
                .with_chat_id(1)
                .with_menu_button(MenuButton::Default),
            |x| assert!(x),
        ),
        (
            "set-chat-menu-button-web-app",
            SetChatMenuButton::default()
                .with_chat_id(1)
                .with_menu_button(MenuButton::WebApp(MenuButtonWebApp::new(
                    "test",
                    WebAppInfo::from("https://example.com"),
                ))),
            |x| assert!(x),
        ),
    ])
    .await;
}

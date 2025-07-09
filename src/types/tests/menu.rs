use crate::types::*;

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
    let method = GetChatMenuButton::default();
    assert_payload_eq!(POST JSON "getChatMenuButton" => method);
    let method = GetChatMenuButton::default().with_chat_id(1);
    assert_payload_eq!(POST JSON "getChatMenuButton" => method);
}

#[test]
fn set_chat_menu_button() {
    let method = SetChatMenuButton::default();
    assert_payload_eq!(POST JSON "setChatMenuButton" => method);
    let method = SetChatMenuButton::default().chat_id(1);
    assert_payload_eq!(POST JSON "setChatMenuButton" => method);
    let method = SetChatMenuButton::default().menu_button(MenuButton::Default);
    assert_payload_eq!(POST JSON "setChatMenuButton" => method);
    let method = SetChatMenuButton::default()
        .chat_id(1)
        .menu_button(MenuButton::Commands);
    assert_payload_eq!(POST JSON "setChatMenuButton" => method);
}

use crate::types::{tests::assert_json_eq, WebAppData, WebAppInfo};

#[test]
fn web_app_info() {
    assert_json_eq(
        WebAppInfo {
            url: String::from("https://example.com"),
        },
        serde_json::json!({
            "url": "https://example.com"
        }),
    );
}

#[test]
fn web_app_data() {
    assert_json_eq(
        WebAppData {
            data: String::from("web-app-data"),
            button_text: String::from("web-app-button-text"),
        },
        serde_json::json!({
            "data": "web-app-data",
            "button_text": "web-app-button-text"
        }),
    );
}

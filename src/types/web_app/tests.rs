use crate::types::{WebAppData, WebAppInfo, tests::assert_json_eq};

#[test]
fn web_app_info() {
    assert_json_eq(
        WebAppInfo::from("https://example.com"),
        serde_json::json!({
            "url": "https://example.com"
        }),
    );
}

#[test]
fn web_app_data() {
    assert_json_eq(
        WebAppData::new("web-app-data", "web-app-button-text"),
        serde_json::json!({
            "data": "web-app-data",
            "button_text": "web-app-button-text"
        }),
    );
}

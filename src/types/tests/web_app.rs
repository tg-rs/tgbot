use crate::types::*;

#[test]
fn web_app_info() {
    insta::assert_json_snapshot!(WebAppInfo::from("https://example.com"));
}

#[test]
fn web_app_data() {
    insta::assert_json_snapshot!(WebAppData::new("web-app-data", "web-app-button-text"));
}

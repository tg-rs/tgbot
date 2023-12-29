use crate::types::{tests::assert_json_eq, LinkPreviewOptions};

#[test]
fn link_preview_options() {
    assert_json_eq(LinkPreviewOptions::default(), serde_json::json!({}));
    assert_json_eq(
        LinkPreviewOptions::default()
            .with_is_disabled(false)
            .with_prefer_large_media(true)
            .with_prefer_small_media(false)
            .with_show_above_text(true)
            .with_url("https://example.com"),
        serde_json::json!({
            "is_disabled": false,
            "prefer_large_media": true,
            "prefer_small_media": false,
            "show_above_text": true,
            "url": "https://example.com"
        }),
    );
}

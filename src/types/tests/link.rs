use crate::types::*;

#[test]
fn link_preview_options() {
    insta::assert_json_snapshot!(LinkPreviewOptions::default());
    insta::assert_json_snapshot!(
        LinkPreviewOptions::default()
            .with_is_disabled(false)
            .with_prefer_large_media(true)
            .with_prefer_small_media(false)
            .with_show_above_text(true)
            .with_url("https://example.com"),
    );
}

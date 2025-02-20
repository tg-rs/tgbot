use crate::types::{ParseMode, tests::assert_json_eq};

#[test]
fn parse_mode() {
    for (expected_struct, expected_value) in [
        (ParseMode::Html, serde_json::json!("HTML")),
        (ParseMode::Markdown, serde_json::json!("Markdown")),
        (ParseMode::MarkdownV2, serde_json::json!("MarkdownV2")),
    ] {
        assert_json_eq(expected_struct, expected_value);
    }
}

#[test]
fn to_string() {
    assert_eq!(ParseMode::Html.to_string(), "HTML");
    assert_eq!(ParseMode::Markdown.to_string(), "Markdown");
    assert_eq!(ParseMode::MarkdownV2.to_string(), "MarkdownV2");
}

#[test]
fn parse_mode_escape() {
    assert_eq!(ParseMode::Html.escape("<>&"), "&lt;&gt;&amp;");
    assert_eq!(ParseMode::Markdown.escape(r#"_*`["#), r"\_\*\`\[");
    assert_eq!(
        ParseMode::MarkdownV2.escape(r#"_*[]()~`>#+-=|{}.!"#),
        r"\_\*\[\]\(\)\~\`\>\#\+\-\=\|\{\}\.\!"
    );
}

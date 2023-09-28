use crate::types::ParseMode;

#[test]
fn serialize() {
    assert_eq!(serde_json::to_string(&ParseMode::Html).unwrap(), r#""HTML""#);
    assert_eq!(serde_json::to_string(&ParseMode::Markdown).unwrap(), r#""Markdown""#);
    assert_eq!(
        serde_json::to_string(&ParseMode::MarkdownV2).unwrap(),
        r#""MarkdownV2""#
    );
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
    assert_eq!(ParseMode::Markdown.escape(r#"_*`["#), r#"\_\*\`\["#);
    assert_eq!(
        ParseMode::MarkdownV2.escape(r#"_*[]()~`>#+-=|{}.!"#),
        r#"\_\*\[\]\(\)\~\`\>\#\+\-\=\|\{\}\.\!"#
    );
}

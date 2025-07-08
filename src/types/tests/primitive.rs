use crate::types::*;

#[test]
fn ok() {
    insta::assert_json_snapshot!(True);
    insta::assert_json_snapshot!(False);
}

#[test]
fn err() {
    let err = serde_json::from_value::<True>(serde_json::json!(false))
        .unwrap_err()
        .to_string();
    assert_eq!(err, "invalid value: boolean `false`, expected true");
    let err = serde_json::from_value::<False>(serde_json::json!(true))
        .unwrap_err()
        .to_string();
    assert_eq!(err, "invalid value: boolean `true`, expected false");
}

#[test]
fn parse_mode() {
    for value in [ParseMode::Html, ParseMode::Markdown, ParseMode::MarkdownV2] {
        insta::assert_json_snapshot!(value);
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

#[test]
fn star_amount() {
    let value = StarAmount::from(1);
    assert_eq!(value.amount, 1);
    assert!(value.nanostar_amount.is_none());
    let value = value.with_nanostar_amount(1);
    assert_eq!(value.nanostar_amount, Some(1));
}

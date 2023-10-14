use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedVoice,
    InlineQueryResultVoice,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_voice() {
    let result = InlineQueryResultVoice::new("voice-id", "voice-url", "voice-title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .caption("voice-caption")
                .parse_mode(ParseMode::Markdown)
                .voice_duration(100)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url(
                    "voice-kb-text",
                    "voice-kb-url",
                )]])
                .input_message_content(InputMessageContentText::new("voice-content-text")),
        ),
        serde_json::json!({
            "type": "voice",
            "id": "voice-id",
            "voice_url": "voice-url",
            "title": "voice-title",
            "caption": "voice-caption",
            "parse_mode": "Markdown",
            "voice_duration": 100,
            "reply_markup": {"inline_keyboard": [[{"text": "voice-kb-text", "url": "voice-kb-url"}]]},
            "input_message_content": {"message_text": "voice-content-text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "voice",
            "id": "voice-id",
            "voice_url": "voice-url",
            "title": "voice-title"
        }),
    );
}

#[test]
fn inline_query_result_cached_voice() {
    let result = InlineQueryResultCachedVoice::new("id", "file-id", "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "voice",
            "id": "id",
            "voice_file_id": "file-id",
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "voice",
            "id": "id",
            "voice_file_id": "file-id",
            "title": "title"
        }),
    );
}

use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultAudio,
    InlineQueryResultCachedAudio,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_audio() {
    let result = InlineQueryResultAudio::new("id", "url", "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .caption("caption")
                .parse_mode(ParseMode::Html)
                .performer("performer")
                .audio_duration(100)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "audio",
            "id": "id",
            "audio_url": "url",
            "title": "title",
            "caption": "caption",
            "parse_mode": "HTML",
            "performer": "performer",
            "audio_duration": 100,
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "audio",
            "id": "id",
            "audio_url": "url",
            "title": "title"
        }),
    );
}

#[test]
fn inline_query_result_cached_audio() {
    let result = InlineQueryResultCachedAudio::new("id", "file-id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .caption("test")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "audio",
            "id": "id",
            "audio_file_id": "file-id",
            "caption": "test",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "audio",
            "id": "id",
            "audio_file_id": "file-id"
        }),
    );
}

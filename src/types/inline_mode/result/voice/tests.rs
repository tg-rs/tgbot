use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedVoice,
    InlineQueryResultVoice,
    InputMessageContentText,
    ParseMode,
    tests::assert_json_eq,
};

#[test]
fn inline_query_result_voice() {
    let result = InlineQueryResultVoice::new("voice-id", "voice-title", "voice-url");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("voice-caption")
                .with_input_message_content(InputMessageContentText::new("voice-content-text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_reply_markup([[InlineKeyboardButton::for_url("voice-kb-text", "voice-kb-url")]])
                .with_voice_duration(100),
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
    let result = InlineQueryResultCachedVoice::new("id", "title", "file-id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("caption")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
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

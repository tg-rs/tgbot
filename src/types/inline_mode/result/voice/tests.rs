use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedVoice,
    InlineQueryResultVoice,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_voice_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultVoice::new("id", "url", "title")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .voice_duration(100)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
        serde_json::json!({
            "type": "voice",
            "id": "id",
            "voice_url": "url",
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "voice_duration": 100,
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        })
    );
}

#[test]
fn inline_query_result_voice_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultVoice::new(
            "id", "url", "title",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "voice",
            "id": "id",
            "voice_url": "url",
            "title": "title"
        })
    );
}

#[test]
fn inline_query_result_cached_voice_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultCachedVoice::new("id", "file-id", "title")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
        serde_json::json!({
            "type": "voice",
            "id": "id",
            "voice_file_id": "file-id",
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        })
    );
}

#[test]
fn inline_query_result_cached_voice_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedVoice::new(
            "id", "file-id", "title",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "voice",
            "id": "id",
            "voice_file_id": "file-id",
            "title": "title"
        })
    );
}

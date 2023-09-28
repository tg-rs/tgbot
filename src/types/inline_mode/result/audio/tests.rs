use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultAudio,
    InlineQueryResultCachedAudio,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn serialize_inline_query_result_audio_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultAudio::new("id", "url", "title")
                .caption("caption")
                .parse_mode(ParseMode::Html)
                .performer("performer")
                .audio_duration(100)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
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
        })
    );
}

#[test]
fn serialize_inline_query_result_audio_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultAudio::new(
            "id", "url", "title",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "audio",
            "id": "id",
            "audio_url": "url",
            "title": "title"
        })
    );
}

#[test]
fn inline_query_result_cached_audio_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultCachedAudio::new("id", "file-id")
                .caption("test")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
        serde_json::json!({
            "type": "audio",
            "id": "id",
            "audio_file_id": "file-id",
            "caption": "test",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        })
    );
}

#[test]
fn inline_query_result_cached_audio_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedAudio::new(
            "id", "file-id",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "audio",
            "id": "id",
            "audio_file_id": "file-id"
        })
    );
}

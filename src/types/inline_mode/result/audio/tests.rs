use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultAudio,
    InlineQueryResultCachedAudio,
    InputMessageContentText,
    ParseMode,
    tests::assert_json_eq,
};

#[test]
fn inline_query_result_audio() {
    let result = InlineQueryResultAudio::new("url", "id", "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_audio_duration(100)
                .with_caption("caption")
                .with_caption_parse_mode(ParseMode::Html)
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_performer("performer")
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
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
    let result = InlineQueryResultCachedAudio::new("file-id", "id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("test")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
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

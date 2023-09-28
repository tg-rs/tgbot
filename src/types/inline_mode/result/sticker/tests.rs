use crate::types::{InlineKeyboardButton, InlineQueryResult, InlineQueryResultCachedSticker, InputMessageContentText};

#[test]
fn inline_query_result_cached_sticker_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultCachedSticker::new("id", "file-id")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
        serde_json::json!({
            "type": "sticker",
            "id": "id",
            "sticker_file_id": "file-id",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        })
    );
}

#[test]
fn inline_query_result_cached_sticker_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedSticker::new(
            "id", "file-id",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "sticker",
            "id": "id",
            "sticker_file_id": "file-id"
        })
    );
}

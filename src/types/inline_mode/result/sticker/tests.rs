use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedSticker,
    InputMessageContentText,
};

#[test]
fn inline_query_result_cached_sticker() {
    let result = InlineQueryResultCachedSticker::new("id", "file-id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "sticker",
            "id": "id",
            "sticker_file_id": "file-id",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "sticker",
            "id": "id",
            "sticker_file_id": "file-id"
        }),
    );
}

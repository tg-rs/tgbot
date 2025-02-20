use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedSticker,
    InputMessageContentText,
    tests::assert_json_eq,
};

#[test]
fn inline_query_result_cached_sticker() {
    let result = InlineQueryResultCachedSticker::new("id", "file-id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
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

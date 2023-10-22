use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedPhoto,
    InlineQueryResultPhoto,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_photo() {
    let result = InlineQueryResultPhoto::new("id", "url", "thumb-url");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .photo_width(200)
                .photo_height(300)
                .title("title")
                .description("desc")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "photo",
            "id": "id",
            "photo_url": "url",
            "thumbnail_url": "thumb-url",
            "photo_width": 200,
            "photo_height": 300,
            "title": "title",
            "description": "desc",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "photo",
            "id": "id",
            "photo_url": "url",
            "thumbnail_url": "thumb-url"
        }),
    );
}

#[test]
fn inline_query_result_cached_photo() {
    let result = InlineQueryResultCachedPhoto::new("id", "file-id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .title("title")
                .description("desc")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "photo",
            "id": "id",
            "photo_file_id": "file-id",
            "title": "title",
            "description": "desc",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "photo",
            "id": "id",
            "photo_file_id": "file-id"
        }),
    );
}

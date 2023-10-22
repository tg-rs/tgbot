use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedDocument,
    InlineQueryResultDocument,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_document() {
    let result = InlineQueryResultDocument::new("id", "title", "url", "mime");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .description("desc")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
                .thumbnail_url("thumb-url")
                .thumbnail_width(200)
                .thumbnail_height(200),
        ),
        serde_json::json!({
            "type": "document",
            "id": "id",
            "title": "title",
            "document_url": "url",
            "mime_type": "mime",
            "caption": "caption",
            "parse_mode": "Markdown",
            "description": "desc",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"},
            "thumbnail_url": "thumb-url",
            "thumbnail_width": 200,
            "thumbnail_height": 200
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "document",
            "id": "id",
            "title": "title",
            "document_url": "url",
            "mime_type": "mime"
        }),
    );
}

#[test]
fn inline_query_result_cached_document() {
    let result = InlineQueryResultCachedDocument::new("id", "title", "file-id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .description("desc")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "document",
            "id": "id",
            "title": "title",
            "document_file_id": "file-id",
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
            "type": "document",
            "id": "id",
            "title": "title",
            "document_file_id": "file-id"
        }),
    );
}

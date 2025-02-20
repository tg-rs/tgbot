use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedDocument,
    InlineQueryResultDocument,
    InputMessageContentText,
    ParseMode,
    tests::assert_json_eq,
};

#[test]
fn inline_query_result_document() {
    let result = InlineQueryResultDocument::new("url", "id", "mime", "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("caption")
                .with_description("desc")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_thumbnail_height(200)
                .with_thumbnail_url("thumb-url")
                .with_thumbnail_width(200),
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
    let result = InlineQueryResultCachedDocument::new("file-id", "id", "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("caption")
                .with_description("desc")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
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

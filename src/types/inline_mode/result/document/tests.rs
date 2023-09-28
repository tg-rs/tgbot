use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedDocument,
    InlineQueryResultDocument,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_document_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultDocument::new("id", "title", "url", "mime")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .description("desc")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
                .thumb_url("thumb-url")
                .thumb_width(200)
                .thumb_height(200)
        ))
        .unwrap(),
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
            "thumb_url": "thumb-url",
            "thumb_width": 200,
            "thumb_height": 200
        })
    );
}

#[test]
fn inline_query_result_document_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultDocument::new(
            "id", "title", "url", "mime",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "document",
            "id": "id",
            "title": "title",
            "document_url": "url",
            "mime_type": "mime"
        })
    );
}

#[test]
fn inline_query_result_cached_document_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultCachedDocument::new("id", "title", "file-id")
                .description("desc")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
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
        })
    );
}

#[test]
fn inline_query_result_cached_document_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedDocument::new(
            "id", "title", "file-id",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "document",
            "id": "id",
            "title": "title",
            "document_file_id": "file-id"
        })
    );
}

use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedPhoto,
    InlineQueryResultPhoto,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_photo_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultPhoto::new("id", "url", "thumb-url")
                .photo_width(200)
                .photo_height(300)
                .title("title")
                .description("desc")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
        serde_json::json!({
            "type": "photo",
            "id": "id",
            "photo_url": "url",
            "thumb_url": "thumb-url",
            "photo_width": 200,
            "photo_height": 300,
            "title": "title",
            "description": "desc",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        })
    );
}

#[test]
fn inline_query_result_photo_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultPhoto::new(
            "id",
            "url",
            "thumb-url",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "photo",
            "id": "id",
            "photo_url": "url",
            "thumb_url": "thumb-url"
        })
    );
}

#[test]
fn inline_query_result_cached_photo_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultCachedPhoto::new("id", "file-id")
                .title("title")
                .description("desc")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
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
        })
    );
}

#[test]
fn inline_query_result_cached_photo_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedPhoto::new(
            "id", "file-id",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "photo",
            "id": "id",
            "photo_file_id": "file-id"
        })
    );
}

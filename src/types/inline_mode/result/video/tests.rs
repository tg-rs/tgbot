use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedVideo,
    InlineQueryResultVideo,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_video() {
    let result = InlineQueryResultVideo::new("id", "url", "mime", "thumb-url", "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .video_width(200)
                .video_height(300)
                .video_duration(400)
                .description("desc")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "video",
            "id": "id",
            "video_url": "url",
            "mime_type": "mime",
            "thumb_url": "thumb-url",
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "video_width": 200,
            "video_height": 300,
            "video_duration": 400,
            "description": "desc",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "video",
            "id": "id",
            "video_url": "url",
            "mime_type": "mime",
            "thumb_url": "thumb-url",
            "title": "title"
        }),
    );
}

#[test]
fn inline_query_result_cached_video() {
    let result = InlineQueryResultCachedVideo::new("id", "file-id", "title");
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
            "type": "video",
            "id": "id",
            "video_file_id": "file-id",
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
            "type": "video",
            "id": "id",
            "video_file_id": "file-id",
            "title": "title"
        }),
    );
}
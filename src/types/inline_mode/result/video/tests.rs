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
    let result = InlineQueryResultVideo::new("id", "mime", "thumb-url", "title", "url");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("caption")
                .with_description("desc")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_video_duration(400)
                .with_video_width(200)
                .with_video_height(300),
        ),
        serde_json::json!({
            "type": "video",
            "id": "id",
            "video_url": "url",
            "mime_type": "mime",
            "thumbnail_url": "thumb-url",
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
            "thumbnail_url": "thumb-url",
            "title": "title"
        }),
    );
}

#[test]
fn inline_query_result_cached_video() {
    let result = InlineQueryResultCachedVideo::new("id", "title", "file-id");
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

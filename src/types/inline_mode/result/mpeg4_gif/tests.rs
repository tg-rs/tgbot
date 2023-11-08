use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultMpeg4Gif,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_mpeg4_gif() {
    let result = InlineQueryResultMpeg4Gif::new("id", "url", "thumb-url");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("caption")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_mpeg4_width(200)
                .with_mpeg4_height(300)
                .with_mpeg4_duration(400)
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_thumbnail_mime_type("video/mp4")
                .with_title("title")
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
        ),
        serde_json::json!({
            "type": "mpeg4_gif",
            "id": "id",
            "mpeg4_url": "url",
            "thumbnail_url": "thumb-url",
            "thumbnail_mime_type": "video/mp4",
            "mpeg4_width": 200,
            "mpeg4_height": 300,
            "mpeg4_duration": 400,
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "mpeg4_gif",
            "id": "id",
            "mpeg4_url": "url",
            "thumbnail_url": "thumb-url"
        }),
    );
}

#[test]
fn inline_query_result_cached_mpeg4_gif() {
    let result = InlineQueryResultCachedMpeg4Gif::new("id", "file-id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("caption")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_title("title"),
        ),
        serde_json::json!({
            "type": "mpeg4_gif",
            "id": "id",
            "mpeg4_file_id": "file-id",
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "mpeg4_gif",
            "id": "id",
            "mpeg4_file_id": "file-id"
        }),
    );
}

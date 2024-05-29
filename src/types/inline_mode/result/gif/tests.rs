use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedGif,
    InlineQueryResultGif,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_gif() {
    let result = InlineQueryResultGif::new("url", "id", "thumb-url");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("caption")
                .with_gif_width(200)
                .with_gif_height(300)
                .with_gif_duration(400)
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_show_caption_above_media(true)
                .with_thumbnail_mime_type("video/mp4")
                .with_title("title"),
        ),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_url": "url",
            "thumbnail_url": "thumb-url",
            "thumbnail_mime_type": "video/mp4",
            "gif_width": 200,
            "gif_height": 300,
            "gif_duration": 400,
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "show_caption_above_media": true,
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_url": "url",
            "thumbnail_url": "thumb-url"
        }),
    );
}

#[test]
fn inline_query_result_cached_gif() {
    let result = InlineQueryResultCachedGif::new("file-id", "id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_caption("caption")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_caption_parse_mode(ParseMode::Markdown)
                .with_title("title")
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_show_caption_above_media(true),
        ),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_file_id": "file-id",
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "show_caption_above_media": true,
            "input_message_content": {"message_text": "text"}
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_file_id": "file-id"
        }),
    );
}

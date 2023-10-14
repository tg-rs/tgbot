use crate::{
    tests::assert_json_eq,
    types::{
        InlineKeyboardButton,
        InlineQueryResult,
        InlineQueryResultCachedGif,
        InlineQueryResultGif,
        InputMessageContentText,
        ParseMode,
    },
};

#[test]
fn inline_query_result_gif() {
    let result = InlineQueryResultGif::new("id", "url", "thumb-url");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .thumb_mime_type("video/mp4")
                .gif_width(200)
                .gif_height(300)
                .gif_duration(400)
                .title("title")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_url": "url",
            "thumb_url": "thumb-url",
            "thumb_mime_type": "video/mp4",
            "gif_width": 200,
            "gif_height": 300,
            "gif_duration": 400,
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
            "type": "gif",
            "id": "id",
            "gif_url": "url",
            "thumb_url": "thumb-url"
        }),
    );
}

#[test]
fn inline_query_result_cached_gif() {
    let result = InlineQueryResultCachedGif::new("id", "file-id");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .title("title")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text")),
        ),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_file_id": "file-id",
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
            "type": "gif",
            "id": "id",
            "gif_file_id": "file-id"
        }),
    );
}

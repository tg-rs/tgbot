use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedGif,
    InlineQueryResultGif,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_gif_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultGif::new("id", "url", "thumb-url")
                .thumb_mime_type("video/mp4")
                .gif_width(200)
                .gif_height(300)
                .gif_duration(400)
                .title("title")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
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
        })
    );
}

#[test]
fn inline_query_result_gif_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultGif::new(
            "id",
            "url",
            "thumb-url",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_url": "url",
            "thumb_url": "thumb-url"
        })
    );
}

#[test]
fn inline_query_result_cached_gif_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultCachedGif::new("id", "file-id")
                .title("title")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_file_id": "file-id",
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        })
    );
}

#[test]
fn inline_query_result_cached_gif_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedGif::new(
            "id", "file-id",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "gif",
            "id": "id",
            "gif_file_id": "file-id"
        })
    );
}

use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultMpeg4Gif,
    InputMessageContentText,
    ParseMode,
};

#[test]
fn inline_query_result_mpeg4_gif_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultMpeg4Gif::new("id", "url", "thumb-url")
                .thumb_mime_type("video/mp4")
                .mpeg4_width(200)
                .mpeg4_height(300)
                .mpeg4_duration(400)
                .title("title")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
        serde_json::json!({
            "type": "mpeg4_gif",
            "id": "id",
            "mpeg4_url": "url",
            "thumb_url": "thumb-url",
            "thumb_mime_type": "video/mp4",
            "mpeg4_width": 200,
            "mpeg4_height": 300,
            "mpeg4_duration": 400,
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        })
    );
}

#[test]
fn inline_query_result_mpeg4_gif_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultMpeg4Gif::new(
            "id",
            "url",
            "thumb-url",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "mpeg4_gif",
            "id": "id",
            "mpeg4_url": "url",
            "thumb_url": "thumb-url"
        })
    );
}

#[test]
fn inline_query_result_cached_mpeg4_gif_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultCachedMpeg4Gif::new("id", "file-id")
                .title("title")
                .caption("caption")
                .parse_mode(ParseMode::Markdown)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
        ))
        .unwrap(),
        serde_json::json!({
            "type": "mpeg4_gif",
            "id": "id",
            "mpeg4_file_id": "file-id",
            "title": "title",
            "caption": "caption",
            "parse_mode": "Markdown",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"}
        })
    );
}

#[test]
fn inline_query_result_cached_mpeg4_gif_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultCachedMpeg4Gif::new(
            "id", "file-id",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "mpeg4_gif",
            "id": "id",
            "mpeg4_file_id": "file-id"
        })
    );
}

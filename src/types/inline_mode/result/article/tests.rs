use crate::types::{InlineKeyboardButton, InlineQueryResult, InlineQueryResultArticle, InputMessageContentText};

#[test]
fn inline_query_result_article_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultArticle::new("id", "title", InputMessageContentText::new("text"))
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .url("URL")
                .hide_url(true)
                .description("desc")
                .thumb_url("thumb-url")
                .thumb_width(200)
                .thumb_height(200)
        ))
        .unwrap(),
        serde_json::json!({
            "type": "article",
            "id": "id",
            "title": "title",
            "input_message_content": {"message_text": "text"},
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "url": "URL",
            "hide_url": true,
            "description": "desc",
            "thumb_url": "thumb-url",
            "thumb_width": 200,
            "thumb_height": 200
        })
    );
}

#[test]
fn inline_query_result_article_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultArticle::new(
            "id",
            "title",
            InputMessageContentText::new("text"),
        )))
        .unwrap(),
        serde_json::json!({
            "type": "article",
            "id": "id",
            "title": "title",
            "input_message_content": {"message_text": "text"}
        })
    );
}

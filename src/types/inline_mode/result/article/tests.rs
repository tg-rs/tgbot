use crate::{
    tests::assert_json_eq,
    types::{InlineKeyboardButton, InlineQueryResult, InlineQueryResultArticle, InputMessageContentText},
};

#[test]
fn inline_query_result_article() {
    let result = InlineQueryResultArticle::new("id", "title", InputMessageContentText::new("text"));
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .url("URL")
                .hide_url(true)
                .description("desc")
                .thumb_url("thumb-url")
                .thumb_width(200)
                .thumb_height(200),
        ),
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
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "article",
            "id": "id",
            "title": "title",
            "input_message_content": {"message_text": "text"}
        }),
    );
}

use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultArticle,
    InputMessageContentText,
};

#[test]
fn inline_query_result_article() {
    let result = InlineQueryResultArticle::new("id", InputMessageContentText::new("text"), "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_description("desc")
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_thumbnail_url("thumb-url")
                .with_thumbnail_width(200)
                .with_thumbnail_height(200)
                .with_url("URL"),
        ),
        serde_json::json!({
            "type": "article",
            "id": "id",
            "title": "title",
            "input_message_content": {"message_text": "text"},
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "url": "URL",
            "description": "desc",
            "thumbnail_url": "thumb-url",
            "thumbnail_width": 200,
            "thumbnail_height": 200
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

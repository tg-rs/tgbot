use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultLocation,
    InputMessageContentText,
};

#[test]
fn inline_query_result_location() {
    let result = InlineQueryResultLocation::new("id", 1.0, 2.0, "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .live_period(100)
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
                .thumb_url("thumb-url")
                .thumb_width(200)
                .thumb_height(300),
        ),
        serde_json::json!({
            "type": "location",
            "id": "id",
            "latitude": 1.0,
            "longitude": 2.0,
            "title": "title",
            "live_period": 100,
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"},
            "thumb_url": "thumb-url",
            "thumb_width": 200,
            "thumb_height": 300
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "location",
            "id": "id",
            "latitude": 1.0,
            "longitude": 2.0,
            "title": "title"
        }),
    );
}

use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultLocation,
    InputMessageContentText,
    tests::assert_json_eq,
};

#[test]
fn inline_query_result_location() {
    let result = InlineQueryResultLocation::new("id", 1.0, 2.0, "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_live_period(100)
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_thumbnail_url("thumb-url")
                .with_thumbnail_width(200)
                .with_thumbnail_height(300),
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
            "thumbnail_url": "thumb-url",
            "thumbnail_width": 200,
            "thumbnail_height": 300
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

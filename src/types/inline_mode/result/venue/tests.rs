use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultVenue,
    InputMessageContentText,
};

#[test]
fn inline_query_result_venue() {
    let result = InlineQueryResultVenue::new("id", 1.0, 2.0, "title", "addr");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .foursquare_id("f-id")
                .foursquare_type("f-type")
                .google_place_id("g-id")
                .google_place_type("g-type")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
                .thumb_url("thumb-url")
                .thumb_width(200)
                .thumb_height(300),
        ),
        serde_json::json!({
            "type": "venue",
            "id": "id",
            "latitude": 1.0,
            "longitude": 2.0,
            "title": "title",
            "address": "addr",
            "foursquare_id": "f-id",
            "foursquare_type": "f-type",
            "google_place_id": "g-id",
            "google_place_type": "g-type",
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
            "type": "venue",
            "id": "id",
            "latitude": 1.0,
            "longitude": 2.0,
            "title": "title",
            "address": "addr"
        }),
    );
}

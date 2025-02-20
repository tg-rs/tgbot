use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultVenue,
    InputMessageContentText,
    tests::assert_json_eq,
};

#[test]
fn inline_query_result_venue() {
    let result = InlineQueryResultVenue::new("addr", "id", 1.0, 2.0, "title");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_foursquare_id("f-id")
                .with_foursquare_type("f-type")
                .with_google_place_id("g-id")
                .with_google_place_type("g-type")
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_thumbnail_height(300)
                .with_thumbnail_url("thumb-url")
                .with_thumbnail_width(200),
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
            "thumbnail_url": "thumb-url",
            "thumbnail_width": 200,
            "thumbnail_height": 300
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

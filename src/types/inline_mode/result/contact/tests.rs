use crate::types::{
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultContact,
    InputMessageContentText,
    tests::assert_json_eq,
};

#[test]
fn inline_query_result_contact() {
    let result = InlineQueryResultContact::new("name", "id", "phone");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .with_input_message_content(InputMessageContentText::new("text"))
                .with_last_name("last name")
                .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
                .with_thumbnail_url("url")
                .with_thumbnail_width(200)
                .with_thumbnail_height(200)
                .with_vcard("vcard"),
        ),
        serde_json::json!({
            "type": "contact",
            "id": "id",
            "phone_number": "phone",
            "first_name": "name",
            "last_name": "last name",
            "vcard": "vcard",
            "reply_markup": {"inline_keyboard": [[{"text": "text", "url": "url"}]]},
            "input_message_content": {"message_text": "text"},
            "thumbnail_url": "url",
            "thumbnail_width": 200,
            "thumbnail_height": 200
        }),
    );
    assert_json_eq(
        InlineQueryResult::from(result),
        serde_json::json!({
            "type": "contact",
            "id": "id",
            "phone_number": "phone",
            "first_name": "name"
        }),
    );
}

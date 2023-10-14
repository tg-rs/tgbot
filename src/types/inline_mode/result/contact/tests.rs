use crate::types::{
    tests::assert_json_eq,
    InlineKeyboardButton,
    InlineQueryResult,
    InlineQueryResultContact,
    InputMessageContentText,
};

#[test]
fn inline_query_result_contact() {
    let result = InlineQueryResultContact::new("id", "phone", "name");
    assert_json_eq(
        InlineQueryResult::from(
            result
                .clone()
                .last_name("last name")
                .vcard("vcard")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
                .thumb_url("url")
                .thumb_width(200)
                .thumb_height(200),
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
            "thumb_url": "url",
            "thumb_width": 200,
            "thumb_height": 200
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

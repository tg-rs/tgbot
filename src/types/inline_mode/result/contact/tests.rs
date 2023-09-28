use crate::types::{InlineKeyboardButton, InlineQueryResult, InlineQueryResultContact, InputMessageContentText};

#[test]
fn inline_query_result_contact_serialize_full() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(
            InlineQueryResultContact::new("id", "phone", "name")
                .last_name("last name")
                .vcard("vcard")
                .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
                .input_message_content(InputMessageContentText::new("text"))
                .thumb_url("url")
                .thumb_width(200)
                .thumb_height(200)
        ))
        .unwrap(),
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
        })
    );
}

#[test]
fn inline_query_result_contact_serialize_partial() {
    assert_eq!(
        serde_json::to_value(InlineQueryResult::from(InlineQueryResultContact::new(
            "id", "phone", "name",
        )))
        .unwrap(),
        serde_json::json!({
            "type": "contact",
            "id": "id",
            "phone_number": "phone",
            "first_name": "name"
        })
    );
}

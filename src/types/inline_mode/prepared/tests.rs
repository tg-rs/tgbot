use crate::{
    api::{Payload, assert_payload_eq},
    types::{InlineQueryResultContact, PreparedInlineMessage, SavePreparedInlineMessage, tests::assert_json_eq},
};

#[test]
fn prepared_inline_message() {
    assert_json_eq(
        PreparedInlineMessage::new("id", 1),
        serde_json::json!({
            "id": "id",
            "expiration_date": 1,
        }),
    );
}

#[test]
fn save_prepared_inline_message() {
    let method = SavePreparedInlineMessage::new(1, InlineQueryResultContact::new("test", "result-id", "+1000"));
    assert_payload_eq(
        Payload::json(
            "savePreparedInlineMessage",
            serde_json::json!({
                "user_id": 1,
                "result": {
                    "type": "contact",
                    "first_name": "test",
                    "id": "result-id",
                    "phone_number": "+1000",
                },
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "savePreparedInlineMessage",
            serde_json::json!({
                "user_id": 1,
                "result": {
                    "type": "contact",
                    "first_name": "test",
                    "id": "result-id",
                    "phone_number": "+1000",
                },
                "allow_bot_chats": true,
                "allow_channel_chats": true,
                "allow_group_chats": true,
                "allow_user_chats": true,
            }),
        ),
        method
            .with_allow_bot_chats(true)
            .with_allow_channel_chats(true)
            .with_allow_group_chats(true)
            .with_allow_user_chats(true),
    );
}

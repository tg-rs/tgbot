use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        BusinessConnection,
        BusinessIntro,
        BusinessLocation,
        BusinessMessagesDeleted,
        GetBusinessConnection,
        Location,
        PrivateChat,
        Sticker,
        StickerType,
        User,
    },
};

#[test]
fn business_connection() {
    let expected_struct = BusinessConnection::new(0, "id", User::new(1, "test", false), 2);
    let mut expected_value = serde_json::json!({
        "can_reply": false,
        "date": 0,
        "id": "id",
        "is_enabled": false,
        "user": {
            "id": 1,
            "first_name": "test",
            "is_bot": false
        },
        "user_chat_id": 2,
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    let expected_struct = expected_struct.with_can_reply(true).with_is_enabled(true);
    expected_value["can_reply"] = serde_json::json!(true);
    expected_value["is_enabled"] = serde_json::json!(true);
    assert_json_eq(expected_struct, expected_value)
}

#[test]
fn business_intro() {
    assert_json_eq(BusinessIntro::default(), serde_json::json!({}));

    assert_json_eq(
        BusinessIntro::default()
            .with_message("msg")
            .with_sticker(Sticker::new(
                "file-id",
                "file-unique-id",
                StickerType::Regular,
                512,
                512,
            ))
            .with_title("title"),
        serde_json::json!({
            "message": "msg",
            "sticker": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "type": "regular",
                "height": 512,
                "width": 512,
                "is_animated": false,
                "is_video": false,
            },
            "title": "title"
        }),
    );
}

#[test]
fn business_location() {
    assert_json_eq(
        BusinessLocation::new("address"),
        serde_json::json!({"address": "address"}),
    );
    assert_json_eq(
        BusinessLocation::new("address").with_location(Location::new(1.0, 2.0)),
        serde_json::json!({
            "address": "address",
            "location": {
                "latitude": 1.0,
                "longitude": 2.0,
            }
        }),
    );
}

#[test]
fn business_messages_deleted() {
    let expected_struct = BusinessMessagesDeleted::new("id", PrivateChat::new(1, "test"), [2]);
    let expected_value = serde_json::json!({
        "business_connection_id": "id",
        "chat": {
            "type": "private",
            "id": 1,
            "first_name": "test"
        },
        "message_ids": [2]
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn get_business_connection() {
    assert_payload_eq(
        Payload::json(
            "getBusinessConnection",
            serde_json::json!({"business_connection_id": "id"}),
        ),
        GetBusinessConnection::new("id"),
    )
}

use crate::{
    api::{Payload, assert_payload_eq},
    types::{
        BusinessBotRights,
        BusinessConnection,
        BusinessIntro,
        BusinessLocation,
        BusinessMessagesDeleted,
        BusinessOpeningHours,
        DeleteBusinessMessages,
        GetBusinessConnection,
        Location,
        PrivateChat,
        ReadBusinessMessage,
        SetBusinessAccountName,
        SetBusinessAccountUsername,
        Sticker,
        StickerType,
        User,
        tests::assert_json_eq,
    },
};

#[test]
fn business_bot_rights() {
    let expected_struct = BusinessBotRights::default();
    assert_json_eq(expected_struct, serde_json::json!({}));

    let expected_struct = expected_struct
        .with_can_change_gift_settings(true)
        .with_can_convert_gifts_to_stars(true)
        .with_can_delete_all_messages(true)
        .with_can_delete_outgoing_messages(true)
        .with_can_edit_bio(true)
        .with_can_edit_name(true)
        .with_can_edit_profile_photo(true)
        .with_can_edit_username(true)
        .with_can_manage_stories(true)
        .with_can_read_messages(true)
        .with_can_reply(true)
        .with_can_transfer_and_upgrade_gifts(true)
        .with_can_transfer_stars(true)
        .with_can_view_gifts_and_stars(true);
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "can_change_gift_settings": true,
            "can_convert_gifts_to_stars": true,
            "can_delete_all_messages": true,
            "can_delete_outgoing_messages": true,
            "can_edit_bio": true,
            "can_edit_name": true,
            "can_edit_profile_photo": true,
            "can_edit_username": true,
            "can_manage_stories": true,
            "can_read_messages": true,
            "can_reply": true,
            "can_transfer_and_upgrade_gifts": true,
            "can_transfer_stars": true,
            "can_view_gifts_and_stars": true,
        }),
    );
}

#[test]
fn business_connection() {
    let expected_struct = BusinessConnection::new(0, "id", User::new(1, "test", false), 2);
    let mut expected_value = serde_json::json!({
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

    let expected_struct = expected_struct
        .with_rights(BusinessBotRights::default())
        .with_is_enabled(true);
    expected_value["rights"] = serde_json::json!({});
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
fn business_opening_hours() {
    let expected_struct = BusinessOpeningHours::new("UTC", [(1, 2), (3, 4)]);
    let expected_value = serde_json::json!({
        "time_zone_name": "UTC",
        "opening_hours": [
            {
                "opening_minute": 1,
                "closing_minute": 2,
            },
            {
                "opening_minute": 3,
                "closing_minute": 4,
            }
        ]
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn delete_business_messages() {
    assert_payload_eq(
        Payload::json(
            "deleteBusinessMessages",
            serde_json::json!({
                "business_connection_id": "id",
                "message_ids": [1, 2, 3]
            }),
        ),
        DeleteBusinessMessages::new("id", [1, 2, 3]),
    );
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

#[test]
fn read_business_message() {
    assert_payload_eq(
        Payload::json(
            "readBusinessMessage",
            serde_json::json!({
                "business_connection_id": "id",
                "chat_id": 1,
                "message_id": 2,
            }),
        ),
        ReadBusinessMessage::new("id", 1, 2),
    );
}

#[test]
fn set_business_account_name() {
    let actual_method = SetBusinessAccountName::new("id", "John");
    assert_payload_eq(
        Payload::json(
            "setBusinessAccountName",
            serde_json::json!({
                "business_connection_id": "id",
                "first_name": "John",
            }),
        ),
        actual_method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setBusinessAccountName",
            serde_json::json!({
                "business_connection_id": "id",
                "first_name": "John",
                "last_name": "Doe",
            }),
        ),
        actual_method.with_last_name("Doe"),
    );
}

#[test]
fn set_business_account_username() {
    let actual_method = SetBusinessAccountUsername::new("id");
    assert_payload_eq(
        Payload::json(
            "setBusinessAccountUsername",
            serde_json::json!({
                "business_connection_id": "id",
            }),
        ),
        actual_method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setBusinessAccountUsername",
            serde_json::json!({
                "business_connection_id": "id",
                "username": "johndoe",
            }),
        ),
        actual_method.with_username("johndoe"),
    );
}

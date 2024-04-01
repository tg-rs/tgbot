use crate::types::{
    tests::assert_json_eq,
    AccentColor,
    BusinessIntro,
    BusinessLocation,
    Chat,
    ChatPhoto,
    Message,
    MessageData,
    PrivateChat,
    ProfileAccentColor,
    Text,
    User,
};

#[test]
fn private_chat() {
    let expected_struct = Chat::Private(
        PrivateChat::new(1, "John")
            .with_last_name("Doe")
            .with_username("john_doe")
            .with_photo(ChatPhoto::new(
                "big-file-id",
                "big-file-unique-id",
                "small-file-id",
                "small-file-unique-id",
            ))
            .with_bio("Bio")
            .with_business_intro(BusinessIntro::default())
            .with_business_location(BusinessLocation::new("address"))
            .with_pinned_message(
                Message::new(
                    1,
                    0,
                    PrivateChat::new(1, "John")
                        .with_last_name("Doe")
                        .with_username("john_doe"),
                    MessageData::Text(Text::from("message-text")),
                    User::new(1, "John", false)
                        .with_last_name("Doe")
                        .with_username("john_doe"),
                )
                .with_edit_date(0),
            )
            .with_has_private_forwards(true)
            .with_message_auto_delete_time(86400)
            .with_has_restricted_voice_and_video_messages(true)
            .with_active_usernames(["john_doe"])
            .with_emoji_status_custom_emoji_id("emoji-id")
            .with_emoji_status_expiration_date(0)
            .with_accent_color(AccentColor::Red)
            .with_background_custom_emoji_id("emoji-id")
            .with_profile_accent_color(ProfileAccentColor::try_from(1).unwrap())
            .with_profile_background_custom_emoji_id("emoji-id"),
    );
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(expected_struct.get_username().unwrap(), "john_doe");
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "private",
            "username": "john_doe",
            "first_name": "John",
            "last_name": "Doe",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "bio": "Bio",
            "business_intro": {},
            "business_location": {
                "address": "address"
            },
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "private",
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "john_doe"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "john_doe"
                },
                "text": "message-text",
                "edit_date": 0,
                "has_protected_content": false,
                "is_automatic_forward": false,
            },
            "has_private_forwards": true,
            "message_auto_delete_time": 86400,
            "has_restricted_voice_and_video_messages": true,
            "active_usernames": ["john_doe"],
            "emoji_status_custom_emoji_id": "emoji-id",
            "emoji_status_expiration_date": 0,
            "accent_color_id": 0,
            "background_custom_emoji_id": "emoji-id",
            "profile_accent_color_id": 1,
            "profile_background_custom_emoji_id": "emoji-id"
        }),
    );
    let expected_struct = Chat::Private(PrivateChat::new(1, "John"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "private",
            "first_name": "John",
        }),
    );
}

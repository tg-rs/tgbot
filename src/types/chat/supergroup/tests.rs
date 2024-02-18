use crate::types::{
    tests::assert_json_eq,
    AccentColor,
    Chat,
    ChatLocation,
    ChatPermissions,
    ChatPhoto,
    Location,
    Message,
    MessageData,
    ProfileAccentColor,
    ReactionType,
    SupergroupChat,
    Text,
    User,
};

#[test]
fn supergroup() {
    let expected_struct = Chat::Supergroup(
        SupergroupChat::new(1, "Supergroup Chat")
            .with_username("supergroup_chat")
            .with_photo(ChatPhoto::new(
                "big-file-id",
                "big-file-unique-id",
                "small-file-id",
                "small-file-unique-id",
            ))
            .with_description("Description")
            .with_emoji_status_custom_emoji_id("emoji-id")
            .with_emoji_status_expiration_date(0)
            .with_invite_link("example.com/join/supergroup")
            .with_pinned_message(Message::new(
                1,
                0,
                SupergroupChat::new(1, "Supergroup Chat").with_username("supergroup_chat"),
                MessageData::Text(Text::from("message-text")),
                User::new(1, "User", false),
            ))
            .with_sticker_set_name("Sticker Set")
            .with_can_set_sticker_set(true)
            .with_custom_emoji_sticker_set_name("test")
            .with_permissions(
                ChatPermissions::default()
                    .with_can_send_messages(true)
                    .with_can_send_polls(true)
                    .with_can_send_other_messages(true)
                    .with_can_add_web_page_previews(true)
                    .with_can_change_info(true)
                    .with_can_invite_users(true)
                    .with_can_pin_messages(true),
            )
            .with_slow_mode_delay(10)
            .with_message_auto_delete_time(86400)
            .with_linked_chat_id(2)
            .with_location(ChatLocation::new("Address", Location::new(1.0, 0.0)))
            .with_has_protected_content(true)
            .with_join_to_send_messages(true)
            .with_join_by_request(true)
            .with_is_forum(true)
            .with_active_usernames(vec!["supergroup_chat"])
            .with_has_hidden_members(true)
            .with_has_aggressive_anti_spam_enabled(true)
            .with_available_reactions([ReactionType::emoji("👍")])
            .with_accent_color(AccentColor::Red)
            .with_background_custom_emoji_id("emoji-id")
            .with_profile_accent_color(ProfileAccentColor::try_from(1).unwrap())
            .with_profile_background_custom_emoji_id("emoji-id")
            .with_has_visible_history(true)
            .with_unrestrict_boost_count(1),
    );
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(expected_struct.get_username().unwrap(), "supergroup_chat");
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "Supergroup Chat",
            "username": "supergroup_chat",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "description": "Description",
            "invite_link": "example.com/join/supergroup",
            "sticker_set_name": "Sticker Set",
            "can_set_sticker_set": true,
            "custom_emoji_sticker_set_name": "test",
            "slow_mode_delay": 10,
            "permissions": {
                "can_send_messages": true,
                "can_send_polls": true,
                "can_send_other_messages": true,
                "can_add_web_page_previews": true,
                "can_change_info": true,
                "can_invite_users": true,
                "can_pin_messages": true,
            },
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "supergroup",
                    "title": "Supergroup Chat",
                    "username": "supergroup_chat"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "User"
                },
                "text": "message-text",
                "has_protected_content": false,
                "is_automatic_forward": false,
            },
            "linked_chat_id": 2,
            "location": {
                "location": {
                    "longitude": 0.0,
                    "latitude": 1.0
                },
                "address": "Address"
            },
            "has_protected_content": true,
            "message_auto_delete_time": 86400,
            "join_to_send_messages": true,
            "join_by_request": true,
            "is_forum": true,
            "active_usernames": ["supergroup_chat"],
            "has_hidden_members": true,
            "has_aggressive_anti_spam_enabled": true,
            "available_reactions": [
                {
                    "type": "emoji",
                    "emoji": "👍"
                }
            ],
            "emoji_status_custom_emoji_id": "emoji-id",
            "emoji_status_expiration_date": 0,
            "accent_color_id": 0,
            "background_custom_emoji_id": "emoji-id",
            "profile_accent_color_id": 1,
            "profile_background_custom_emoji_id": "emoji-id",
            "has_visible_history": true,
            "unrestrict_boost_count": 1
        }),
    );

    let expected_struct = Chat::Supergroup(SupergroupChat::new(1, "Supergroup Chat"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "Supergroup Chat"
        }),
    );
}

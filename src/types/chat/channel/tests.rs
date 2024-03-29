use crate::types::{
    tests::assert_json_eq,
    AccentColor,
    ChannelChat,
    Chat,
    ChatPhoto,
    Message,
    MessageData,
    ProfileAccentColor,
    ReactionType,
    Text,
    User,
};

#[test]
fn channel_chat() {
    let expected_struct = Chat::Channel(
        ChannelChat::new(1, "Channel")
            .with_username("channel_username")
            .with_photo(ChatPhoto::new(
                "big-file-id",
                "big-file-unique-id",
                "small-file-id",
                "small-file-unique-id",
            ))
            .with_description("Description")
            .with_invite_link("example.com/join/channel")
            .with_pinned_message(Message::new(
                1,
                0,
                ChannelChat::new(1, "Channel").with_username("channel_username"),
                MessageData::Text(Text::from("text")),
                User::new(1, "User", false),
            ))
            .with_linked_chat_id(2)
            .with_has_protected_content(true)
            .with_message_auto_delete_time(86400)
            .with_active_usernames(["channel_username"])
            .with_available_reactions([ReactionType::emoji("👍")])
            .with_emoji_status_custom_emoji_id("emoji-id")
            .with_emoji_status_expiration_date(0)
            .with_accent_color(AccentColor::Red)
            .with_background_custom_emoji_id("emoji-id")
            .with_profile_accent_color(ProfileAccentColor::try_from(1).unwrap())
            .with_profile_background_custom_emoji_id("emoji-id"),
    );
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(&expected_struct.get_username().unwrap(), &"channel_username");
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "Channel",
            "username": "channel_username",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "description": "Description",
            "invite_link": "example.com/join/channel",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "User"
                },
                "chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "Channel",
                    "username": "channel_username"
                },
                "text": "text",
                "has_protected_content": false,
                "is_automatic_forward": false
            },
            "linked_chat_id": 2,
            "has_protected_content": true,
            "message_auto_delete_time": 86400,
            "active_usernames": ["channel_username"],
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
            "profile_background_custom_emoji_id": "emoji-id"
        }),
    );

    let expected_struct = Chat::Channel(ChannelChat::new(1, "Channel"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "Channel"
        }),
    );
}

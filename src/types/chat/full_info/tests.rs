use crate::types::{
    AccentColor,
    Birthdate,
    BusinessIntro,
    BusinessLocation,
    BusinessOpeningHours,
    ChannelChat,
    Chat,
    ChatFullInfo,
    ChatFullInfoType,
    ChatLocation,
    ChatPermissions,
    ChatPhoto,
    GroupChat,
    Location,
    Message,
    MessageData,
    PrivateChat,
    ProfileAccentColor,
    ReactionType,
    SupergroupChat,
    Text,
    User,
    tests::assert_json_eq,
};

fn create_chat_full_info(chat_type: ChatFullInfoType) -> ChatFullInfo {
    ChatFullInfo {
        id: 1,
        chat_type,
        accent_color: AccentColor::Red,
        max_reaction_count: 0,
        available_reactions: None,
        background_custom_emoji_id: None,
        has_hidden_members: None,
        has_protected_content: None,
        has_visible_history: None,
        message_auto_delete_time: None,
        photo: None,
        pinned_message: None,
        profile_accent_color: None,
        profile_background_custom_emoji_id: None,
        active_usernames: None,
        username: None,
        description: None,
        invite_link: None,
        title: None,
        can_send_paid_media: None,
        linked_chat_id: None,
        bio: None,
        birthdate: None,
        business_intro: None,
        business_location: None,
        business_opening_hours: None,
        emoji_status_custom_emoji_id: None,
        emoji_status_expiration_date: None,
        first_name: None,
        has_private_forwards: None,
        has_restricted_voice_and_video_messages: None,
        last_name: None,
        personal_chat: None,
        can_set_sticker_set: None,
        custom_emoji_sticker_set_name: None,
        has_aggressive_anti_spam_enabled: None,
        is_forum: None,
        join_to_send_messages: None,
        join_by_request: None,
        location: None,
        slow_mode_delay: None,
        sticker_set_name: None,
        unrestrict_boost_count: None,
        permissions: None,
        accepted_gift_types: None,
    }
}

#[test]
fn chat_full_info_channel() {
    let expected_struct = create_chat_full_info(ChatFullInfoType::Channel);

    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": 1,
            "type": "channel",
            "accent_color_id": 0,
            "max_reaction_count": 0,
        }),
    );

    let mut expected_struct = expected_struct;
    expected_struct.available_reactions = Some(vec![
        ReactionType::emoji("👍"),
        ReactionType::Paid,
        ReactionType::custom_emoji("5420319826440644296"),
    ]);
    expected_struct.background_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.has_protected_content = Some(true);
    expected_struct.message_auto_delete_time = Some(86400);
    expected_struct.photo = Some(ChatPhoto::new(
        "big-file-id",
        "big-file-unique-id",
        "small-file-id",
        "small-file-unique-id",
    ));
    expected_struct.pinned_message = Some(Message::new(
        1,
        0,
        ChannelChat::new(1, "Channel").with_username("channel_username"),
        MessageData::Text(Text::from("text")),
        User::new(1, "User", false),
    ));
    expected_struct.profile_accent_color = Some(ProfileAccentColor::try_from(1).unwrap());
    expected_struct.profile_background_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.active_usernames = Some(vec![String::from("channel_username")]);
    expected_struct.username = Some(String::from("channel_username"));
    expected_struct.description = Some(String::from("Description"));
    expected_struct.invite_link = Some(String::from("example.com/join/channel"));
    expected_struct.can_send_paid_media = Some(true);
    expected_struct.title = Some(String::from("Channel"));
    expected_struct.linked_chat_id = Some(2);
    expected_struct.emoji_status_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.emoji_status_expiration_date = Some(0);

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
            "can_send_paid_media": true,
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
                },
                {"type": "paid"},
                {"type": "custom_emoji", "custom_emoji_id": "5420319826440644296"},
            ],
            "emoji_status_custom_emoji_id": "emoji-id",
            "emoji_status_expiration_date": 0,
            "accent_color_id": 0,
            "background_custom_emoji_id": "emoji-id",
            "profile_accent_color_id": 1,
            "profile_background_custom_emoji_id": "emoji-id",
            "max_reaction_count": 0,
        }),
    );
}

#[test]
fn group_chat() {
    let expected_struct = create_chat_full_info(ChatFullInfoType::Group);

    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": 1,
            "type": "group",
            "accent_color_id": 0,
            "max_reaction_count": 0,
        }),
    );

    let mut expected_struct = expected_struct;
    expected_struct.title = Some(String::from("Group"));
    expected_struct.photo = Some(ChatPhoto::new(
        "big-file-id",
        "big-file-unique-id",
        "small-file-id",
        "small-file-unique-id",
    ));
    expected_struct.invite_link = Some(String::from("example.com/join/group"));
    expected_struct.pinned_message = Some(Message::new(
        1,
        0,
        GroupChat::new(1, "Group"),
        MessageData::Text(Text::from("text")),
        User::new(1, "User", false),
    ));
    expected_struct.permissions = Some(ChatPermissions::allowed());
    expected_struct.has_protected_content = Some(true);
    expected_struct.message_auto_delete_time = Some(86400);
    expected_struct.has_hidden_members = Some(true);
    expected_struct.available_reactions = Some(vec![ReactionType::emoji("👍")]);
    expected_struct.emoji_status_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.emoji_status_expiration_date = Some(0);
    expected_struct.background_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.profile_accent_color = Some(ProfileAccentColor::try_from(1).unwrap());
    expected_struct.profile_background_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.has_visible_history = Some(true);

    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "Group",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "invite_link": "example.com/join/group",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "Group"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "User"
                },
                "text": "text",
                "has_protected_content": false,
                "is_automatic_forward": false
            },
            "permissions": {
                "can_send_messages": true,
                "can_send_audios": true,
                "can_send_documents": true,
                "can_send_photos": true,
                "can_send_videos": true,
                "can_send_video_notes": true,
                "can_send_voice_notes": true,
                "can_send_polls": true,
                "can_send_other_messages": true,
                "can_add_web_page_previews": true,
                "can_change_info": true,
                "can_invite_users": true,
                "can_pin_messages": true,
                "can_manage_topics": true,
            },
            "has_protected_content": true,
            "message_auto_delete_time": 86400,
            "has_hidden_members": true,
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
            "max_reaction_count": 0,
        }),
    );
}

#[test]
fn private_chat() {
    let expected_struct = create_chat_full_info(ChatFullInfoType::Private);

    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": 1,
            "type": "private",
            "accent_color_id": 0,
            "max_reaction_count": 0,
        }),
    );

    let mut expected_struct = expected_struct;
    expected_struct.first_name = Some(String::from("John"));
    expected_struct.last_name = Some(String::from("Doe"));
    expected_struct.username = Some(String::from("john_doe"));
    expected_struct.photo = Some(ChatPhoto::new(
        "big-file-id",
        "big-file-unique-id",
        "small-file-id",
        "small-file-unique-id",
    ));
    expected_struct.bio = Some(String::from("Bio"));
    expected_struct.birthdate = Some(Birthdate::new(1, 1));
    expected_struct.business_intro = Some(BusinessIntro::default());
    expected_struct.business_location = Some(BusinessLocation::new("address"));
    expected_struct.business_opening_hours = Some(BusinessOpeningHours::new("UTC", [(0, 1)]));
    expected_struct.pinned_message = Some(
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
    );
    expected_struct.has_private_forwards = Some(true);
    expected_struct.message_auto_delete_time = Some(86400);
    expected_struct.has_restricted_voice_and_video_messages = Some(true);
    expected_struct.active_usernames = Some(vec![String::from("john_doe")]);
    expected_struct.emoji_status_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.emoji_status_expiration_date = Some(0);
    expected_struct.background_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.personal_chat = Some(Chat::from(ChannelChat::new(1, "test")));
    expected_struct.profile_accent_color = Some(ProfileAccentColor::try_from(1).unwrap());
    expected_struct.profile_background_custom_emoji_id = Some(String::from("emoji-id"));

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
            "birthdate": {
                "day": 1,
                "month": 1
            },
            "business_intro": {},
            "business_location": {
                "address": "address"
            },
            "business_opening_hours": {
                "time_zone_name": "UTC",
                "opening_hours": [
                    {
                        "opening_minute": 0,
                        "closing_minute": 1
                    }
                ]
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
            "personal_chat": {
                "type": "channel",
                "id": 1,
                "title": "test"
            },
            "profile_accent_color_id": 1,
            "profile_background_custom_emoji_id": "emoji-id",
            "max_reaction_count": 0,
        }),
    );
}

#[test]
fn supergroup_chat() {
    let expected_struct = create_chat_full_info(ChatFullInfoType::Supergroup);

    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "accent_color_id": 0,
            "max_reaction_count": 0,
        }),
    );

    let mut expected_struct = expected_struct.clone();
    expected_struct.title = Some(String::from("Supergroup Chat"));
    expected_struct.username = Some(String::from("supergroup_chat"));
    expected_struct.photo = Some(ChatPhoto::new(
        "big-file-id",
        "big-file-unique-id",
        "small-file-id",
        "small-file-unique-id",
    ));
    expected_struct.description = Some(String::from("Description"));
    expected_struct.emoji_status_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.emoji_status_expiration_date = Some(0);
    expected_struct.invite_link = Some(String::from("example.com/join/supergroup"));
    expected_struct.pinned_message = Some(Message::new(
        1,
        0,
        SupergroupChat::new(1, "Supergroup Chat").with_username("supergroup_chat"),
        MessageData::Text(Text::from("message-text")),
        User::new(1, "User", false),
    ));
    expected_struct.sticker_set_name = Some(String::from("Sticker Set"));
    expected_struct.can_set_sticker_set = Some(true);
    expected_struct.custom_emoji_sticker_set_name = Some(String::from("test"));
    expected_struct.permissions = Some(
        ChatPermissions::default()
            .with_can_send_messages(true)
            .with_can_send_polls(true)
            .with_can_send_other_messages(true)
            .with_can_add_web_page_previews(true)
            .with_can_change_info(true)
            .with_can_invite_users(true)
            .with_can_pin_messages(true),
    );
    expected_struct.slow_mode_delay = Some(10);
    expected_struct.message_auto_delete_time = Some(86400);
    expected_struct.linked_chat_id = Some(2);
    expected_struct.location = Some(ChatLocation::new("Address", Location::new(1.0, 0.0)));
    expected_struct.has_protected_content = Some(true);
    expected_struct.join_to_send_messages = Some(true);
    expected_struct.join_by_request = Some(true);
    expected_struct.is_forum = Some(true);
    expected_struct.active_usernames = Some(vec![String::from("supergroup_chat")]);
    expected_struct.has_hidden_members = Some(true);
    expected_struct.has_aggressive_anti_spam_enabled = Some(true);
    expected_struct.available_reactions = Some(vec![ReactionType::emoji("👍")]);
    expected_struct.background_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.profile_accent_color = Some(ProfileAccentColor::try_from(1).unwrap());
    expected_struct.profile_background_custom_emoji_id = Some(String::from("emoji-id"));
    expected_struct.has_visible_history = Some(true);
    expected_struct.unrestrict_boost_count = Some(1);

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
            "unrestrict_boost_count": 1,
            "max_reaction_count": 0,
        }),
    );
}

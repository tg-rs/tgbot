use std::collections::HashMap;

use crate::{
    api::{Form, FormValue, Payload, assert_payload_eq},
    types::*,
};

#[test]
fn channel_chat() {
    let expected_struct = Chat::Channel(ChannelChat::new(1, "Channel").with_username("channel_username"));
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(&expected_struct.get_username().unwrap(), &"channel_username");
    insta::assert_json_snapshot!(expected_struct);

    let expected_struct = Chat::Channel(ChannelChat::new(1, "Channel"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn group_chat() {
    let expected_struct = Chat::Group(GroupChat::new(1, "Group"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(&expected_struct.get_username().is_none());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn private_chat() {
    let expected_struct = Chat::Private(
        PrivateChat::new(1, "John")
            .with_last_name("Doe")
            .with_username("john_doe"),
    );
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(expected_struct.get_username().unwrap(), "john_doe");
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = Chat::Private(PrivateChat::new(1, "John"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn supergroup_chat() {
    let expected_struct = Chat::Supergroup(
        SupergroupChat::new(1, "Supergroup Chat")
            .with_is_forum(true)
            .with_username("supergroup_chat"),
    );
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(expected_struct.get_username().unwrap(), "supergroup_chat");
    insta::assert_json_snapshot!(expected_struct);

    let expected_struct = Chat::Supergroup(SupergroupChat::new(1, "Supergroup Chat"));
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_background() {
    insta::assert_json_snapshot!(ChatBackground::from(BackgroundType::Wallpaper {
        dark_theme_dimming: 100,
        document: Document::new("file-id", "file-unique-id"),
        is_blurred: Some(true),
        is_moving: Some(false),
    }));
}

#[test]
fn get_chat() {
    assert_payload_eq(
        Payload::json(
            "getChat",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChat::new(1),
    );
}

#[test]
fn leave_chat() {
    assert_payload_eq(
        Payload::json(
            "leaveChat",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        LeaveChat::new(1),
    );
}

#[test]
fn set_chat_description() {
    let method = SetChatDescription::new(1);
    assert_payload_eq(
        Payload::json(
            "setChatDescription",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setChatDescription",
            serde_json::json!({
                "chat_id": 1,
                "description": "Description"
            }),
        ),
        method.with_description("Description"),
    );
}

#[test]
fn set_chat_title() {
    assert_payload_eq(
        Payload::json(
            "setChatTitle",
            serde_json::json!({
                "chat_id": 1,
                "title": "Chat"
            }),
        ),
        SetChatTitle::new(1, "Chat"),
    );
}

#[test]
fn chat_action() {
    for value in [
        ChatAction::ChooseSticker,
        ChatAction::FindLocation,
        ChatAction::RecordVideo,
        ChatAction::RecordVideoNote,
        ChatAction::RecordVoice,
        ChatAction::Typing,
        ChatAction::UploadDocument,
        ChatAction::UploadPhoto,
        ChatAction::UploadVideo,
        ChatAction::UploadVideoNote,
        ChatAction::UploadVoice,
    ] {
        insta::assert_json_snapshot!(value);
    }
}

#[test]
fn send_chat_action() {
    let method = SendChatAction::new(1, ChatAction::Typing);
    assert_payload_eq(
        Payload::json(
            "sendChatAction",
            serde_json::json!({
                "chat_id": 1,
                "action": "typing"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "sendChatAction",
            serde_json::json!({
                "chat_id": 1,
                "action": "typing",
                "business_connection_id": "id",
                "message_thread_id": 1,
            }),
        ),
        method.with_business_connection_id("id").with_message_thread_id(1),
    );
}

#[test]
fn chat_boost() {
    insta::assert_json_snapshot!(ChatBoost::new(
        0,
        "id",
        0,
        ChatBoostSource::GiftCode(User::new(1, "test", false))
    ));
}

#[test]
fn chat_boost_removed() {
    insta::assert_json_snapshot!(ChatBoostRemoved::new(
        "id",
        ChannelChat::new(1, "test"),
        0,
        ChatBoostSource::GiftCode(User::new(1, "test", false)),
    ));
}

#[test]
fn chat_boost_source() {
    insta::assert_json_snapshot!(ChatBoostSource::GiftCode(User::new(1, "test", false)),);
    insta::assert_json_snapshot!(ChatBoostSource::Giveaway(ChatBoostSourceGiveaway::new(1)),);
    insta::assert_json_snapshot!(ChatBoostSource::Giveaway(
        ChatBoostSourceGiveaway::new(1).with_is_unclaimed(true)
    ));
    insta::assert_json_snapshot!(ChatBoostSource::Giveaway(
        ChatBoostSourceGiveaway::new(1)
            .with_prize_star_count(2)
            .with_user(User::new(1, "test", false)),
    ));
    insta::assert_json_snapshot!(ChatBoostSource::Premium(User::new(1, "test", false)));
}

#[test]
fn chat_boost_updated() {
    insta::assert_json_snapshot!(ChatBoostUpdated::new(
        ChatBoost::new(0, "id", 0, ChatBoostSource::GiftCode(User::new(1, "test", false))),
        ChannelChat::new(1, "test"),
    ));
}

#[test]
fn user_chat_boosts() {
    insta::assert_json_snapshot!(UserChatBoosts::from([ChatBoost::new(
        0,
        "id",
        0,
        ChatBoostSource::GiftCode(User::new(1, "test", false)),
    )]));
}

#[test]
fn get_user_chat_boosts() {
    assert_payload_eq(
        Payload::json(
            "getUserChatBoosts",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
            }),
        ),
        GetUserChatBoosts::new(1, 2),
    );
}

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

    insta::assert_json_snapshot!(expected_struct.clone());

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

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn group_chat_full_info() {
    let expected_struct = create_chat_full_info(ChatFullInfoType::Group);

    insta::assert_json_snapshot!(expected_struct.clone());

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

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn private_chat_full_info() {
    let expected_struct = create_chat_full_info(ChatFullInfoType::Private);

    insta::assert_json_snapshot!(expected_struct.clone());

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

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn supergroup_chat_full_info() {
    let expected_struct = create_chat_full_info(ChatFullInfoType::Supergroup);

    insta::assert_json_snapshot!(expected_struct.clone());

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

    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_id() {
    let chat_id = ChatId::from(1);
    if let ChatId::Id(chat_id) = chat_id {
        assert_eq!(chat_id, 1);
    } else {
        panic!("Unexpected chat id: {chat_id:?}");
    }
    assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#"1"#);
    assert_eq!(chat_id.to_string(), "1");

    let chat_id = ChatId::from("username");
    if let ChatId::Username(ref username) = chat_id {
        assert_eq!(username, "username");
    } else {
        panic!("Unexpected chat id: {chat_id:?}");
    }
    assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
    assert_eq!(chat_id.to_string(), "username");

    let chat_id = ChatId::from(String::from("username"));
    if let ChatId::Username(ref username) = chat_id {
        assert_eq!(username, "username");
    } else {
        panic!("Unexpected chat id: {chat_id:?}");
    }
    assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
    assert_eq!(chat_id.to_string(), "username");

    let mut map = HashMap::new();
    let chat_id_1 = ChatId::from(1);
    let chat_id_2 = ChatId::from("username");
    map.insert(chat_id_1.clone(), "1".to_string());
    map.insert(chat_id_2.clone(), "2".to_string());
    assert_eq!(map.get(&chat_id_1).unwrap(), "1");
    assert_eq!(map.get(&chat_id_2).unwrap(), "2");
}

#[test]
fn chat_invite_link() {
    insta::assert_json_snapshot!(
        ChatInviteLink::new("example.com/join/chat", User::new(1, "User", false))
            .with_creates_join_request(true)
            .with_is_primary(true)
            .with_is_revoked(false)
            .with_name("Link")
            .with_expire_date(0)
            .with_member_limit(10)
            .with_pending_join_request_count(0)
    );
    insta::assert_json_snapshot!(ChatInviteLink::new(
        "example.com/join/chat",
        User::new(1, "User", false)
    ));
}

#[test]
fn create_chat_invite_link() {
    let method = CreateChatInviteLink::new(1);
    assert_payload_eq(
        Payload::json(
            "createChatInviteLink",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "createChatInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "name": "Link",
                "expire_date": 0,
                "member_limit": 1,
                "creates_join_request": false
            }),
        ),
        method
            .with_name("Link")
            .with_expire_date(0)
            .with_member_limit(1)
            .with_creates_join_request(false),
    )
}

#[test]
fn create_chat_subscription_invite_link() {
    let method = CreateChatSubscriptionInviteLink::new(1, 2592000, 1);

    assert_payload_eq(
        Payload::json(
            "createChatSubscriptionInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "subscription_period": 2592000,
                "subscription_price": 1,
            }),
        ),
        method.clone(),
    );

    assert_payload_eq(
        Payload::json(
            "createChatSubscriptionInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "subscription_period": 2592000,
                "subscription_price": 1,
                "name": "test",
            }),
        ),
        method.with_name("test"),
    );
}

#[test]
fn edit_chat_invite_link() {
    let method = EditChatInviteLink::new(1, "example.com/join/chat");
    assert_payload_eq(
        Payload::json(
            "editChatInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "example.com/join/chat"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "editChatInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "example.com/join/chat",
                "name": "Link",
                "expire_date": 0,
                "member_limit": 1,
                "creates_join_request": false
            }),
        ),
        method
            .with_name("Link")
            .with_expire_date(0)
            .with_member_limit(1)
            .with_creates_join_request(false),
    );
}

#[test]
fn edit_chat_subscription_invite_link() {
    let method = EditChatSubscriptionInviteLink::new(1, "test");

    assert_payload_eq(
        Payload::json(
            "editChatSubscriptionInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "test",
            }),
        ),
        method.clone(),
    );

    assert_payload_eq(
        Payload::json(
            "editChatSubscriptionInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "test",
                "name": "test",
            }),
        ),
        method.with_name("test"),
    );
}

#[test]
fn export_chat_invite_link() {
    assert_payload_eq(
        Payload::json(
            "exportChatInviteLink",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        ExportChatInviteLink::new(1),
    );
}

#[test]
fn revoke_chat_invite_link() {
    assert_payload_eq(
        Payload::json(
            "revokeChatInviteLink",
            serde_json::json!({
                "chat_id": 1,
                "invite_link": "example.com/join/chat"
            }),
        ),
        RevokeChatInviteLink::new(1, "example.com/join/chat"),
    );
}

#[test]
fn chat_join_request() {
    insta::assert_json_snapshot!(
        ChatJoinRequest::new(
            Chat::Channel(ChannelChat::new(1, "Channel")),
            0,
            User::new(1, "User", false),
        )
        .with_bio("Bio")
        .with_invite_link(
            ChatInviteLink::new("example.com/join/channel", User::new(2, "User", false)).with_is_primary(true),
        )
        .with_user_chat_id(1)
    );
    insta::assert_json_snapshot!(ChatJoinRequest::new(
        Chat::Channel(ChannelChat::new(1, "Channel")),
        0,
        User::new(1, "User", false),
    ));
}

#[test]
fn approve_chat_join_request() {
    assert_payload_eq(
        Payload::json(
            "approveChatJoinRequest",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
            }),
        ),
        ApproveChatJoinRequest::new(1, 1),
    );
}

#[test]
fn decline_chat_join_request() {
    assert_payload_eq(
        Payload::json(
            "declineChatJoinRequest",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
            }),
        ),
        DeclineChatJoinRequest::new(1, 1),
    );
}

#[test]
fn chat_location() {
    insta::assert_json_snapshot!(ChatLocation::new("Address", Location::new(1.0, 0.0)));
}

#[test]
fn chat_member_admin() {
    let expected_struct = ChatMember::Administrator(
        ChatMemberAdministrator::new(
            User::new(1, "John", false)
                .with_last_name("Doe")
                .with_username("john_doe")
                .with_language_code("RU"),
        )
        .with_custom_title("Alpha")
        .with_is_anonymous(false)
        .with_can_be_edited(true)
        .with_can_change_info(false)
        .with_can_post_messages(true)
        .with_can_edit_messages(false)
        .with_can_delete_messages(true)
        .with_can_invite_users(false)
        .with_can_restrict_members(true)
        .with_can_pin_messages(false)
        .with_can_post_stories(true)
        .with_can_edit_stories(true)
        .with_can_promote_members(true)
        .with_can_manage_video_chats(false)
        .with_can_manage_chat(true)
        .with_can_manage_topics(true)
        .with_can_delete_stories(true),
    );
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = ChatMember::Administrator(
        ChatMemberAdministrator::new(User::new(1, "John", false))
            .with_can_be_edited(true)
            .with_can_delete_messages(true)
            .with_can_restrict_members(true)
            .with_can_promote_members(true)
            .with_can_manage_chat(true),
    );
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_member_creator() {
    let expected_struct = ChatMember::Creator(
        ChatMemberCreator::new(User::new(1, "John", false))
            .with_is_anonymous(false)
            .with_custom_title("Alpha"),
    );

    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = ChatMember::Creator(ChatMemberCreator::new(User::new(1, "John", false)));
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_member_kicked() {
    let expected_struct = ChatMember::Kicked(ChatMemberKicked::new(
        0,
        User::new(1, "John", false)
            .with_last_name("Doe")
            .with_language_code("RU")
            .with_username("john_doe"),
    ));
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_member_left() {
    let expected_struct = ChatMember::Left(User::new(1, "John", true));
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_member() {
    let expected_struct = ChatMember::Member {
        user: User::new(1, "John", false),
        until_date: None,
    };
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);

    let expected_struct = ChatMember::Member {
        user: User::new(1, "John", false),
        until_date: Some(1),
    };
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_member_restricted() {
    let expected_struct = ChatMember::Restricted(
        ChatMemberRestricted::new(User::new(1, "John", false), 0)
            .with_can_change_info(true)
            .with_can_invite_users(false)
            .with_can_send_polls(true)
            .with_can_pin_messages(false)
            .with_can_send_messages(true)
            .with_can_send_audios(true)
            .with_can_send_documents(false)
            .with_can_send_photos(true)
            .with_can_send_videos(false)
            .with_can_send_video_notes(true)
            .with_can_send_other_messages(true)
            .with_can_add_web_page_previews(false)
            .with_can_manage_topics(false)
            .with_is_member(true)
            .with_can_send_voice_notes(false),
    );
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = ChatMember::Restricted(
        ChatMemberRestricted::new(User::new(1, "John", true), 0)
            .with_can_change_info(true)
            .with_can_send_polls(true)
            .with_can_send_messages(true)
            .with_can_send_other_messages(true),
    );
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn chat_member_updated() {
    insta::assert_json_snapshot!(
        ChatMemberUpdated::new(
            Chat::Group(GroupChat::new(1, "group-title")),
            0,
            User::new(1, "John", true),
            ChatMember::Kicked(ChatMemberKicked::new(0, User::new(2, "John", false))),
            ChatMember::Member {
                user: User::new(2, "John", false),
                until_date: None,
            },
        )
        .with_invite_link(
            ChatInviteLink::new("https://t.me/joinchat/o8oIBrbCI3U2OGJi", User::new(1, "John", false))
                .with_is_primary(true),
        )
        .with_via_chat_folder_invite_link(true)
        .with_via_join_request(true)
    );
    insta::assert_json_snapshot!(ChatMemberUpdated::new(
        Chat::Group(GroupChat::new(1, "group-title")),
        0,
        User::new(1, "John", true),
        ChatMember::Kicked(ChatMemberKicked::new(0, User::new(2, "John", false))),
        ChatMember::Member {
            user: User::new(2, "John", false),
            until_date: None,
        },
    ));
}

#[test]
fn ban_chat_member() {
    let method = BanChatMember::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "banChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "banChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 3,
                "revoke_messages": true,
            }),
        ),
        method.with_until_date(3).with_revoke_messages(true),
    );
}

#[test]
fn get_chat_administrators() {
    assert_payload_eq(
        Payload::json(
            "getChatAdministrators",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChatAdministrators::new(1),
    );
}

#[test]
fn get_chat_member() {
    assert_payload_eq(
        Payload::json(
            "getChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        GetChatMember::new(1, 2),
    );
}

#[test]
fn get_chat_members_count() {
    assert_payload_eq(
        Payload::json(
            "getChatMemberCount",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChatMemberCount::new(1),
    );
}

#[test]
fn promote_chat_member() {
    assert_payload_eq(
        Payload::json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": true,
                "can_change_info": true,
                "can_delete_messages": true,
                "can_edit_messages": true,
                "can_invite_users": true,
                "can_manage_chat": true,
                "can_manage_video_chats": true,
                "can_pin_messages": true,
                "can_post_messages": true,
                "can_promote_members": true,
                "can_restrict_members": true,
                "can_manage_topics": true,
                "can_post_stories": true,
                "can_edit_stories": true,
                "can_delete_stories": true
            }),
        ),
        PromoteChatMember::new(1, 2).promote_all(),
    );
    assert_payload_eq(
        Payload::json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": false,
                "can_change_info": false,
                "can_delete_messages": false,
                "can_edit_messages": false,
                "can_invite_users": false,
                "can_manage_chat": false,
                "can_manage_video_chats": false,
                "can_pin_messages": false,
                "can_post_messages": false,
                "can_promote_members": false,
                "can_restrict_members": false,
                "can_manage_topics": false,
                "can_post_stories": false,
                "can_edit_stories": false,
                "can_delete_stories": false
            }),
        ),
        PromoteChatMember::new(1, 2).demote_all(),
    );
    assert_payload_eq(
        Payload::json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": false,
                "can_change_info": true,
                "can_delete_messages": false,
                "can_edit_messages": true,
                "can_invite_users": true,
                "can_manage_chat": false,
                "can_manage_video_chats": true,
                "can_pin_messages": true,
                "can_post_messages": false,
                "can_promote_members": false,
                "can_restrict_members": false,
                "can_manage_topics": true,
                "can_post_stories": true,
                "can_edit_stories": true,
                "can_delete_stories": true
            }),
        ),
        PromoteChatMember::new(1, 2)
            .with_is_anonymous(false)
            .with_can_change_info(true)
            .with_can_edit_messages(true)
            .with_can_delete_messages(false)
            .with_can_invite_users(true)
            .with_can_manage_chat(false)
            .with_can_manage_video_chats(true)
            .with_can_pin_messages(true)
            .with_can_post_messages(false)
            .with_can_promote_members(false)
            .with_can_restrict_members(false)
            .with_can_manage_topics(true)
            .with_can_post_stories(true)
            .with_can_edit_stories(true)
            .with_can_delete_stories(true),
    );
    assert_payload_eq(
        Payload::json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        PromoteChatMember::new(1, 2),
    );
}

#[test]
fn restrict_chat_member() {
    let method = RestrictChatMember::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "permissions": {}
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
                "permissions": {}
            }),
        ),
        method.with_until_date(100),
    );
    let method = RestrictChatMember::new(1, 2).allow_all();
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
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
                }
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
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
                }
            }),
        ),
        method.with_until_date(100),
    );
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
                "permissions": {
                    "can_send_messages": true,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": false
                },
                "use_independent_chat_permissions": true
            }),
        ),
        RestrictChatMember::new(1, 2)
            .with_permissions(
                ChatPermissions::default()
                    .with_can_send_messages(true)
                    .with_can_send_other_messages(true)
                    .with_can_add_web_page_previews(false),
            )
            .with_until_date(100)
            .with_use_independent_chat_permissions(true),
    );
    assert_payload_eq(
        Payload::json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "permissions": {}
            }),
        ),
        RestrictChatMember::new(1, 2),
    );
}

#[test]
fn set_chat_administrator_custom_title() {
    assert_payload_eq(
        Payload::json(
            "setChatAdministratorCustomTitle",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
                "custom_title": "Alpha"
            }),
        ),
        SetChatAdministratorCustomTitle::new(1, "Alpha", 1),
    );
}

#[test]
fn unban_chat_member() {
    let method = UnbanChatMember::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "unbanChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "unbanChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "only_if_banned": true
            }),
        ),
        method.with_only_if_banned(true),
    );
}

#[test]
fn pin_chat_message() {
    let method = PinChatMessage::new(1, 2);
    assert_payload_eq(
        Payload::json(
            "pinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "pinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "message_id": 2,
                "business_connection_id": "c-id",
                "disable_notification": true
            }),
        ),
        method
            .with_business_connection_id("c-id")
            .with_disable_notification(true),
    );
}

#[test]
fn unpin_chat_message() {
    let method = UnpinChatMessage::new(1);
    assert_payload_eq(
        Payload::json(
            "unpinChatMessage",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "unpinChatMessage",
            serde_json::json!({
                "chat_id": 1,
                "business_connection_id": "c-id",
                "message_id": 2
            }),
        ),
        method.with_business_connection_id("c-id").with_message_id(2),
    );
}

#[test]
fn unpin_all_chat_messages() {
    assert_payload_eq(
        Payload::json(
            "unpinAllChatMessages",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        UnpinAllChatMessages::new(1),
    );
}

#[test]
fn chat_administrator_rights() {
    insta::assert_json_snapshot!(ChatAdministratorRights::default());
    insta::assert_json_snapshot!(ChatAdministratorRights::all());
    insta::assert_json_snapshot!(
        ChatAdministratorRights::default()
            .with_can_change_info(true)
            .with_can_delete_messages(false)
            .with_can_delete_stories(false)
            .with_can_edit_messages(false)
            .with_can_edit_stories(true)
            .with_can_invite_users(false)
            .with_can_manage_chat(true)
            .with_can_manage_topics(true)
            .with_can_manage_video_chats(false)
            .with_can_pin_messages(true)
            .with_can_post_messages(true)
            .with_can_post_stories(false)
            .with_can_promote_members(true)
            .with_can_restrict_members(false)
            .with_is_anonymous(true)
    )
}

#[test]
fn chat_permissions() {
    insta::assert_json_snapshot!(
        ChatPermissions::default()
            .with_can_send_messages(true)
            .with_can_send_audios(false)
            .with_can_send_documents(true)
            .with_can_send_photos(false)
            .with_can_send_videos(true)
            .with_can_send_video_notes(false)
            .with_can_send_voice_notes(true)
            .with_can_send_polls(true)
            .with_can_send_other_messages(false)
            .with_can_add_web_page_previews(true)
            .with_can_change_info(false)
            .with_can_invite_users(true)
            .with_can_pin_messages(false)
            .with_can_manage_topics(true),
    );
    insta::assert_json_snapshot!(ChatPermissions::default());
    insta::assert_json_snapshot!(ChatPermissions::allowed(),);
    insta::assert_json_snapshot!(ChatPermissions::restricted(),);
}

#[test]
fn set_chat_permissions() {
    let permissions = ChatPermissions::default().with_can_send_messages(true);
    let method = SetChatPermissions::new(1, permissions);
    assert_payload_eq(
        Payload::json(
            "setChatPermissions",
            serde_json::json!({
                "chat_id": 1,
                "permissions": {
                    "can_send_messages": true
                }
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setChatPermissions",
            serde_json::json!({
                "chat_id": 1,
                "permissions": {
                    "can_send_messages": true
                },
                "use_independent_chat_permissions": true
            }),
        ),
        method.with_use_independent_chat_permissions(true),
    );
}

#[test]
fn chat_photo() {
    insta::assert_json_snapshot!(ChatPhoto::new(
        "big-file-id",
        "big-file-unique-id",
        "small-file-id",
        "small-file-unique-id",
    ));
}

#[test]
fn delete_chat_photo() {
    assert_payload_eq(
        Payload::json(
            "deleteChatPhoto",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        DeleteChatPhoto::new(1),
    );
}

#[test]
fn set_chat_photo() {
    assert_payload_eq(
        Payload::form(
            "setChatPhoto",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("photo", InputFile::file_id("photo-id").into()),
            ]),
        ),
        SetChatPhoto::new(1, InputFile::file_id("photo-id")),
    );
}

#[test]
fn ban_chat_sender_chat() {
    assert_payload_eq(
        Payload::json(
            "banChatSenderChat",
            serde_json::json!({
                "chat_id": 1,
                "sender_chat_id": 1,
            }),
        ),
        BanChatSenderChat::new(1, 1),
    );
}

#[test]
fn unban_chat_sender_chat() {
    assert_payload_eq(
        Payload::json(
            "unbanChatSenderChat",
            serde_json::json!({
                "chat_id": 1,
                "sender_chat_id": 1,
            }),
        ),
        UnbanChatSenderChat::new(1, 1),
    );
}

#[test]
fn delete_chat_sticker_set() {
    assert_payload_eq(
        Payload::json(
            "deleteChatStickerSet",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        DeleteChatStickerSet::new(1),
    );
}

#[test]
fn set_chat_sticker_set() {
    assert_payload_eq(
        Payload::json(
            "setChatStickerSet",
            serde_json::json!({"chat_id": 1, "sticker_set_name": "Sticker Set"}),
        ),
        SetChatStickerSet::new(1, "Sticker Set"),
    );
}

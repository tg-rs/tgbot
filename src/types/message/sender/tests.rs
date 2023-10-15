use crate::types::{
    tests::assert_json_eq,
    ChannelChat,
    Chat,
    GroupChat,
    Message,
    MessageData,
    MessageSender,
    PrivateChat,
    SupergroupChat,
    User,
};

fn create_message_struct(chat: Chat) -> Message {
    Message {
        id: 1,
        date: 0,
        edit_date: None,
        sender: MessageSender::Unknown,
        chat,
        author_signature: None,
        has_protected_content: false,
        forward: None,
        is_automatic_forward: false,
        is_topic_message: None,
        message_thread_id: None,
        reply_to: None,
        via_bot: None,
        media_group_id: None,
        reply_markup: None,
        data: MessageData::Empty,
    }
}

#[test]
fn channel_chat() {
    let chat = Chat::Channel(ChannelChat {
        id: 1,
        title: String::from("Channel"),
        username: None,
        photo: None,
        description: None,
        invite_link: None,
        pinned_message: None,
        linked_chat_id: None,
        has_protected_content: None,
        message_auto_delete_time: None,
    });
    let mut expected_struct = create_message_struct(chat);
    expected_struct.author_signature = Some(String::from("test"));
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_none());
    assert!(expected_struct.sender.get_user_id().is_none());
    assert!(expected_struct.sender.get_user_username().is_none());
    let expected_value = serde_json::json!({
        "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "Channel"
            },
            "has_protected_content": false,
            "is_automatic_forward": false
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn group_chat() {
    let chat = Chat::Group(GroupChat {
        id: 1,
        title: String::from("Group"),
        photo: None,
        invite_link: None,
        pinned_message: None,
        permissions: None,
        has_protected_content: None,
        message_auto_delete_time: None,
    });
    let mut expected_struct = create_message_struct(chat.clone());
    expected_struct.sender = MessageSender::User(User {
        id: 1,
        is_bot: false,
        first_name: String::from("User"),
        last_name: None,
        username: Some(String::from("test")),
        language_code: None,
        is_premium: None,
        added_to_attachment_menu: None,
    });
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id(), Some(1));
    assert_eq!(expected_struct.sender.get_user_username(), Some("test"));

    let expected_value = serde_json::json!({
        "message_id": 1,
        "date": 0,
        "from": {"id": 1, "first_name": "User", "is_bot": false, "username": "test"},
        "chat": {"id": 1, "type": "group", "title": "Group"},
        "has_protected_content": false,
        "is_automatic_forward": false
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.sender = MessageSender::Chat(chat.clone());
    assert_eq!(expected_struct.sender.get_chat().unwrap(), &chat);
    let expected_value = serde_json::json!({
        "message_id": 1,
        "date": 0,
        "chat": {"id": 1, "type": "group", "title": "Group"},
        "sender_chat": {
            "id": 1,
            "type": "group",
            "title": "Group"
        },
        "has_protected_content": false,
        "is_automatic_forward": false
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn private_chat() {
    let chat = Chat::Private(PrivateChat {
        id: 1,
        first_name: String::from("Target"),
        last_name: None,
        username: None,
        photo: None,
        bio: None,
        pinned_message: None,
        has_private_forwards: None,
        message_auto_delete_time: None,
        has_restricted_voice_and_video_messages: None,
    });
    let mut expected_struct = create_message_struct(chat);
    expected_struct.sender = MessageSender::User(User {
        id: 1,
        is_bot: false,
        first_name: String::from("Target"),
        last_name: None,
        username: None,
        language_code: None,
        is_premium: None,
        added_to_attachment_menu: None,
    });
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id(), Some(1));
    assert!(expected_struct.sender.get_user_username().is_none());
    let expected_value = serde_json::json!({
        "message_id": 1,
        "date": 0,
        "from": {"id": 1, "first_name": "Target", "is_bot": false},
        "chat": {"id": 1, "type": "private", "first_name": "Target"},
        "has_protected_content": false,
        "is_automatic_forward": false
    });
    assert_json_eq(expected_struct, expected_value);
}

#[test]
fn supergroup_chat() {
    let chat = Chat::Supergroup(SupergroupChat {
        id: 1,
        title: String::from("Chat"),
        username: None,
        photo: None,
        description: None,
        invite_link: None,
        pinned_message: None,
        sticker_set_name: None,
        can_set_sticker_set: None,
        permissions: None,
        slow_mode_delay: None,
        message_auto_delete_time: None,
        linked_chat_id: None,
        location: None,
        has_protected_content: None,
        join_to_send_messages: None,
        join_by_request: None,
        is_forum: None,
    });
    let mut expected_struct = create_message_struct(chat);
    expected_struct.sender = MessageSender::User(User {
        id: 1,
        is_bot: false,
        first_name: String::from("User"),
        last_name: None,
        username: None,
        language_code: None,
        is_premium: None,
        added_to_attachment_menu: None,
    });
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id(), Some(1));
    assert!(expected_struct.sender.get_user_username().is_none());
    let expected_value = serde_json::json!({
        "message_id": 1,
        "date": 0,
        "from": {"id": 1, "first_name": "User", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "Chat"},
        "has_protected_content": false,
        "is_automatic_forward": false
    });
    assert_json_eq(expected_struct, expected_value);
}

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
    Message::new(
        1,
        0,
        chat,
        MessageData::Unknown(serde_json::json!({})),
        MessageSender::Unknown,
    )
}

#[test]
fn channel_chat() {
    let chat = Chat::Channel(ChannelChat::new(1, "Channel"));
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
    let chat = Chat::Group(GroupChat::new(1, "Group"));
    let mut expected_struct = create_message_struct(chat.clone());
    expected_struct.sender = User::new(1, "User", false).with_username("test").into();
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id().unwrap(), 1);
    assert_eq!(expected_struct.sender.get_user_username().unwrap(), "test");

    let expected_value = serde_json::json!({
        "message_id": 1,
        "date": 0,
        "from": {"id": 1, "first_name": "User", "is_bot": false, "username": "test"},
        "chat": {"id": 1, "type": "group", "title": "Group"},
        "has_protected_content": false,
        "is_automatic_forward": false
    });
    assert_json_eq(expected_struct.clone(), expected_value.clone());

    expected_struct.sender = chat.clone().into();
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
    let chat = Chat::Private(PrivateChat::new(1, "Target"));
    let mut expected_struct = create_message_struct(chat);
    expected_struct.sender = User::new(1, "Target", false).into();
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id().unwrap(), 1);
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
    let chat = Chat::Supergroup(SupergroupChat::new(1, "Chat"));
    let mut expected_struct = create_message_struct(chat);
    expected_struct.sender = User::new(1, "User", false).into();
    assert_eq!(expected_struct.chat.get_id(), 1);
    assert!(expected_struct.chat.get_username().is_none());
    assert!(expected_struct.sender.get_user().is_some());
    assert_eq!(expected_struct.sender.get_user_id().unwrap(), 1);
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

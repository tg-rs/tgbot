use crate::types::{Chat, Message};

#[test]
fn channel_chat() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1111,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": 7,
                "type": "channel",
                "title": "channeltitle"
            },
            "text": "test message from channel"
    }))
    .unwrap();
    assert_eq!(msg.chat.get_id(), 7);
    assert!(msg.chat.get_username().is_none());
    assert!(msg.sender.get_user().is_none());
    assert!(msg.sender.get_user_id().is_none());
    assert!(msg.sender.get_user_username().is_none());
}

#[test]
fn group_chat() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false, "username": "test"},
        "chat": {"id": 5, "type": "group", "title": "grouptitle"},
        "text": "test"
    }))
    .unwrap();
    assert_eq!(msg.chat.get_id(), 5);
    assert!(msg.chat.get_username().is_none());
    assert!(msg.sender.get_user().is_some());
    assert_eq!(msg.sender.get_user_id(), Some(1));
    assert_eq!(msg.sender.get_user_username(), Some("test"));
}

#[test]
fn group_chat_with_sender_chat() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1111,
        "date": 0,
        "author_signature": "test",
        "chat": {
            "id": 6,
            "type": "group",
            "title": "grouptitle"
        },
        "sender_chat": {
            "id": 6,
            "type": "channel",
            "title": "channeltitle",
            "username": "channelusername"
        },
        "is_automatic_forward": true,
        "text": "test message from channel"
    }))
    .unwrap();
    assert_eq!(msg.chat.get_id(), 6);
    assert!(msg.chat.get_username().is_none());
    assert!(msg.sender.get_user().is_none());
    assert!(msg.sender.get_user_id().is_none());
    assert!(msg.sender.get_user_username().is_none());
    assert!(msg.is_automatic_forward);
    if let Chat::Channel(chat) = msg.sender.get_chat().as_ref().unwrap() {
        assert_eq!(chat.id, 6);
    } else {
        panic!("Unexpected sender: {:?}", msg.sender);
    }
}

#[test]
fn private_chat() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 3, "type": "private", "first_name": "firstname"},
        "text": "test"
    }))
    .unwrap();
    assert_eq!(msg.chat.get_id(), 3);
    assert!(msg.chat.get_username().is_none());
    assert!(msg.sender.get_user().is_some());
    assert_eq!(msg.sender.get_user_id(), Some(1));
    assert!(msg.sender.get_user_username().is_none());
}

#[test]
fn supergroup_chat() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "has_protected_content": true,
        "text": "test"
    }))
    .unwrap();
    assert_eq!(msg.chat.get_id(), 1);
    assert!(msg.chat.get_username().is_none());
    assert!(msg.sender.get_user().is_some());
    assert_eq!(msg.sender.get_user_id(), Some(1));
    assert!(msg.sender.get_user_username().is_none());
    assert!(msg.has_protected_content);
}

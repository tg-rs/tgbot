use crate::types::Chat;

#[test]
fn group_chat_deserialize_full() {
    let chat: Chat = serde_json::from_value(serde_json::json!({
        "id": 1,
        "type": "group",
        "title": "grouptitle",
        "photo": {
            "small_file_id": "smallfileid",
            "small_file_unique_id": "smallfileuniqueid",
            "big_file_id": "bigfileid",
            "big_file_unique_id": "bigfileuniqueid",
        },
        "invite_link": "groupinvitelink",
        "pinned_message": {
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "user"
            },
            "text": "test"
        },
        "permissions": {"can_send_messages": true},
        "has_protected_content": true,
        "message_auto_delete_time": 86400
    }))
    .unwrap();
    assert_eq!(chat.get_id(), 1);
    assert!(chat.get_username().is_none());
    if let Chat::Group(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, "grouptitle");
        let photo = chat.photo.unwrap();
        assert_eq!(photo.small_file_id, "smallfileid");
        assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
        assert_eq!(photo.big_file_id, "bigfileid");
        assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
        assert_eq!(chat.invite_link.unwrap(), "groupinvitelink");
        let permissions = chat.permissions.unwrap();
        assert!(permissions.can_send_messages.unwrap());
        assert!(chat.pinned_message.is_some());
        assert!(chat.has_protected_content.unwrap());
        assert_eq!(chat.message_auto_delete_time.unwrap(), 86400)
    } else {
        panic!("Unexpected chat: {:?}", chat);
    }
}

#[test]
fn group_chat_deserialize_partial() {
    let chat: Chat = serde_json::from_value(serde_json::json!({
        "id": 1,
        "type": "group",
        "title": "grouptitle"
    }))
    .unwrap();
    assert_eq!(chat.get_id(), 1);
    assert!(chat.get_username().is_none());
    if let Chat::Group(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, "grouptitle");
        assert!(chat.photo.is_none());
        assert!(chat.invite_link.is_none());
        assert!(chat.pinned_message.is_none());
        assert!(chat.permissions.is_none());
    } else {
        panic!("Unexpected chat: {:?}", chat);
    }
}

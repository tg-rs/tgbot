use crate::types::Chat;

#[test]
fn channel_chat_deserialize_full() {
    let chat: Chat = serde_json::from_value(serde_json::json!({
        "id": 1,
        "type": "channel",
        "title": "channeltitle",
        "username": "channelusername",
        "photo": {
            "small_file_id": "smallfileid",
            "small_file_unique_id": "smallfileuniqueid",
            "big_file_id": "bigfileid",
            "big_file_unique_id": "bigfileuniqueid",
        },
        "description": "channeldescription",
        "invite_link": "channelinvitelink",
        "pinned_message": {
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "channeltitle"
            },
            "text": "test"
        },
        "linked_chat_id": 2,
        "has_protected_content": true,
        "message_auto_delete_time": 86400
    }))
    .unwrap();
    assert_eq!(chat.get_id(), 1);
    assert_eq!(chat.get_username().unwrap(), "channelusername");
    if let Chat::Channel(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, "channeltitle");
        assert_eq!(chat.username.unwrap(), "channelusername");
        let photo = chat.photo.unwrap();
        assert_eq!(photo.small_file_id, "smallfileid");
        assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
        assert_eq!(photo.big_file_id, "bigfileid");
        assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
        assert_eq!(chat.description.unwrap(), "channeldescription");
        assert_eq!(chat.invite_link.unwrap(), "channelinvitelink");
        assert!(chat.pinned_message.is_some());
        assert_eq!(chat.linked_chat_id.unwrap(), 2);
        assert!(chat.has_protected_content.unwrap());
        assert_eq!(chat.message_auto_delete_time.unwrap(), 86400);
    } else {
        panic!("Unexpected chat: {:?}", chat);
    }
}

#[test]
fn channel_chat_deserialize_partial() {
    let chat: Chat = serde_json::from_value(serde_json::json!({
        "id": 1,
        "type": "channel",
        "title": "channeltitle"
    }))
    .unwrap();
    assert_eq!(chat.get_id(), 1);
    assert!(chat.get_username().is_none());
    if let Chat::Channel(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, "channeltitle");
        assert!(chat.username.is_none());
        assert!(chat.photo.is_none());
        assert!(chat.description.is_none());
        assert!(chat.invite_link.is_none());
        assert!(chat.pinned_message.is_none());
    } else {
        panic!("Unexpected chat: {:?}", chat);
    }
}

use crate::types::Chat;

#[test]
fn private_chat_deserialize_full() {
    let chat: Chat = serde_json::from_value(serde_json::json!({
        "id": 1,
        "type": "private",
        "username": "testusername",
        "first_name": "testfirstname",
        "last_name": "testlastname",
        "photo": {
            "small_file_id": "smallfileid",
            "small_file_unique_id": "smallfileuniqueid",
            "big_file_id": "bigfileid",
            "big_file_unique_id": "bigfileuniqueid",
        },
        "bio": "testbio",
        "pinned_message": {
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 2,
                "type": "private",
                "first_name": "test"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "user"
            },
            "text": "test"
        },
        "has_private_forwards": true,
        "message_auto_delete_time": 86400
    }))
    .unwrap();
    assert_eq!(chat.get_id(), 1);
    assert_eq!(chat.get_username().unwrap(), "testusername");
    if let Chat::Private(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.username.unwrap(), "testusername");
        assert_eq!(chat.first_name, "testfirstname");
        assert_eq!(chat.last_name.unwrap(), "testlastname");
        let photo = chat.photo.unwrap();
        assert_eq!(photo.small_file_id, "smallfileid");
        assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
        assert_eq!(photo.big_file_id, "bigfileid");
        assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
        assert_eq!(chat.bio.unwrap(), "testbio");
        assert_eq!(chat.pinned_message.unwrap().id, 1);
        assert!(chat.has_private_forwards.unwrap());
        assert_eq!(chat.message_auto_delete_time.unwrap(), 86400);
    } else {
        panic!("Unexpected chat: {:?}", chat)
    }
}

#[test]
fn private_chat_deserialize_partial() {
    let chat: Chat = serde_json::from_value(serde_json::json!({
        "id": 1,
        "type": "private",
        "first_name": "testfirstname"
    }))
    .unwrap();
    assert_eq!(chat.get_id(), 1);
    assert!(chat.get_username().is_none());
    if let Chat::Private(chat) = chat {
        assert_eq!(chat.id, 1);
        assert!(chat.username.is_none());
        assert_eq!(chat.first_name, "testfirstname");
        assert!(chat.last_name.is_none());
        assert!(chat.photo.is_none());
    } else {
        panic!("Unexpected chat: {:?}", chat)
    }
}

use crate::types::Chat;

#[test]
fn supergroup_deserialize_full() {
    let chat: Chat = serde_json::from_value(serde_json::json!({
        "id": 1,
        "type": "supergroup",
        "title": "supergrouptitle",
        "username": "supergroupusername",
        "photo": {
            "small_file_id": "smallfileid",
            "small_file_unique_id": "smallfileuniqueid",
            "big_file_id": "bigfileid",
            "big_file_unique_id": "bigfileuniqueid",
        },
        "description": "supergroupdescription",
        "invite_link": "supergroupinvitelink",
        "sticker_set_name": "supergroupstickersetname",
        "can_set_sticker_set": true,
        "slow_mode_delay": 10,
        "permissions": {
            "can_send_messages": true
        },
        "pinned_message": {
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "type": "supergroup",
                "title": "supergrouptitle",
                "username": "supergroupusername"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "user"
            },
            "text": "test"
        },
        "linked_chat_id": 2,
        "location": {
            "location": {
                "longitude": 0,
                "latitude": 1
            },
            "address": "test location"
        },
        "has_protected_content": true,
        "message_auto_delete_time": 86400
    }))
    .unwrap();
    assert_eq!(chat.get_id(), 1);
    assert_eq!(chat.get_username().unwrap(), "supergroupusername");
    if let Chat::Supergroup(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, "supergrouptitle");
        assert_eq!(chat.username.unwrap(), "supergroupusername");
        let photo = chat.photo.unwrap();
        assert_eq!(photo.small_file_id, "smallfileid");
        assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
        assert_eq!(photo.big_file_id, "bigfileid");
        assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
        assert_eq!(chat.description.unwrap(), "supergroupdescription");
        assert_eq!(chat.invite_link.unwrap(), "supergroupinvitelink");
        assert_eq!(chat.sticker_set_name.unwrap(), "supergroupstickersetname");
        assert_eq!(chat.slow_mode_delay.unwrap(), 10);
        assert!(chat.can_set_sticker_set.unwrap());
        assert!(chat.pinned_message.is_some());
        let permissions = chat.permissions.unwrap();
        assert!(permissions.can_send_messages.unwrap());
        assert_eq!(chat.linked_chat_id.unwrap(), 2);
        assert_eq!(chat.location.unwrap().address, "test location");
        assert!(chat.has_protected_content.unwrap());
        assert_eq!(chat.message_auto_delete_time.unwrap(), 86400);
    } else {
        panic!("Unexpected chat: {:?}", chat)
    }
}

#[test]
fn supergroup_deserialize_partial() {
    let chat: Chat = serde_json::from_value(serde_json::json!({
        "id": 1,
        "type": "supergroup",
        "title": "supergrouptitle"
    }))
    .unwrap();
    assert_eq!(chat.get_id(), 1);
    assert!(chat.get_username().is_none());
    if let Chat::Supergroup(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, "supergrouptitle");
        assert!(chat.username.is_none());
        assert!(chat.photo.is_none());
        assert!(chat.description.is_none());
        assert!(chat.invite_link.is_none());
        assert!(chat.sticker_set_name.is_none());
        assert!(chat.can_set_sticker_set.is_none());
        assert!(chat.pinned_message.is_none());
        assert!(chat.permissions.is_none());
    } else {
        panic!("Unexpected chat: {:?}", chat)
    }
}

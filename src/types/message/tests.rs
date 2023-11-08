use crate::types::{tests::assert_json_eq, EditMessageResult, Message, MessageData, SupergroupChat, User};

#[test]
fn get_text() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "text"
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "text");
}

#[test]
fn get_text_from_audio() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test audio caption",
        "audio": {
            "file_id": "file-id",
            "file_unique_id": "unique-id",
            "duration": 243
        }
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test audio caption");
}

#[test]
fn get_text_from_document() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test document caption",
        "document": {
            "file_id": "file-id",
            "file_unique_id": "unique-id",
        }
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test document caption");
}

#[test]
fn get_text_from_photo() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test photo caption",
        "photo": [{
            "file_id": "photo-id",
            "file_unique_id": "unique-id",
            "width": 200,
            "height": 200
        }]
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test photo caption");
}

#[test]
fn get_text_from_video() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test video caption",
        "video": {
            "file_id": "video-id",
            "file_unique_id": "unique-id",
            "width": 1,
            "height": 2,
            "duration": 3
        }
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test video caption");
}

#[test]
fn get_text_from_voice() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "caption": "test voice caption",
        "voice": {
            "file_id": "voice-id",
            "file_unique_id": "unique-id",
            "duration": 123
        }
    }))
    .unwrap();
    assert_eq!(msg.get_text().unwrap().data, "test voice caption");
}

#[test]
fn get_text_returns_none() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "group_chat_created": true
    }))
    .unwrap();
    assert!(msg.get_text().is_none());
}

#[test]
fn is_edited() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "edit_date": 1
    }))
    .unwrap();
    assert!(msg.is_edited());

    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test"
    }))
    .unwrap();
    assert!(!msg.is_edited());
}

#[test]
fn reply_to() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "reply_to_message": {
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
            "text": "test"
        }
    }))
    .unwrap();
    if let Some(msg) = msg.reply_to {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected reply_to data: {:?}", msg.reply_to);
    }
}

#[test]
fn reply_to_with_empty_data() {
    let data: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "reply_to_message": {
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        }
    }))
    .unwrap();
    assert!(data.reply_to.is_some());
}

#[test]
fn via_bot() {
    let data: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "via_bot": {
            "id": 3,
            "is_bot": true,
            "first_name": "example",
            "last_name": "bot",
            "username": "example_bot"
        }
    }))
    .unwrap();
    let bot = data.via_bot.expect("via_bot is empty");
    assert_eq!(bot.id, 3);
    assert_eq!(bot.first_name, "example");
    assert_eq!(bot.last_name.unwrap(), "bot");
    assert_eq!(bot.username.unwrap(), "example_bot");
}

#[test]
fn edit_message_result() {
    let expected_struct = EditMessageResult::Message(Message::new(
        1,
        0,
        SupergroupChat::new(1, "test"),
        MessageData::Unknown(serde_json::json!({})),
        User::new(1, "test", false),
    ));
    let expected_value = serde_json::json!({
        "message_id": 1,
        "date": 0,
        "from": {"id": 1, "first_name": "test", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "test"},
        "has_protected_content": false,
        "is_automatic_forward": false
    });
    assert_json_eq(expected_struct, expected_value);

    let expected_struct = EditMessageResult::Bool(true);
    let expected_value = serde_json::json!(true);
    assert_json_eq(expected_struct, expected_value);
}

use crate::types::{EditMessageResult, Message};

#[test]
fn get_text() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "caption": "test audio caption",
        "audio": {
            "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "caption": "test document caption",
        "document": {
            "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA",
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "edit_date": 1
    }))
    .unwrap();
    assert!(msg.is_edited());

    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "reply_to_message": {
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "reply_to_message": {
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
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
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "via_bot": {
            "id": 3,
            "is_bot": true,
            "first_name": "example",
            "last_name": "bot",
            "username": "examplebot"
        }
    }))
    .unwrap();
    let bot = data.via_bot.expect("via_bot is empty");
    assert_eq!(bot.id, 3);
    assert_eq!(bot.first_name, "example");
    assert_eq!(bot.last_name.unwrap(), "bot");
    assert_eq!(bot.username.unwrap(), "examplebot");
}

#[test]
fn deserialize_edit_message_result() {
    let result: EditMessageResult = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "text"
    }))
    .unwrap();
    assert!(matches!(result, EditMessageResult::Message(_)));
    let result: EditMessageResult = serde_json::from_value(serde_json::json!(true)).unwrap();
    assert!(matches!(result, EditMessageResult::Bool(true)));
}

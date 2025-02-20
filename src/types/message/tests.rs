use crate::types::{
    EditMessageResult,
    InaccessibleMessage,
    LinkPreviewOptions,
    MaybeInaccessibleMessage,
    Message,
    MessageData,
    ReplyTo,
    SupergroupChat,
    User,
    tests::assert_json_eq,
};

#[test]
fn maybe_inaccesible() {
    let msg: MaybeInaccessibleMessage = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 0,
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
    }))
    .unwrap();
    match msg {
        MaybeInaccessibleMessage::InaccessibleMessage(InaccessibleMessage { chat, message_id }) => {
            assert_eq!(chat.get_id(), 1);
            assert_eq!(message_id, 1);
        }
        MaybeInaccessibleMessage::Message(_) => panic!("Unexpected message variant"),
    };

    let msg: MaybeInaccessibleMessage = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
    }))
    .unwrap();
    match msg {
        MaybeInaccessibleMessage::InaccessibleMessage(_) => panic!("Unexpected message variant"),
        MaybeInaccessibleMessage::Message(msg) => {
            assert_eq!(msg.id, 1);
        }
    };
}

#[test]
fn external_reply() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "text",
        "external_reply": {
            "origin": {
                "type": "hidden_user",
                "date": 1,
                "sender_user_name": "test"
            },
            "animation": {
                "duration": 10,
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
                "height": 20,
                "width": 30
            }
        }
    }))
    .unwrap();
    assert!(msg.external_reply.is_some());
}

#[test]
fn quote() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "text",
        "quote": {
            "position": 0,
            "text": "test"
        }
    }))
    .unwrap();
    assert!(msg.quote.is_some());
}

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
fn link_peview_options() {
    let data: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "link_preview_options": {
            "is_disabled": true
        }
    }))
    .unwrap();
    let options = data
        .link_preview_options
        .as_ref()
        .expect("link_preview_options is empty");
    assert!(options.is_disabled.unwrap());
    let data = data.with_link_preview_options(LinkPreviewOptions::default());
    assert!(data.link_preview_options.as_ref().unwrap().is_disabled.is_none());
}

#[test]
fn reply_to_message() {
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
    if let Some(ReplyTo::Message(msg)) = msg.reply_to {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected reply_to data: {:?}", msg.reply_to);
    }
}

#[test]
fn reply_to_message_with_empty_data() {
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
fn reply_to_story() {
    let msg: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "reply_to_story": {
            "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
            "id": 1
        }
    }))
    .unwrap();
    if let Some(ReplyTo::Story(story)) = msg.reply_to {
        assert_eq!(story.id, 1);
    } else {
        panic!("Unexpected reply_to data: {:?}", msg.reply_to);
    }
}

#[test]
fn sender_boost_count() {
    let data: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "test",
        "sender_boost_count": 1
    }))
    .unwrap();
    assert_eq!(data.sender_boost_count.unwrap(), 1);
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
    assert_eq!(&bot.username.unwrap(), "example_bot");
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

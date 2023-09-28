use crate::types::{Forward, ForwardFrom, Message};

#[test]
fn deserialize_forward_from_user() {
    let input = serde_json::json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "forward_from": {"id": 2, "first_name": "firstname", "is_bot": false},
        "forward_date": 0
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let Some(Forward {
        date,
        from: ForwardFrom::User(user),
    }) = msg.forward
    {
        assert_eq!(date, 0);
        assert_eq!(user.id, 2);
        assert_eq!(user.first_name, String::from("firstname"));
        assert!(!user.is_bot);
    } else {
        panic!("Unexpected forward data: {:?}", msg.forward);
    }
}

#[test]
fn deserialize_forward_from_hidden_user() {
    let input = serde_json::json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "forward_sender_name": "Hidden User",
        "forward_date": 0
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let Some(Forward {
        date,
        from: ForwardFrom::HiddenUser(name),
    }) = msg.forward
    {
        assert_eq!(date, 0);
        assert_eq!(name, String::from("Hidden User"));
    } else {
        panic!("Unexpected forward data: {:?}", msg.forward);
    }
}

#[test]
fn deserialize_forward_from_channel() {
    let input = serde_json::json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "forward_from_chat": {"id": 1, "type": "channel", "title": "test"},
        "forward_from_message_id": 1,
        "forward_signature": "test",
        "forward_date": 0
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let Some(Forward {
        date,
        from: ForwardFrom::Channel {
            chat,
            message_id,
            signature,
        },
    }) = msg.forward
    {
        assert_eq!(date, 0);
        assert_eq!(message_id, 1);
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, String::from("test"));
        assert_eq!(signature, Some(String::from("test")));
    } else {
        panic!("Unexpected forward data: {:?}", msg.forward);
    }
}

use crate::types::{Message, Text};

#[test]
fn get_bot_commands() {
    let input = serde_json::json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
        "text": "/command1 /command2 /command3@botname",
        "entities": [
            {"type": "bot_command", "offset": 0, "length": 9},
            {"type": "bot_command", "offset": 10, "length": 9},
            {"type": "bot_command", "offset": 20, "length": 17},
        ]
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    let commands = msg.get_text().and_then(|text| text.get_bot_commands()).unwrap();
    assert_eq!(commands.len(), 3);
    assert_eq!(commands[0].command, "/command1");
    assert!(commands[0].bot_name.is_none());
    assert_eq!(commands[1].command, "/command2");
    assert!(commands[1].bot_name.is_none());
    assert_eq!(commands[2].command, "/command3");
    assert_eq!(commands[2].bot_name.as_ref().unwrap(), "botname");
}

#[test]
fn traits() {
    let text = Text::from(String::from("test"));
    assert_eq!(text, String::from("test"));
    assert_eq!(text, *"test");
    assert_eq!(text.as_ref(), "test");
}

use crate::types::{Command, Message};

fn create_command(command: &str) -> Command {
    let len = command.split_whitespace().next().unwrap().len();
    let message: Message = serde_json::from_value(serde_json::json!(
        {
            "message_id": 1111,
            "date": 0,
            "from": {"id": 1, "is_bot": false, "first_name": "test"},
            "chat": {"id": 1, "type": "private", "first_name": "test"},
            "text": command,
            "entities": [
                {"type": "bot_command", "offset": 0, "length": len}
            ]
        }
    ))
    .unwrap();
    Command::try_from(message).unwrap()
}

#[test]
fn command() {
    let command = create_command("/test_command 'arg1 v' arg2");
    assert_eq!(command.get_name(), "/test_command");
    assert_eq!(command.get_args(), &["arg1 v", "arg2"]);
    assert_eq!(command.get_message().id, 1111);
}

#[test]
fn command_no_args() {
    let command = create_command("/test_command");
    assert_eq!(command.get_name(), "/test_command");
    assert!(command.get_args().is_empty());
    assert_eq!(command.get_message().id, 1111);
}

#[test]
fn command_bot_suffix() {
    let command = create_command("/test_command@bot 'arg1 v' arg2");
    assert_eq!(command.get_name(), "/test_command");
    assert_eq!(command.get_args(), &["arg1 v", "arg2"]);
    assert_eq!(command.get_message().id, 1111);
}

#[test]
fn command_bot_suffix_no_args() {
    let command = create_command("/test_command@abc");
    assert_eq!(command.get_name(), "/test_command");
    assert!(command.get_args().is_empty());
    assert_eq!(command.get_message().id, 1111);
}

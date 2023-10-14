use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        Bot,
        BotCommand,
        BotCommandScope,
        Close,
        DeleteBotCommands,
        GetBot,
        GetBotCommands,
        LogOut,
        SetBotCommands,
    },
};

#[test]
fn bot() {
    assert_json_eq(
        Bot {
            can_join_groups: true,
            can_read_all_group_messages: true,
            first_name: String::from("Loo"),
            id: 1,
            last_name: Some(String::from("Maclin")),
            supports_inline_queries: false,
            username: String::from("loo_maclin_bot"),
        },
        serde_json::json!({
            "can_join_groups": true,
            "can_read_all_group_messages": true,
            "first_name": "Loo",
            "id": 1,
            "last_name": "Maclin",
            "supports_inline_queries": false,
            "username": "loo_maclin_bot",
        }),
    );
}

#[test]
fn bot_command() {
    let err = BotCommand::new("", "description").unwrap_err().to_string();
    assert_eq!(err, "command name can have a length of 1 up to 32 characters, got 0");
    let err = BotCommand::new("2".repeat(33), "description").unwrap_err().to_string();
    assert_eq!(err, "command name can have a length of 1 up to 32 characters, got 33");
    let err = BotCommand::new("name", "d").unwrap_err().to_string();
    assert_eq!(
        err,
        "command description can have a length of 3 up to 256 characters, got 1"
    );
    let err = BotCommand::new("name", "d".repeat(257)).unwrap_err().to_string();
    assert_eq!(
        err,
        "command description can have a length of 3 up to 256 characters, got 257"
    );

    let bot_command = BotCommand::new("name", "description").unwrap();
    assert_eq!(bot_command.name(), "name");
    assert_eq!(bot_command.description(), "description");
}

#[test]
fn bot_command_scope() {
    for (expected_struct, expected_value) in [
        (BotCommandScope::Default, serde_json::json!({"type": "default"})),
        (
            BotCommandScope::AllPrivateChats,
            serde_json::json!({"type": "all_private_chats"}),
        ),
        (
            BotCommandScope::AllGroupChats,
            serde_json::json!({"type": "all_group_chats"}),
        ),
        (
            BotCommandScope::AllChatAdministrators,
            serde_json::json!({"type": "all_chat_administrators"}),
        ),
        (
            BotCommandScope::chat(1),
            serde_json::json!({"type": "chat", "chat_id": 1}),
        ),
        (
            BotCommandScope::chat_administrators(1),
            serde_json::json!({"type": "chat_administrators", "chat_id": 1}),
        ),
        (
            BotCommandScope::chat_member(1, 1),
            serde_json::json!({"type": "chat_member", "chat_id": 1, "user_id": 1}),
        ),
    ] {
        assert_json_eq(expected_struct, expected_value);
    }
}

#[test]
fn close() {
    assert_payload_eq(Payload::empty("close"), Close);
}

#[test]
fn delete_bot_commands() {
    let method = DeleteBotCommands::default();
    assert_payload_eq(Payload::json("deleteMyCommands", serde_json::json!({})), method.clone());
    assert_payload_eq(
        Payload::json(
            "deleteMyCommands",
            serde_json::json!({
                "scope": {
                    "type": "default"
                },
                "language_code": "ru"
            }),
        ),
        method.scope(BotCommandScope::Default).language_code("ru"),
    );
}

#[test]
fn get_me() {
    assert_payload_eq(Payload::empty("getMe"), GetBot);
}

#[test]
fn get_bot_commands() {
    let method = GetBotCommands::default();
    assert_payload_eq(Payload::json("getMyCommands", serde_json::json!({})), method.clone());
    assert_payload_eq(
        Payload::json(
            "getMyCommands",
            serde_json::json!({
                "scope": {
                    "type": "default"
                },
                "language_code": "ru"
            }),
        ),
        method.scope(BotCommandScope::Default).language_code("ru"),
    );
}

#[test]
fn log_out() {
    assert_payload_eq(Payload::empty("logOut"), LogOut);
}

#[test]
fn set_bot_commands() {
    let method = SetBotCommands::new(vec![BotCommand::new("name", "description").unwrap()]);
    assert_payload_eq(
        Payload::json(
            "setMyCommands",
            serde_json::json!({
                "commands": [
                    {
                        "command": "name",
                        "description": "description"
                    }
                ]
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setMyCommands",
            serde_json::json!({
                "commands": [
                    {
                        "command": "name",
                        "description": "description"
                    }
                ],
                "scope": {
                    "type": "all_private_chats"
                },
                "language_code": "ru"
            }),
        ),
        method.scope(BotCommandScope::AllPrivateChats).language_code("ru"),
    );
}

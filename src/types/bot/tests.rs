use serde_json::Value as JsonValue;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{
        Bot,
        BotCommand,
        BotCommandScope,
        Close,
        DeleteBotCommands,
        GetBotCommands,
        GetMe,
        LogOut,
        SetBotCommands,
    },
};

#[test]
fn bot_deserialize() {
    let data: Bot = serde_json::from_value(serde_json::json!({
        "id": 1,
        "is_bot": true,
        "first_name": "Loo",
        "last_name": "Maclin",
        "username": "loomaclinbot",
        "can_join_groups": true,
        "can_read_all_group_messages": true,
        "supports_inline_queries": false
    }))
    .unwrap();
    assert_eq!(data.id, 1);
    assert_eq!(data.first_name, "Loo");
    assert_eq!(data.last_name.unwrap(), "Maclin");
    assert_eq!(data.username, "loomaclinbot");
    assert!(data.can_join_groups);
    assert!(data.can_read_all_group_messages);
    assert!(!data.supports_inline_queries);
}

#[test]
fn bot_command_new() {
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
}

#[test]
fn bot_command_scope() {
    for (scope, scope_type) in [
        (BotCommandScope::Default, "default"),
        (BotCommandScope::AllPrivateChats, "all_private_chats"),
        (BotCommandScope::AllGroupChats, "all_group_chats"),
        (BotCommandScope::AllChatAdministrators, "all_chat_administrators"),
        (BotCommandScope::chat(1), "chat"),
        (BotCommandScope::chat_administrators(1), "chat_administrators"),
        (BotCommandScope::chat_member(1, 1), "chat_member"),
    ] {
        let serialized_scope = serde_json::to_string(&scope).unwrap();
        let value: JsonValue = serde_json::from_str(&serialized_scope).unwrap();
        assert_eq!(value["type"], scope_type);
        let parsed_scope: BotCommandScope = serde_json::from_value(value).unwrap();
        assert_eq!(scope, parsed_scope);
    }
}

#[test]
fn close() {
    let request = Close.into_request();
    assert_eq!(request.get_method(), RequestMethod::Get);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/close");
    if let RequestBody::Empty = request.into_body() {
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn delete_bot_commands() {
    let request = DeleteBotCommands::default().into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/deleteMyCommands"
    );
    if let RequestBody::Json(data) = request.into_body() {
        assert_eq!(data.unwrap(), r#"{}"#)
    } else {
        panic!("Unexpected request body");
    }

    let request = DeleteBotCommands::default()
        .scope(BotCommandScope::Default)
        .language_code("ru")
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/deleteMyCommands"
    );
    if let RequestBody::Json(data) = request.into_body() {
        assert_eq!(data.unwrap(), r#"{"scope":{"type":"default"},"language_code":"ru"}"#)
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn get_bot_commands() {
    let request = GetBotCommands::default().into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/getMyCommands"
    );
    if let RequestBody::Json(data) = request.into_body() {
        assert_eq!(data.unwrap(), r#"{}"#)
    } else {
        panic!("Unexpected request body");
    }

    let request = GetBotCommands::default()
        .scope(BotCommandScope::Default)
        .language_code("ru")
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/getMyCommands"
    );
    if let RequestBody::Json(data) = request.into_body() {
        assert_eq!(data.unwrap(), r#"{"scope":{"type":"default"},"language_code":"ru"}"#)
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn get_me() {
    let request = GetMe.into_request();
    assert_eq!(request.get_method(), RequestMethod::Get);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/getMe");
    if let RequestBody::Empty = request.into_body() {
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn log_out() {
    let request = LogOut.into_request();
    assert_eq!(request.get_method(), RequestMethod::Get);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/logOut");
    if let RequestBody::Empty = request.into_body() {
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn set_bot_commands() {
    let request = SetBotCommands::new(vec![BotCommand::new("name", "description").unwrap()]).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/setMyCommands"
    );
    if let RequestBody::Json(data) = request.into_body() {
        assert_eq!(
            data.unwrap(),
            r#"{"commands":[{"command":"name","description":"description"}]}"#
        );
    } else {
        panic!("Unexpected request body");
    }

    let request = SetBotCommands::new(vec![BotCommand::new("name", "description").unwrap()])
        .scope(BotCommandScope::AllPrivateChats)
        .language_code("ru")
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/setMyCommands"
    );
    if let RequestBody::Json(data) = request.into_body() {
        assert_eq!(
            serde_json::from_str::<JsonValue>(&data.unwrap()).unwrap(),
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
            })
        );
    } else {
        panic!("Unexpected request body");
    }
}

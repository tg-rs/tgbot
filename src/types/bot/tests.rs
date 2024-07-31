use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        Bot,
        BotCommand,
        BotCommandScope,
        BotDescription,
        BotName,
        BotShortDescription,
        ChatAdministratorRights,
        Close,
        DeleteBotCommands,
        GetBot,
        GetBotCommands,
        GetBotDefaultAdministratorRights,
        GetBotDescription,
        GetBotName,
        GetBotShortDescription,
        LogOut,
        SetBotCommands,
        SetBotDefaultAdministratorRights,
        SetBotDescription,
        SetBotName,
        SetBotShortDescription,
    },
};

#[test]
fn bot() {
    assert_json_eq(
        Bot::new(1, "loo_maclin_bot", "Loo")
            .with_last_name("Maclin")
            .with_can_connect_to_business(true)
            .with_can_join_groups(true)
            .with_can_read_all_group_messages(true)
            .with_has_main_web_app(true)
            .with_supports_inline_queries(false),
        serde_json::json!({
            "can_connect_to_business": true,
            "can_join_groups": true,
            "can_read_all_group_messages": true,
            "first_name": "Loo",
            "id": 1,
            "has_main_web_app": true,
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
fn bot_description() {
    assert_json_eq(
        BotDescription::new("test-description"),
        serde_json::json!({
            "description": "test-description"
        }),
    );
}

#[test]
fn bot_name() {
    assert_json_eq(
        BotName::new("test_bot"),
        serde_json::json!({
            "name": "test_bot"
        }),
    );
}

#[test]
fn bot_short_description() {
    assert_json_eq(
        BotShortDescription::new("test-short-description"),
        serde_json::json!({
            "short_description": "test-short-description"
        }),
    );
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
        method.with_scope(BotCommandScope::Default).with_language_code("ru"),
    );
}

#[test]
fn get_bot() {
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
        method.with_scope(BotCommandScope::Default).with_language_code("ru"),
    );
}

#[test]
fn get_bot_default_administrator_rights() {
    let method = GetBotDefaultAdministratorRights::default();
    assert_payload_eq(
        Payload::json("getMyDefaultAdministratorRights", serde_json::json!({})),
        method,
    );
    assert_payload_eq(
        Payload::json(
            "getMyDefaultAdministratorRights",
            serde_json::json!({
                "for_channels": true
            }),
        ),
        method.with_for_channels(true),
    );
}

#[test]
fn get_bot_description() {
    let method = GetBotDescription::default();
    assert_payload_eq(Payload::json("getMyDescription", serde_json::json!({})), method.clone());
    assert_payload_eq(
        Payload::json(
            "getMyDescription",
            serde_json::json!({
                "language_code": "RU"
            }),
        ),
        method.with_language_code("RU"),
    );
}

#[test]
fn get_bot_name() {
    let method = GetBotName::default();
    assert_payload_eq(Payload::json("getMyName", serde_json::json!({})), method.clone());
    assert_payload_eq(
        Payload::json(
            "getMyName",
            serde_json::json!({
                "language_code": "RU"
            }),
        ),
        method.with_language_code("RU"),
    );
}

#[test]
fn get_bot_short_description() {
    let method = GetBotShortDescription::default();
    assert_payload_eq(
        Payload::json("getMyShortDescription", serde_json::json!({})),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "getMyShortDescription",
            serde_json::json!({
                "language_code": "RU"
            }),
        ),
        method.with_language_code("RU"),
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
        method
            .with_scope(BotCommandScope::AllPrivateChats)
            .with_language_code("ru"),
    );
}

#[test]
fn set_bot_default_administrator_rights() {
    let method = SetBotDefaultAdministratorRights::default();
    assert_payload_eq(
        Payload::json("setMyDefaultAdministratorRights", serde_json::json!({})),
        method,
    );
    assert_payload_eq(
        Payload::json(
            "setMyDefaultAdministratorRights",
            serde_json::json!({
                "rights": {
                    "is_anonymous": false,
                    "can_manage_chat": false,
                    "can_delete_messages": false,
                    "can_manage_video_chats": false,
                    "can_restrict_members": false,
                    "can_promote_members": false,
                    "can_change_info": false,
                    "can_invite_users": false,
                }
            }),
        ),
        method.with_rights(ChatAdministratorRights::default()),
    );
    assert_payload_eq(
        Payload::json(
            "setMyDefaultAdministratorRights",
            serde_json::json!({
                "for_channels": true
            }),
        ),
        method.with_for_channels(true),
    );
    assert_payload_eq(
        Payload::json(
            "setMyDefaultAdministratorRights",
            serde_json::json!({
                "rights": {
                    "is_anonymous": false,
                    "can_manage_chat": false,
                    "can_delete_messages": false,
                    "can_manage_video_chats": false,
                    "can_restrict_members": false,
                    "can_promote_members": false,
                    "can_change_info": false,
                    "can_invite_users": false,
                },
                "for_channels": true
            }),
        ),
        method
            .with_rights(ChatAdministratorRights::default())
            .with_for_channels(true),
    );
}

#[test]
fn set_bot_description() {
    let method = SetBotDescription::default();
    assert_payload_eq(Payload::json("setMyDescription", serde_json::json!({})), method.clone());
    assert_payload_eq(
        Payload::json(
            "setMyDescription",
            serde_json::json!({
                "description": "test-description",
                "language_code": "RU"
            }),
        ),
        method.with_description("test-description").with_language_code("RU"),
    );
}

#[test]
fn set_bot_name() {
    let method = SetBotName::default();
    assert_payload_eq(Payload::json("setMyName", serde_json::json!({})), method.clone());
    assert_payload_eq(
        Payload::json(
            "setMyName",
            serde_json::json!({
                "name": "test_bot_name",
                "language_code": "RU"
            }),
        ),
        method.with_name("test_bot_name").with_language_code("RU"),
    );
}

#[test]
fn set_bot_short_description() {
    let method = SetBotShortDescription::default();
    assert_payload_eq(
        Payload::json("setMyShortDescription", serde_json::json!({})),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "setMyShortDescription",
            serde_json::json!({
                "short_description": "test-short-description",
                "language_code": "RU"
            }),
        ),
        method
            .with_short_description("test-short-description")
            .with_language_code("RU"),
    );
}

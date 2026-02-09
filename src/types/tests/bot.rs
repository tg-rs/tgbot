use crate::types::*;

#[test]
fn bot() {
    insta::assert_json_snapshot!(
        Bot::new(1, "loo_maclin_bot", "Loo")
            .with_last_name("Maclin")
            .with_can_connect_to_business(true)
            .with_can_join_groups(true)
            .with_can_read_all_group_messages(true)
            .with_has_main_web_app(true)
            .with_has_topics_enabled(false)
            .with_supports_inline_queries(true),
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
    for value in [
        BotCommandScope::Default,
        BotCommandScope::AllPrivateChats,
        BotCommandScope::AllGroupChats,
        BotCommandScope::AllChatAdministrators,
        BotCommandScope::chat(1),
        BotCommandScope::chat_administrators(1),
        BotCommandScope::chat_member(1, 1),
    ] {
        insta::assert_json_snapshot!(value);
    }
}

#[test]
fn bot_description() {
    insta::assert_json_snapshot!(BotDescription::new("test-description"));
}

#[test]
fn bot_name() {
    insta::assert_json_snapshot!(BotName::new("test_bot"));
}

#[test]
fn bot_short_description() {
    insta::assert_json_snapshot!(BotShortDescription::new("test-short-description"));
}

#[test]
fn close() {
    assert_payload_eq!(GET "close" => Close);
}

#[test]
fn delete_bot_commands() {
    let method = DeleteBotCommands::default();
    assert_payload_eq!(POST JSON "deleteMyCommands" => method.clone());
    let method = method.with_scope(BotCommandScope::Default).with_language_code("ru");
    assert_payload_eq!(POST JSON "deleteMyCommands" => method);
}

#[test]
fn get_bot() {
    assert_payload_eq!(GET "getMe" => GetBot);
}

#[test]
fn get_bot_commands() {
    let method = GetBotCommands::default();
    assert_payload_eq!(POST JSON "getMyCommands" => method.clone());
    let method = method.with_scope(BotCommandScope::Default).with_language_code("ru");
    assert_payload_eq!(POST JSON "getMyCommands" => method);
}

#[test]
fn get_bot_default_administrator_rights() {
    let method = GetBotDefaultAdministratorRights::default();
    assert_payload_eq!(POST JSON "getMyDefaultAdministratorRights" => method);
    assert_payload_eq!(POST JSON "getMyDefaultAdministratorRights" => method.with_for_channels(true));
}

#[test]
fn get_bot_description() {
    let method = GetBotDescription::default();
    assert_payload_eq!(POST JSON "getMyDescription" => method.clone());
    assert_payload_eq!(POST JSON "getMyDescription" => method.with_language_code("RU"));
}

#[test]
fn get_bot_name() {
    let method = GetBotName::default();
    assert_payload_eq!(POST JSON "getMyName" => method.clone());
    let method = method.with_language_code("RU");
    assert_payload_eq!(POST JSON "getMyName" => method);
}

#[test]
fn get_bot_short_description() {
    let method = GetBotShortDescription::default();
    assert_payload_eq!(POST JSON "getMyShortDescription" => method.clone());
    assert_payload_eq!(POST JSON "getMyShortDescription" => method.with_language_code("RU"));
}

#[test]
fn get_bot_star_balance() {
    let method = GetBotStarBalance;
    assert_payload_eq!(GET "getMyStarBalance" => method);
}

#[test]
fn log_out() {
    assert_payload_eq!(GET "logOut" => LogOut);
}

#[test]
fn set_bot_commands() {
    let method = SetBotCommands::new(vec![BotCommand::new("name", "description").unwrap()]);
    assert_payload_eq!(POST JSON "setMyCommands" => method.clone());
    let method = method
        .with_scope(BotCommandScope::AllPrivateChats)
        .with_language_code("ru");
    assert_payload_eq!(POST JSON "setMyCommands" => method);
}

#[test]
fn set_bot_default_administrator_rights() {
    let method = SetBotDefaultAdministratorRights::default();
    assert_payload_eq!(POST JSON "setMyDefaultAdministratorRights" => method);
    let method = method.with_rights(ChatAdministratorRights::default());
    assert_payload_eq!(POST JSON "setMyDefaultAdministratorRights" => method);
    assert_payload_eq!(POST JSON "setMyDefaultAdministratorRights" => method.with_for_channels(true));
    let method = method
        .with_rights(ChatAdministratorRights::default())
        .with_for_channels(true);
    assert_payload_eq!(POST JSON "setMyDefaultAdministratorRights" => method);
}

#[test]
fn set_bot_description() {
    let method = SetBotDescription::default();
    assert_payload_eq!(POST JSON "setMyDescription" => method.clone());
    let method = method.with_description("test-description").with_language_code("RU");
    assert_payload_eq!(POST JSON "setMyDescription" => method);
}

#[test]
fn set_bot_name() {
    let method = SetBotName::default();
    assert_payload_eq!(POST JSON "setMyName" => method.clone());
    let method = method.with_name("test_bot_name").with_language_code("RU");
    assert_payload_eq!(POST JSON "setMyName" => method);
}

#[test]
fn set_bot_profile_photo() {
    let photo = InputProfilePhotoStatic::new(InputFile::url("https://example.com/photo.png"));
    let method = SetBotProfilePhoto::new(photo).unwrap();
    assert_payload_eq!(POST FORM "setMyProfilePhoto" => method);
}

#[test]
fn set_bot_short_description() {
    let method = SetBotShortDescription::default();
    assert_payload_eq!(POST JSON "setMyShortDescription" => method.clone());
    let method = method
        .with_short_description("test-short-description")
        .with_language_code("RU");
    assert_payload_eq!(POST JSON "setMyShortDescription" => method);
}

#[test]
fn remove_bot_profile_photo() {
    let method = RemoveBotProfilePhoto;
    assert_payload_eq!(GET "removeMyProfilePhoto" => method);
}

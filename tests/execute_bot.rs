#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;

    cx.execute_batch([("close", Close, |x| assert!(x))]).await;

    cx.execute_batch([
        ("delete-bot-commands-base", DeleteBotCommands::default(), |x| assert!(x)),
        (
            "delete-bot-commands-all",
            DeleteBotCommands::default()
                .with_scope(BotCommandScope::Default)
                .with_language_code("ru"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        ("get-bot-base", GetBot, |x| {
            assert_eq!(x.id, 1);
            assert_eq!(x.first_name, "Test");
            assert!(x.last_name.is_none());
            assert_eq!(x.username, "test");
            assert!(x.can_join_groups);
            assert!(x.can_read_all_group_messages);
            assert!(x.supports_guest_queries);
            assert!(x.supports_inline_queries);
            assert!(x.can_connect_to_business);
            assert!(x.has_main_web_app);
            assert!(x.has_topics_enabled);
            assert!(x.allows_users_to_create_topics);
            assert!(x.can_manage_bots);
            assert!(x.supports_join_request_queries);
        }),
        ("get-bot-all", GetBot, |x| {
            assert_eq!(x.id, 1);
            assert_eq!(x.first_name, "Test");
            assert_eq!(x.last_name.unwrap(), "Test");
            assert_eq!(x.username, "test");
            assert!(x.can_join_groups);
            assert!(x.can_read_all_group_messages);
            assert!(x.supports_guest_queries);
            assert!(x.supports_inline_queries);
            assert!(x.can_connect_to_business);
            assert!(x.has_main_web_app);
            assert!(x.has_topics_enabled);
            assert!(x.allows_users_to_create_topics);
            assert!(x.can_manage_bots);
            assert!(x.supports_join_request_queries);
        }),
    ])
    .await;

    cx.execute_batch([
        ("get-bot-commands-base", GetBotCommands::default(), |x| {
            assert_eq!(x.len(), 1);
            assert_eq!(x[0].name(), "test");
            assert_eq!(x[0].description(), "test");
            assert!(x[0].is_ephemeral().is_none());
        }),
        (
            "get-bot-commands-default",
            GetBotCommands::default()
                .with_scope(BotCommandScope::Default)
                .with_language_code("ru"),
            |x| {
                assert_eq!(x.len(), 1);
                assert_eq!(x[0].name(), "test");
                assert_eq!(x[0].description(), "test");
                assert!(x[0].is_ephemeral().unwrap());
            },
        ),
        (
            "get-bot-commands-all-private-chats",
            GetBotCommands::default().with_scope(BotCommandScope::AllPrivateChats),
            |x| {
                assert_eq!(x.len(), 1);
            },
        ),
        (
            "get-bot-commands-all-group-chats",
            GetBotCommands::default().with_scope(BotCommandScope::AllGroupChats),
            |x| {
                assert_eq!(x.len(), 1);
            },
        ),
        (
            "get-bot-commands-all-chat-administrators",
            GetBotCommands::default().with_scope(BotCommandScope::AllChatAdministrators),
            |x| {
                assert_eq!(x.len(), 1);
            },
        ),
        (
            "get-bot-commands-chat",
            GetBotCommands::default().with_scope(BotCommandScope::chat(1)),
            |x| {
                assert_eq!(x.len(), 1);
            },
        ),
        (
            "get-bot-commands-chat-administrators",
            GetBotCommands::default().with_scope(BotCommandScope::chat_administrators(1)),
            |x| {
                assert_eq!(x.len(), 1);
            },
        ),
        (
            "get-bot-commands-chat-member",
            GetBotCommands::default().with_scope(BotCommandScope::chat_member(1, 2)),
            |x| {
                assert_eq!(x.len(), 1);
            },
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "get-bot-default-administrator-rights-base",
            GetBotDefaultAdministratorRights::default(),
            |x| {
                assert!(x.is_anonymous);
                assert!(x.can_manage_chat);
                assert!(x.can_delete_messages);
                assert!(x.can_manage_video_chats);
                assert!(x.can_restrict_members);
                assert!(x.can_promote_members);
                assert!(x.can_change_info);
                assert!(x.can_invite_users);
                assert!(x.can_post_stories.unwrap());
                assert!(x.can_edit_stories.unwrap());
                assert!(x.can_delete_stories.unwrap());
                assert!(x.can_post_messages.is_none());
                assert!(x.can_edit_messages.is_none());
                assert!(x.can_pin_messages.is_none());
                assert!(x.can_manage_topics.is_none());
                assert!(x.can_manage_direct_messages.is_none());
                assert!(x.can_manage_tags.is_none());
            },
        ),
        (
            "get-bot-default-administrator-rights-all",
            GetBotDefaultAdministratorRights::default().with_for_channels(true),
            |x| {
                assert!(x.is_anonymous);
                assert!(x.can_manage_chat);
                assert!(x.can_delete_messages);
                assert!(x.can_manage_video_chats);
                assert!(x.can_restrict_members);
                assert!(x.can_promote_members);
                assert!(x.can_change_info);
                assert!(x.can_invite_users);
                assert!(x.can_post_stories.unwrap());
                assert!(x.can_edit_stories.unwrap());
                assert!(x.can_delete_stories.unwrap());
                assert!(x.can_post_messages.unwrap());
                assert!(x.can_edit_messages.unwrap());
                assert!(x.can_pin_messages.unwrap());
                assert!(x.can_manage_topics.unwrap());
                assert!(x.can_manage_direct_messages.unwrap());
                assert!(x.can_manage_tags.unwrap());
            },
        ),
    ])
    .await;

    cx.execute_batch([
        ("get-bot-description-base", GetBotDescription::default(), |x| {
            assert_eq!(x.description, "test")
        }),
        (
            "get-bot-description-all",
            GetBotDescription::default().with_language_code("RU"),
            |x| assert_eq!(x.description, "test"),
        ),
    ])
    .await;

    cx.execute_batch([
        ("get-bot-name-base", GetBotName::default(), |x| {
            assert_eq!(x.name, "test")
        }),
        (
            "get-bot-name-all",
            GetBotName::default().with_language_code("RU"),
            |x| assert_eq!(x.name, "test"),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "get-bot-short-description-base",
            GetBotShortDescription::default(),
            |x| assert_eq!(x.short_description, "test"),
        ),
        (
            "get-bot-short-description-all",
            GetBotShortDescription::default().with_language_code("RU"),
            |x| assert_eq!(x.short_description, "test"),
        ),
    ])
    .await;

    cx.execute_batch([("get-bot-star-balance", GetBotStarBalance, |x| {
        assert_eq!(x.amount, 10);
        assert!(x.nanostar_amount.is_none());
    })])
    .await;

    cx.execute_batch([(
        "get-managed-bot-access-settings",
        GetManagedBotAccessSettings::from(1),
        |x| {
            assert!(x.is_access_restricted);
            assert!(x.added_users.is_none());
        },
    )])
    .await;

    cx.execute_batch([("get-managed-bot-token", GetManagedBotToken::from(1), |x| {
        assert_eq!(x, "test")
    })])
    .await;

    cx.execute_batch([("log-out", LogOut, |x| assert!(x))]).await;

    cx.execute_batch([("replace-managed-bot-token", ReplaceManagedBotToken::from(1), |x| {
        assert_eq!(x, "test")
    })])
    .await;

    cx.execute_batch([
        (
            "set-bot-commands-base",
            SetBotCommands::new(vec![BotCommand::new("name", "description").unwrap()]),
            |x| assert!(x),
        ),
        (
            "set-bot-commands-all",
            SetBotCommands::new(vec![BotCommand::new("name", "description").unwrap()])
                .with_scope(BotCommandScope::AllPrivateChats)
                .with_language_code("ru"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "set-bot-default-administrator-rights-base",
            SetBotDefaultAdministratorRights::default(),
            |x| assert!(x),
        ),
        (
            "set-bot-default-administrator-rights-all",
            SetBotDefaultAdministratorRights::default()
                .with_rights(ChatAdministratorRights::default())
                .with_for_channels(true),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        ("set-bot-description-base", SetBotDescription::default(), |x| assert!(x)),
        (
            "set-bot-description-all",
            SetBotDescription::default()
                .with_description("test-description")
                .with_language_code("RU"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        ("set-bot-name-base", SetBotName::default(), |x| assert!(x)),
        (
            "set-bot-name-all",
            SetBotName::default()
                .with_name("test_bot_name")
                .with_language_code("RU"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([(
        "set-bot-profile-photo",
        SetBotProfilePhoto::new(InputProfilePhotoStatic::new(InputFile::url(
            "https://example.com/photo.png",
        ))),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([
        (
            "set-bot-short-description-base",
            SetBotShortDescription::default(),
            |x| assert!(x),
        ),
        (
            "set-bot-short-description-all",
            SetBotShortDescription::default()
                .with_short_description("test-short-description")
                .with_language_code("RU"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "set-managed-bot-access-settings-base",
            SetManagedBotAccessSettings::new(1, false),
            |x| assert!(x),
        ),
        (
            "set-managed-bot-access-settings-all",
            SetManagedBotAccessSettings::new(1, false).with_added_user_ids([2, 3, 4]),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([("remove-bot-profile-photo", RemoveBotProfilePhoto, |x| assert!(x))])
        .await;
}

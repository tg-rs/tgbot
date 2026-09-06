#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-chat-action-choose-sticker",
            SendChatAction::new(1, ChatAction::ChooseSticker),
            |x| assert!(x),
        ),
        (
            "send-chat-action-find-location",
            SendChatAction::new(1, ChatAction::FindLocation),
            |x| assert!(x),
        ),
        (
            "send-chat-action-record-video",
            SendChatAction::new(1, ChatAction::RecordVideo),
            |x| assert!(x),
        ),
        (
            "send-chat-action-record-voice",
            SendChatAction::new(1, ChatAction::RecordVoice),
            |x| assert!(x),
        ),
        (
            "send-chat-action-record-video-note",
            SendChatAction::new(1, ChatAction::RecordVideoNote),
            |x| assert!(x),
        ),
        (
            "send-chat-action-typing",
            SendChatAction::new("@test_chat", ChatAction::Typing)
                .with_business_connection_id("id")
                .with_message_thread_id(1),
            |x| assert!(x),
        ),
        (
            "send-chat-action-upload-document",
            SendChatAction::new(1, ChatAction::UploadDocument),
            |x| assert!(x),
        ),
        (
            "send-chat-action-upload-photo",
            SendChatAction::new(1, ChatAction::UploadPhoto),
            |x| assert!(x),
        ),
        (
            "send-chat-action-upload-video",
            SendChatAction::new(1, ChatAction::UploadVideo),
            |x| assert!(x),
        ),
        (
            "send-chat-action-upload-video-note",
            SendChatAction::new(1, ChatAction::UploadVideoNote),
            |x| assert!(x),
        ),
        (
            "send-chat-action-upload-voice",
            SendChatAction::new(1, ChatAction::UploadVoice),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute(
        "send-chat-join-request-web-app",
        SendChatJoinRequestWebApp::new("query-id", "https://example.com/"),
        |x| assert!(x),
    )
    .await;
    cx.execute_batch([
        ("get-chat-private", GetChat::new(1), |x| {
            assert_eq!(x.id, 1);
            assert_eq!(x.chat_type, ChatFullInfoType::Private);

            assert_eq!(x.accent_color, AccentColor::Red);
            assert_eq!(x.max_reaction_count, 3);

            assert!(x.available_reactions.is_none());
            assert!(x.background_custom_emoji_id.is_none());
            assert!(x.has_hidden_members.is_none());
            assert!(x.has_protected_content.is_none());
            assert!(x.has_visible_history.is_none());
            assert!(x.message_auto_delete_time.is_none());
            assert!(x.photo.is_none());
            assert!(x.pinned_message.is_none());
            assert!(x.profile_accent_color.is_none());
            assert!(x.profile_background_custom_emoji_id.is_none());

            assert_eq!(x.active_usernames.unwrap(), ["test1"]);
            assert_eq!(x.username.unwrap(), "test");

            assert!(x.description.is_none());
            assert!(x.invite_link.is_none());

            assert!(x.title.is_none());

            assert!(x.can_send_paid_media.is_none());

            assert!(x.linked_chat_id.is_none());

            assert_eq!(x.bio.unwrap(), "test");
            let birthdate = x.birthdate.unwrap();
            assert_eq!(birthdate.day, 20);
            assert_eq!(birthdate.month, 4);
            assert_eq!(birthdate.year.unwrap(), 1990);
            let intro = x.business_intro.unwrap();
            assert_eq!(intro.title.unwrap(), "test");
            assert_eq!(intro.message.unwrap(), "test");
            assert_eq!(intro.sticker.unwrap().file_id, "fid");
            let location = x.business_location.unwrap();
            assert_eq!(location.address, "test");
            assert_eq!(location.location.unwrap().latitude, 1.0);
            let hours = x.business_opening_hours.unwrap();
            assert_eq!(hours.time_zone_name, "UTC");
            assert_eq!(hours.opening_hours[0], BusinessOpeningHoursInterval::from((0, 1)));
            assert!(x.emoji_status_custom_emoji_id.is_none());
            assert!(x.emoji_status_expiration_date.is_none());
            assert_eq!(x.first_name.unwrap(), "John");
            assert!(x.first_profile_audio.is_some());
            assert!(x.has_private_forwards.is_none());
            assert!(x.has_restricted_voice_and_video_messages.unwrap());
            assert_eq!(x.last_name.unwrap(), "Doe");
            assert!(x.personal_chat.is_some());
            let rating = x.rating.unwrap();
            assert_eq!(rating.level, 1);
            assert_eq!(rating.rating, 2);
            assert_eq!(rating.current_level_rating, 3);
            assert_eq!(rating.next_level_rating.unwrap(), 4);

            assert!(x.can_set_sticker_set.is_none());
            assert!(x.custom_emoji_sticker_set_name.is_none());
            assert!(x.has_aggressive_anti_spam_enabled.is_none());
            assert!(x.is_direct_messages.is_none());
            assert!(x.is_forum.is_none());
            assert!(x.join_to_send_messages.is_none());
            assert!(x.join_by_request.is_none());
            assert!(x.location.is_none());
            assert!(x.parent_chat.is_none());
            assert!(x.slow_mode_delay.is_none());
            assert!(x.sticker_set_name.is_none());
            assert!(x.unrestrict_boost_count.is_none());

            assert!(x.permissions.is_none());

            assert!(x.accepted_gift_types.is_none());
            assert!(x.paid_message_star_count.is_none());
            assert!(x.unique_gift_colors.is_none());

            assert!(x.guard_bot.is_none());
            assert!(x.community.is_none());
        }),
        ("get-chat-channel", GetChat::new(1), |x| {
            assert_eq!(x.id, 1);
            assert_eq!(x.chat_type, ChatFullInfoType::Channel);

            assert_eq!(x.accent_color, AccentColor::Red);
            assert_eq!(x.max_reaction_count, 3);

            assert!(x.available_reactions.is_none());
            assert!(x.background_custom_emoji_id.is_none());
            assert!(x.has_hidden_members.is_none());
            assert!(x.has_protected_content.is_none());
            assert!(x.has_visible_history.is_none());
            assert!(x.message_auto_delete_time.is_none());
            assert!(x.photo.is_none());
            assert!(x.pinned_message.is_some());
            assert!(x.profile_accent_color.is_none());
            assert!(x.profile_background_custom_emoji_id.is_none());

            assert!(x.active_usernames.is_none());
            assert_eq!(x.username.unwrap(), "test");

            assert!(x.description.is_none());
            assert_eq!(x.invite_link.unwrap(), "test");

            assert_eq!(x.title.unwrap(), "Title");

            assert!(x.can_send_paid_media.is_none());

            assert_eq!(x.linked_chat_id.unwrap(), 2);

            assert!(x.bio.is_none());
            assert!(x.birthdate.is_none());
            assert!(x.business_intro.is_none());
            assert!(x.business_location.is_none());
            assert!(x.business_opening_hours.is_none());
            assert!(x.emoji_status_custom_emoji_id.is_none());
            assert!(x.emoji_status_expiration_date.is_none());
            assert!(x.first_name.is_none());
            assert!(x.first_profile_audio.is_none());
            assert!(x.has_private_forwards.is_none());
            assert!(x.has_restricted_voice_and_video_messages.is_none());
            assert!(x.last_name.is_none());
            assert!(x.personal_chat.is_none());
            assert!(x.rating.is_none());

            assert!(x.can_set_sticker_set.is_none());
            assert!(x.custom_emoji_sticker_set_name.is_none());
            assert!(x.has_aggressive_anti_spam_enabled.unwrap());
            assert!(x.is_direct_messages.is_none());
            assert!(x.is_forum.is_none());
            assert!(x.join_to_send_messages.is_none());
            assert!(x.join_by_request.is_none());
            assert!(x.location.is_none());
            assert!(x.parent_chat.is_none());
            assert!(x.slow_mode_delay.is_none());
            assert!(x.sticker_set_name.is_none());
            assert!(x.unrestrict_boost_count.is_none());

            assert!(x.permissions.is_none());

            assert!(x.accepted_gift_types.is_none());
            assert!(x.paid_message_star_count.is_none());
            assert!(x.unique_gift_colors.is_none());

            assert!(x.guard_bot.is_none());
            assert!(x.community.is_none());
        }),
        ("get-chat-group", GetChat::new(1), |x| {
            assert_eq!(x.id, 1);
            assert_eq!(x.chat_type, ChatFullInfoType::Group);

            assert_eq!(x.accent_color, AccentColor::Red);
            assert_eq!(x.max_reaction_count, 3);

            let reactions = x.available_reactions.unwrap();
            assert_eq!(reactions.len(), 3);
            assert!(matches!(&reactions[0], ReactionType::Emoji(_)));
            assert!(matches!(&reactions[1], ReactionType::CustomEmoji(_)));
            assert!(matches!(&reactions[2], ReactionType::Paid));
            assert_eq!(x.background_custom_emoji_id.unwrap(), "test");
            assert!(!x.has_hidden_members.unwrap());
            assert!(x.has_protected_content.unwrap());
            assert!(!x.has_visible_history.unwrap());
            assert_eq!(x.message_auto_delete_time.unwrap(), 86400);
            let photo = x.photo.unwrap();
            assert_eq!(photo.small_file_id, "sfid");
            assert_eq!(photo.small_file_unique_id, "sfuid");
            assert_eq!(photo.big_file_id, "bfid");
            assert_eq!(photo.big_file_unique_id, "bfuid");
            assert!(x.pinned_message.is_none());
            assert_eq!(
                x.profile_accent_color.unwrap(),
                ProfileAccentColor::try_from(7).unwrap()
            );
            assert_eq!(x.profile_background_custom_emoji_id.unwrap(), "test");

            assert!(x.active_usernames.is_none());
            assert!(x.username.is_none());

            assert!(x.description.is_none());
            assert!(x.invite_link.is_none());

            assert_eq!(x.title.unwrap(), "Title");

            assert!(x.can_send_paid_media.is_none());

            assert!(x.linked_chat_id.is_none());

            assert!(x.bio.is_none());
            assert!(x.birthdate.is_none());
            assert!(x.business_intro.is_none());
            assert!(x.business_location.is_none());
            assert!(x.business_opening_hours.is_none());
            assert!(x.emoji_status_custom_emoji_id.is_none());
            assert!(x.emoji_status_expiration_date.is_none());
            assert!(x.first_name.is_none());
            assert!(x.first_profile_audio.is_none());
            assert!(x.has_private_forwards.is_none());
            assert!(x.has_restricted_voice_and_video_messages.is_none());
            assert!(x.last_name.is_none());
            assert!(x.personal_chat.is_none());
            assert!(x.rating.is_none());

            assert!(x.can_set_sticker_set.unwrap());
            assert_eq!(x.custom_emoji_sticker_set_name.unwrap(), "test");
            assert!(x.has_aggressive_anti_spam_enabled.is_none());
            assert!(!x.is_direct_messages.unwrap());
            assert!(x.is_forum.is_none());
            assert!(x.join_to_send_messages.is_none());
            assert!(x.join_by_request.is_none());
            assert!(x.location.is_none());
            assert!(x.parent_chat.is_none());
            assert!(x.slow_mode_delay.is_none());
            assert_eq!(x.sticker_set_name.unwrap(), "test");
            assert!(x.unrestrict_boost_count.is_none());

            assert!(x.permissions.is_some());

            assert!(x.accepted_gift_types.is_some());
            assert_eq!(x.paid_message_star_count.unwrap(), 1);
            let gift_colors = x.unique_gift_colors.unwrap();
            assert_eq!(gift_colors.model_custom_emoji_id, "test");
            assert_eq!(gift_colors.symbol_custom_emoji_id, "test");
            assert_eq!(gift_colors.light_theme_main_color, 7);
            assert_eq!(gift_colors.light_theme_other_colors, [8, 9]);
            assert_eq!(gift_colors.dark_theme_main_color, 7);
            assert_eq!(gift_colors.dark_theme_other_colors, [8, 9]);

            let guard_bot = x.guard_bot.unwrap();
            assert_eq!(guard_bot.id, 1);
            assert_eq!(guard_bot.first_name, "Bot");
            assert!(guard_bot.is_bot);
            let community = x.community.unwrap();
            assert_eq!(community.id, 1);
            assert_eq!(community.name, "Test");
        }),
        ("get-chat-supergroup", GetChat::new(1), |x| {
            assert_eq!(x.id, 1);
            assert_eq!(x.chat_type, ChatFullInfoType::Supergroup);

            assert_eq!(x.accent_color, AccentColor::Red);
            assert_eq!(x.max_reaction_count, 3);

            let reactions = x.available_reactions.unwrap();
            assert_eq!(reactions.len(), 3);
            assert!(matches!(&reactions[0], ReactionType::Emoji(_)));
            assert!(matches!(&reactions[1], ReactionType::CustomEmoji(_)));
            assert!(matches!(&reactions[2], ReactionType::Paid));
            assert_eq!(x.background_custom_emoji_id.unwrap(), "test");
            assert!(!x.has_hidden_members.unwrap());
            assert!(x.has_protected_content.unwrap());
            assert!(!x.has_visible_history.unwrap());
            assert_eq!(x.message_auto_delete_time.unwrap(), 86400);
            let photo = x.photo.unwrap();
            assert_eq!(photo.small_file_id, "sfid");
            assert_eq!(photo.small_file_unique_id, "sfuid");
            assert_eq!(photo.big_file_id, "bfid");
            assert_eq!(photo.big_file_unique_id, "bfuid");
            assert!(x.pinned_message.is_none());
            assert_eq!(
                x.profile_accent_color.unwrap(),
                ProfileAccentColor::try_from(7).unwrap()
            );
            assert_eq!(x.profile_background_custom_emoji_id.unwrap(), "test");

            assert!(x.active_usernames.is_none());
            assert!(x.username.is_none());

            assert_eq!(x.description.unwrap(), "Test");
            assert!(x.invite_link.is_none());

            assert_eq!(x.title.unwrap(), "Title");

            assert!(x.can_send_paid_media.is_none());

            assert!(x.linked_chat_id.is_none());

            assert!(x.bio.is_none());
            assert!(x.birthdate.is_none());
            assert!(x.business_intro.is_none());
            assert!(x.business_location.is_none());
            assert!(x.business_opening_hours.is_none());
            assert!(x.emoji_status_custom_emoji_id.is_none());
            assert!(x.emoji_status_expiration_date.is_none());
            assert!(x.first_name.is_none());
            assert!(x.first_profile_audio.is_none());
            assert!(x.has_private_forwards.is_none());
            assert!(x.has_restricted_voice_and_video_messages.is_none());
            assert!(x.last_name.is_none());
            assert!(x.personal_chat.is_none());
            assert!(x.rating.is_none());

            assert!(x.can_set_sticker_set.unwrap());
            assert_eq!(x.custom_emoji_sticker_set_name.unwrap(), "test");
            assert!(x.has_aggressive_anti_spam_enabled.unwrap());
            assert!(x.is_direct_messages.unwrap());
            assert!(x.is_forum.unwrap());
            assert!(x.join_to_send_messages.unwrap());
            assert!(x.join_by_request.unwrap());
            let location = x.location.unwrap();
            assert_eq!(location.address, "test");
            assert_eq!(location.location.latitude, 1.0);
            assert_eq!(location.location.longitude, 2.0);
            assert!(x.parent_chat.is_some());
            assert_eq!(x.slow_mode_delay.unwrap(), 5);
            assert_eq!(x.sticker_set_name.unwrap(), "test");
            assert_eq!(x.unrestrict_boost_count.unwrap(), 10);

            assert!(x.permissions.is_some());

            assert!(x.accepted_gift_types.is_some());
            assert_eq!(x.paid_message_star_count.unwrap(), 1);
            let gift_colors = x.unique_gift_colors.unwrap();
            assert_eq!(gift_colors.model_custom_emoji_id, "test");
            assert_eq!(gift_colors.symbol_custom_emoji_id, "test");
            assert_eq!(gift_colors.light_theme_main_color, 7);
            assert_eq!(gift_colors.light_theme_other_colors, [8, 9]);
            assert_eq!(gift_colors.dark_theme_main_color, 7);
            assert_eq!(gift_colors.dark_theme_other_colors, [8, 9]);

            let guard_bot = x.guard_bot.unwrap();
            assert_eq!(guard_bot.id, 1);
            assert_eq!(guard_bot.first_name, "Bot");
            assert!(guard_bot.is_bot);
            let community = x.community.unwrap();
            assert_eq!(community.id, 1);
            assert_eq!(community.name, "Test");
        }),
    ])
    .await;
    cx.execute_batch([("leave-chat", LeaveChat::new(1), |x| assert!(x))])
        .await;
    cx.execute_batch([
        ("set-chat-description-base", SetChatDescription::new(1), |x| assert!(x)),
        (
            "set-chat-description-all",
            SetChatDescription::new(1).with_description("Description"),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([("set-chat-title", SetChatTitle::new(1, "Chat"), |x| assert!(x))])
        .await;
    cx.execute_batch([("get-user-chat-boosts", GetUserChatBoosts::new(1, 2), |x| {
        assert_eq!(x.boosts.len(), 4);
        let premium = &x.boosts[0];
        assert_eq!(premium.boost_id, "boost-1");
        assert_eq!(premium.add_date, 0);
        assert_eq!(premium.expiration_date, 1);
        let ChatBoostSource::Premium(user) = &premium.source else {
            panic!("Unexpected source")
        };
        assert_eq!(user.id, 1);

        let gift_code = &x.boosts[1];
        assert_eq!(gift_code.boost_id, "boost-2");
        assert_eq!(gift_code.add_date, 0);
        assert_eq!(gift_code.expiration_date, 1);
        let ChatBoostSource::GiftCode(user) = &gift_code.source else {
            panic!("Unexpected source")
        };
        assert_eq!(user.id, 1);

        let giveaway_base = &x.boosts[2];
        assert_eq!(giveaway_base.boost_id, "boost-3");
        assert_eq!(giveaway_base.add_date, 0);
        assert_eq!(giveaway_base.expiration_date, 1);
        let ChatBoostSource::Giveaway(source) = &giveaway_base.source else {
            panic!("Unexpected source")
        };
        assert_eq!(source.giveaway_message_id, 1);
        assert!(source.user.is_none());

        let giveaway_all = &x.boosts[3];
        assert_eq!(giveaway_all.boost_id, "boost-4");
        assert_eq!(giveaway_all.add_date, 0);
        assert_eq!(giveaway_all.expiration_date, 1);
        let ChatBoostSource::Giveaway(source) = &giveaway_all.source else {
            panic!("Unexpected source")
        };
        assert_eq!(source.giveaway_message_id, 2);
        assert!(source.user.is_some());
        assert_eq!(source.prize_star_count.unwrap(), 10);
        assert!(source.is_unclaimed.unwrap());
    })])
    .await;
    cx.execute_batch([
        ("create-chat-invite-link-base", CreateChatInviteLink::new(1), |x| {
            assert_eq!(x.invite_link, "test");
            assert_eq!(x.creator.id, 1);
            assert!(x.creates_join_request);
            assert!(x.is_primary);
            assert!(!x.is_revoked);
            assert!(x.name.is_none());
            assert!(x.expire_date.is_none());
            assert!(x.member_limit.is_none());
            assert!(x.pending_join_request_count.is_none());
            assert!(x.subscription_period.is_none());
            assert!(x.subscription_price.is_none());
        }),
        (
            "create-chat-invite-link-all",
            CreateChatInviteLink::new(1)
                .with_name("Link")
                .with_expire_date(0)
                .with_member_limit(1)
                .with_creates_join_request(false),
            |x| {
                assert_eq!(x.invite_link, "test");
                assert_eq!(x.creator.id, 1);
                assert!(x.creates_join_request);
                assert!(x.is_primary);
                assert!(!x.is_revoked);
                assert_eq!(x.name.unwrap(), "test");
                assert_eq!(x.expire_date.unwrap(), 0);
                assert_eq!(x.member_limit.unwrap(), 1);
                assert_eq!(x.pending_join_request_count.unwrap(), 0);
                assert_eq!(x.subscription_period.unwrap(), 600);
                assert_eq!(x.subscription_price.unwrap(), 100);
            },
        ),
    ])
    .await;
    cx.execute_batch([
        (
            "create-chat-subscription-invite-link-base",
            CreateChatSubscriptionInviteLink::new(1, 2592000, 1),
            |x| {
                assert_eq!(x.invite_link, "test");
                assert_eq!(x.creator.id, 1);
                assert!(x.creates_join_request);
                assert!(x.is_primary);
                assert!(!x.is_revoked);
                assert_eq!(x.name.unwrap(), "test");
                assert_eq!(x.expire_date.unwrap(), 0);
                assert_eq!(x.member_limit.unwrap(), 1);
                assert_eq!(x.pending_join_request_count.unwrap(), 0);
                assert_eq!(x.subscription_period.unwrap(), 600);
                assert_eq!(x.subscription_price.unwrap(), 100);
            },
        ),
        (
            "create-chat-subscription-invite-link-all",
            CreateChatSubscriptionInviteLink::new(1, 2592000, 1).with_name("test"),
            |x| {
                assert_eq!(x.invite_link, "test");
                assert_eq!(x.creator.id, 1);
                assert!(x.creates_join_request);
                assert!(x.is_primary);
                assert!(!x.is_revoked);
                assert_eq!(x.name.unwrap(), "test");
                assert_eq!(x.expire_date.unwrap(), 0);
                assert_eq!(x.member_limit.unwrap(), 1);
                assert_eq!(x.pending_join_request_count.unwrap(), 0);
                assert_eq!(x.subscription_period.unwrap(), 600);
                assert_eq!(x.subscription_price.unwrap(), 100);
            },
        ),
    ])
    .await;
    cx.execute_batch([
        (
            "edit-chat-invite-link-base",
            EditChatInviteLink::new(1, "example.com/join/chat"),
            |x| {
                assert_eq!(x.invite_link, "example.com/join/chat");
            },
        ),
        (
            "edit-chat-invite-link-all",
            EditChatInviteLink::new(1, "example.com/join/chat")
                .with_name("Link")
                .with_expire_date(0)
                .with_member_limit(1)
                .with_creates_join_request(false),
            |x| {
                assert_eq!(x.invite_link, "example.com/join/chat");
            },
        ),
    ])
    .await;
    cx.execute_batch([
        (
            "edit-chat-subscription-invite-link-base",
            EditChatSubscriptionInviteLink::new(1, "test"),
            |x| {
                assert_eq!(x.invite_link, "example.com/join/chat");
            },
        ),
        (
            "edit-chat-subscription-invite-link-all",
            EditChatSubscriptionInviteLink::new(1, "test").with_name("test"),
            |x| {
                assert_eq!(x.invite_link, "example.com/join/chat");
            },
        ),
    ])
    .await;
    cx.execute_batch([("export-chat-invite-link", ExportChatInviteLink::new(1), |x| {
        assert_eq!(x, "test")
    })])
    .await;
    cx.execute_batch([(
        "revoke-chat-invite-link",
        RevokeChatInviteLink::new(1, "example.com/join/chat"),
        |x| assert!(x.is_revoked),
    )])
    .await;
    cx.execute_batch([
        (
            "answer-chat-join-request-query-approve",
            AnswerChatJoinRequestQuery::approve("id"),
            |x| assert!(x),
        ),
        (
            "answer-chat-join-request-query-decline",
            AnswerChatJoinRequestQuery::decline("id"),
            |x| assert!(x),
        ),
        (
            "answer-chat-join-request-query-queue",
            AnswerChatJoinRequestQuery::queue("id"),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([("approve-chat-join-request", ApproveChatJoinRequest::new(1, 1), |x| {
        assert!(x)
    })])
    .await;
    cx.execute_batch([("decline-chat-join-request", DeclineChatJoinRequest::new(1, 1), |x| {
        assert!(x)
    })])
    .await;
    cx.execute_batch([
        ("ban-chat-member-base", BanChatMember::new(1, 2), |x| assert!(x)),
        (
            "ban-chat-member-all",
            BanChatMember::new(1, 2).with_until_date(3).with_revoke_messages(true),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("get-chat-administrators-base", GetChatAdministrators::new(1), |x| {
            assert_eq!(x.len(), 2);
            assert_eq!(x[0].get_user().id, 1);
            assert!(x[0].is_member());
            let ChatMember::Creator(owner) = &x[0] else {
                panic!("Unexpected member");
            };
            assert_eq!(owner.custom_title.as_ref().unwrap(), "test");
            assert_eq!(x[1].get_user().id, 1);
            assert!(x[1].is_member());
            let ChatMember::Administrator(admin) = &x[1] else {
                panic!("Unexpected member");
            };
            assert!(admin.can_post_messages.unwrap());
            assert!(admin.can_edit_messages.unwrap());
            assert!(admin.can_pin_messages.unwrap());
            assert!(admin.can_manage_topics.unwrap());
            assert!(admin.can_manage_direct_messages.unwrap());
            assert!(admin.can_manage_tags.unwrap());
            assert_eq!(admin.custom_title.as_ref().unwrap(), "test");
        }),
        (
            "get-chat-administrators-all",
            GetChatAdministrators::new(1).with_return_bots(true),
            |x| assert_eq!(x.len(), 2),
        ),
    ])
    .await;
    cx.execute_batch([
        ("get-chat-member-owner", GetChatMember::new(1, 2), |x| {
            assert_eq!(x.get_user().id, 1);
            assert!(x.is_member());
            let ChatMember::Creator(member) = x else {
                panic!("Unexpected chat member")
            };
            assert!(!member.is_anonymous);
            assert!(member.custom_title.is_none());
        }),
        ("get-chat-member-administrator", GetChatMember::new(1, 2), |x| {
            assert_eq!(x.get_user().id, 1);
            assert!(x.is_member());
            let ChatMember::Administrator(member) = x else {
                panic!("Unexpected chat member")
            };
            assert!(member.can_be_edited);
            assert!(member.is_anonymous);
            assert!(member.can_manage_chat);
            assert!(member.can_delete_messages);
            assert!(member.can_manage_video_chats);
            assert!(member.can_restrict_members);
            assert!(member.can_promote_members);
            assert!(member.can_change_info);
            assert!(member.can_invite_users);
            assert!(member.can_post_stories.unwrap());
            assert!(member.can_edit_stories.unwrap());
            assert!(member.can_delete_stories.unwrap());
        }),
        ("get-chat-member-member", GetChatMember::new(1, 2), |x| {
            assert_eq!(x.get_user().id, 1);
            assert!(x.is_member());
            let ChatMember::Member { user, tag, until_date } = x else {
                panic!("Unexpected chat member")
            };
            assert_eq!(user.id, 1);
            assert!(tag.is_none());
            assert!(until_date.is_none());
        }),
        ("get-chat-member-restricted", GetChatMember::new(1, 2), |x| {
            assert_eq!(x.get_user().id, 1);
            assert!(!x.is_member());
            let ChatMember::Restricted(member) = x else {
                panic!("Unexpected chat member")
            };
            assert!(!member.is_member);
            assert!(!member.can_send_messages);
            assert!(!member.can_send_audios);
            assert!(!member.can_send_documents);
            assert!(!member.can_send_photos);
            assert!(!member.can_send_videos);
            assert!(!member.can_send_video_notes);
            assert!(!member.can_send_voice_notes);
            assert!(!member.can_send_polls);
            assert!(!member.can_send_other_messages);
            assert!(!member.can_add_web_page_previews);
            assert!(!member.can_react_to_messages);
            assert!(!member.can_edit_tag);
            assert!(!member.can_change_info);
            assert!(!member.can_invite_users);
            assert!(!member.can_pin_messages);
            assert!(!member.can_manage_topics);
            assert_eq!(member.until_date, 0);
        }),
        ("get-chat-member-left", GetChatMember::new(1, 2), |x| {
            assert_eq!(x.get_user().id, 1);
            assert!(!x.is_member());
            let ChatMember::Left(user) = x else {
                panic!("Unexpected chat member")
            };
            assert_eq!(user.id, 1);
        }),
        ("get-chat-member-banned", GetChatMember::new(1, 2), |x| {
            assert_eq!(x.get_user().id, 1);
            assert!(!x.is_member());
            let ChatMember::Kicked(member) = x else {
                panic!("Unexpected chat member")
            };
            assert_eq!(member.user.id, 1);
            assert_eq!(member.until_date, 0);
        }),
    ])
    .await;
    cx.execute_batch([("get-chat-members-count", GetChatMemberCount::new(1), |x| {
        assert_eq!(x, 2)
    })])
    .await;
    cx.execute_batch([
        ("promote-chat-member-promote-base", PromoteChatMember::new(1, 2), |x| {
            assert!(x)
        }),
        (
            "promote-chat-member-promote-all",
            PromoteChatMember::new(1, 2).promote_all(),
            |x| assert!(x),
        ),
        (
            "promote-chat-member-demote-all",
            PromoteChatMember::new(1, 2).demote_all(),
            |x| assert!(x),
        ),
        (
            "promote-chat-member-custom",
            PromoteChatMember::new(1, 2)
                .with_can_change_info(true)
                .with_can_delete_messages(false)
                .with_can_delete_stories(true)
                .with_can_edit_messages(true)
                .with_can_edit_stories(true)
                .with_can_invite_users(true)
                .with_can_manage_chat(false)
                .with_can_manage_direct_messages(false)
                .with_can_manage_tags(false)
                .with_can_manage_topics(true)
                .with_can_manage_video_chats(true)
                .with_can_pin_messages(true)
                .with_can_post_messages(false)
                .with_can_post_stories(true)
                .with_can_promote_members(false)
                .with_can_restrict_members(false)
                .with_can_send_welcome_messages(true)
                .with_is_anonymous(false),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("restrict-chat-member-base", RestrictChatMember::new(1, 2), |x| {
            assert!(x)
        }),
        (
            "restrict-chat-member-allow-all",
            RestrictChatMember::new(1, 2).allow_all(),
            |x| assert!(x),
        ),
        (
            "restrict-chat-member-restrict-all",
            RestrictChatMember::new(1, 2).restrict_all(),
            |x| assert!(x),
        ),
        (
            "restrict-chat-member-custom",
            RestrictChatMember::new(1, 2)
                .with_permissions(
                    ChatPermissions::default()
                        .with_can_send_messages(true)
                        .with_can_send_other_messages(true)
                        .with_can_add_web_page_previews(false),
                )
                .with_until_date(100)
                .with_use_independent_chat_permissions(true),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([(
        "set-chat-administrator-custom-title",
        SetChatAdministratorCustomTitle::new(1, "Alpha", 1),
        |x| assert!(x),
    )])
    .await;
    cx.execute_batch([
        ("set-chat-member-tag-base", SetChatMemberTag::new(1, 1), |x| assert!(x)),
        (
            "set-chat-member-tag-all",
            SetChatMemberTag::new(1, 1).with_tag("test"),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("unban-chat-member-base", UnbanChatMember::new(1, 2), |x| assert!(x)),
        (
            "unban-chat-member-all",
            UnbanChatMember::new(1, 2).with_only_if_banned(true),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("pin-chat-message-base", PinChatMessage::new(1, 2), |x| assert!(x)),
        (
            "pin-chat-message-all",
            PinChatMessage::new(1, 2)
                .with_business_connection_id("c-id")
                .with_disable_notification(true),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([
        ("unpin-chat-message-base", UnpinChatMessage::new(1), |x| assert!(x)),
        (
            "unpin-chat-message-all",
            UnpinChatMessage::new(1)
                .with_business_connection_id("c-id")
                .with_message_id(2),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([("unpin-all-chat-messages", UnpinAllChatMessages::new(1), |x| assert!(x))])
        .await;
    cx.execute_batch([
        (
            "set-chat-permissions-base",
            SetChatPermissions::new(1, ChatPermissions::default().with_can_send_messages(true)),
            |x| assert!(x),
        ),
        (
            "set-chat-permissions-all",
            SetChatPermissions::new(1, ChatPermissions::default().with_can_send_messages(true))
                .with_use_independent_chat_permissions(true),
            |x| assert!(x),
        ),
    ])
    .await;
    cx.execute_batch([("delete-chat-photo", DeleteChatPhoto::new(1), |x| assert!(x))])
        .await;
    cx.execute_batch([(
        "set-chat-photo",
        SetChatPhoto::new(1, InputFile::file_id("photo-id")),
        |x| assert!(x),
    )])
    .await;
    cx.execute_batch([("ban-chat-sender-chat", BanChatSenderChat::new(1, 1), |x| assert!(x))])
        .await;
    cx.execute_batch([("unban-chat-sender-chat", UnbanChatSenderChat::new(1, 1), |x| assert!(x))])
        .await;
    cx.execute_batch([("delete-chat-sticker-set", DeleteChatStickerSet::new(1), |x| assert!(x))])
        .await;
    cx.execute_batch(
        [("set-chat-sticker-set", SetChatStickerSet::new(1, "Sticker Set"), |x| {
            assert!(x)
        })],
    )
    .await;
}

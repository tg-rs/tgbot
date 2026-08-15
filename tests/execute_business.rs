#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;

    cx.execute_batch([("convert-gift-to-stars", ConvertGiftToStars::new("id", "id"), |x| {
        assert!(x)
    })])
    .await;

    cx.execute_batch([(
        "delete-business-messages",
        DeleteBusinessMessages::new("id", [1, 2, 3]),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([("delete-story", DeleteStory::new("id", 1), |x| assert!(x))])
        .await;

    cx.execute_batch([
        (
            "edit-story-photo-base",
            EditStory::new("id", InputStoryContentPhoto::new(InputFile::url("url")), 1),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.chat.get_id(), 1);
            },
        ),
        (
            "edit-story-photo-all",
            {
                EditStory::new("id", InputStoryContentPhoto::new(InputFile::url("url")), 1)
                    .with_areas([
                        StoryArea::new(
                            StoryAreaTypeLink::new("url"),
                            StoryAreaPosition {
                                corner_radius_percentage: 1.0,
                                height_percentage: 2.0,
                                rotation_angle: 3.0,
                                width_percentage: 4.0,
                                x_percentage: 5.0,
                                y_percentage: 6.0,
                            },
                        ),
                        StoryArea::new(
                            StoryAreaTypeLocation::new(1.0, 2.0),
                            StoryAreaPosition {
                                corner_radius_percentage: 1.0,
                                height_percentage: 2.0,
                                rotation_angle: 3.0,
                                width_percentage: 4.0,
                                x_percentage: 5.0,
                                y_percentage: 6.0,
                            },
                        ),
                        StoryArea::new(
                            StoryAreaTypeSuggestedReaction::new(ReactionType::custom_emoji("test")),
                            StoryAreaPosition {
                                corner_radius_percentage: 1.0,
                                height_percentage: 2.0,
                                rotation_angle: 3.0,
                                width_percentage: 4.0,
                                x_percentage: 5.0,
                                y_percentage: 6.0,
                            },
                        ),
                        StoryArea::new(
                            StoryAreaTypeUniqueGift::new("test"),
                            StoryAreaPosition {
                                corner_radius_percentage: 1.0,
                                height_percentage: 2.0,
                                rotation_angle: 3.0,
                                width_percentage: 4.0,
                                x_percentage: 5.0,
                                y_percentage: 6.0,
                            },
                        ),
                        StoryArea::new(
                            StoryAreaTypeWeather::new(0, "test", 1.0),
                            StoryAreaPosition {
                                corner_radius_percentage: 1.0,
                                height_percentage: 2.0,
                                rotation_angle: 3.0,
                                width_percentage: 4.0,
                                x_percentage: 5.0,
                                y_percentage: 6.0,
                            },
                        ),
                        StoryArea::new(
                            StoryAreaTypeSuggestedReaction::new(ReactionType::custom_emoji("test"))
                                .with_is_dark(true)
                                .with_is_flipped(true),
                            StoryAreaPosition {
                                corner_radius_percentage: 1.0,
                                height_percentage: 2.0,
                                rotation_angle: 3.0,
                                width_percentage: 4.0,
                                x_percentage: 5.0,
                                y_percentage: 6.0,
                            },
                        ),
                        StoryArea::new(
                            StoryAreaTypeLocation::new(1.0, 2.0).with_address(LocationAddress::new("IL")),
                            StoryAreaPosition {
                                corner_radius_percentage: 1.0,
                                height_percentage: 2.0,
                                rotation_angle: 3.0,
                                width_percentage: 4.0,
                                x_percentage: 5.0,
                                y_percentage: 6.0,
                            },
                        ),
                        StoryArea::new(
                            StoryAreaTypeLocation::new(1.0, 2.0).with_address(
                                LocationAddress::new("IL")
                                    .with_state("test")
                                    .with_city("test")
                                    .with_street("test"),
                            ),
                            StoryAreaPosition {
                                corner_radius_percentage: 1.0,
                                height_percentage: 2.0,
                                rotation_angle: 3.0,
                                width_percentage: 4.0,
                                x_percentage: 5.0,
                                y_percentage: 6.0,
                            },
                        ),
                    ])
                    .with_caption(
                        InputText::from("test")
                            .with_format(ParseMode::Markdown)
                            .with_format([TextEntity::bold(0..2)]),
                    )
            },
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.chat.get_id(), 1);
            },
        ),
        (
            "edit-story-video-base",
            EditStory::new(
                "id",
                InputStoryContentVideo::new(InputFile::file_id("video-file-id")),
                1,
            ),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.chat.get_id(), 1);
            },
        ),
        (
            "edit-story-video-all",
            EditStory::new(
                "id",
                InputStoryContentVideo::new(InputFile::file_id("video-file-id"))
                    .with_cover_frame_timestamp(0.0)
                    .with_duration(20.0)
                    .with_is_animation(true),
                1,
            ),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.chat.get_id(), 1);
            },
        ),
    ])
    .await;

    cx.execute_batch([(
        "get-business-account-star-balance",
        GetBusinessAccountStarBalance::new("id"),
        |x| {
            assert_eq!(x.amount, 10);
            assert_eq!(x.nanostar_amount.unwrap(), 1);
        },
    )])
    .await;

    cx.execute_batch([
        ("get-business-connection-base", GetBusinessConnection::new("id"), |x| {
            assert_eq!(x.id, "test");
            assert_eq!(x.user.id, 1);
            assert_eq!(x.user_chat_id, 1);
            assert_eq!(x.date, 0);
            assert!(x.is_enabled);
            assert!(x.rights.is_none());
        }),
        ("get-business-connection-all", GetBusinessConnection::new("id"), |x| {
            assert_eq!(x.id, "test");
            assert_eq!(x.user.id, 1);
            assert_eq!(x.user_chat_id, 1);
            assert_eq!(x.date, 0);
            assert!(x.is_enabled);
            let rights = x.rights.unwrap();
            assert!(rights.can_reply.unwrap());
            assert!(rights.can_read_messages.unwrap());
            assert!(rights.can_delete_sent_messages.unwrap());
            assert!(rights.can_delete_all_messages.unwrap());
            assert!(rights.can_edit_name.unwrap());
            assert!(rights.can_edit_bio.unwrap());
            assert!(rights.can_edit_profile_photo.unwrap());
            assert!(rights.can_edit_username.unwrap());
            assert!(rights.can_change_gift_settings.unwrap());
            assert!(rights.can_view_gifts_and_stars.unwrap());
            assert!(rights.can_convert_gifts_to_stars.unwrap());
            assert!(rights.can_transfer_and_upgrade_gifts.unwrap());
            assert!(rights.can_transfer_stars.unwrap());
            assert!(rights.can_manage_stories.unwrap());
        }),
    ])
    .await;

    cx.execute_batch([
        (
            "post-story-base",
            PostStory::new(60, "id", InputStoryContentPhoto::new(InputFile::url("url"))),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.chat.get_id(), 1);
                assert_eq!(x.chat.get_username().unwrap(), "test");
            },
        ),
        (
            "post-story-all",
            PostStory::new(60, "id", InputStoryContentPhoto::new(InputFile::url("url")))
                .with_areas([StoryArea::new(
                    StoryAreaTypeLink::new("url"),
                    StoryAreaPosition {
                        corner_radius_percentage: 1.0,
                        height_percentage: 2.0,
                        rotation_angle: 3.0,
                        width_percentage: 4.0,
                        x_percentage: 5.0,
                        y_percentage: 6.0,
                    },
                )])
                .with_caption(
                    InputText::from("test")
                        .with_format(ParseMode::Markdown)
                        .with_format([TextEntity::bold(0..2)]),
                )
                .with_post_to_chat_page(true)
                .with_protect_content(true),
            |x| {
                assert_eq!(x.id, 1);
                assert_eq!(x.chat.get_id(), 1);
            },
        ),
    ])
    .await;

    cx.execute_batch([("read-business-message", ReadBusinessMessage::new("id", 1, 2), |x| {
        assert!(x)
    })])
    .await;

    cx.execute_batch([
        (
            "remove-business-account-profile-photo-base",
            RemoveBusinessAccountProfilePhoto::new("id"),
            |x| assert!(x),
        ),
        (
            "remove-business-account-profile-photo-all",
            RemoveBusinessAccountProfilePhoto::new("id").with_is_public(true),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        ("set-business-account-bio-base", SetBusinessAccountBio::new("id"), |x| {
            assert!(x)
        }),
        (
            "set-business-account-bio-all",
            SetBusinessAccountBio::new("id").with_bio("Test"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([(
        "set-business-account-gift-settings",
        SetBusinessAccountGiftSettings::new("id", true, AcceptedGiftTypes::default()),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([
        (
            "set-business-account-name-base",
            SetBusinessAccountName::new("id", "John"),
            |x| assert!(x),
        ),
        (
            "set-business-account-name-all",
            SetBusinessAccountName::new("id", "John").with_last_name("Doe"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "set-business-account-profile-photo-static-base",
            SetBusinessAccountProfilePhoto::new("id", InputProfilePhotoStatic::new(InputFile::url("test"))),
            |x| assert!(x),
        ),
        (
            "set-business-account-profile-photo-static-all",
            SetBusinessAccountProfilePhoto::new("id", InputProfilePhotoStatic::new(InputFile::url("test")))
                .with_is_public(true),
            |x| assert!(x),
        ),
        (
            "set-business-account-profile-photo-animated-base",
            SetBusinessAccountProfilePhoto::new("id", InputProfilePhotoAnimated::new(InputFile::url("test"))),
            |x| assert!(x),
        ),
        (
            "set-business-account-profile-photo-animated-all",
            SetBusinessAccountProfilePhoto::new(
                "id",
                InputProfilePhotoAnimated::new(InputFile::url("test")).with_main_frame_timestamp(3.0),
            )
            .with_is_public(true),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        (
            "set-business-account-username-base",
            SetBusinessAccountUsername::new("id"),
            |x| assert!(x),
        ),
        (
            "set-business-account-username-all",
            SetBusinessAccountUsername::new("id").with_username("johndoe"),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([(
        "transfer-business-account-stars",
        TransferBusinessAccountStars::new("id", 1),
        |x| assert!(x),
    )])
    .await;

    cx.execute_batch([
        ("transfer-gift-base", TransferGift::new("id", "id", 1), |x| assert!(x)),
        (
            "transfer-gift-all",
            TransferGift::new("id", "id", 1).with_star_count(1),
            |x| assert!(x),
        ),
    ])
    .await;

    cx.execute_batch([
        ("upgrade-gift-base", UpgradeGift::new("id", "id"), |x| assert!(x)),
        (
            "upgrade-gift-all",
            UpgradeGift::new("id", "id")
                .with_keep_original_details(true)
                .with_star_count(1),
            |x| assert!(x),
        ),
    ])
    .await;
}

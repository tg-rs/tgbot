use crate::types::*;

#[test]
fn business_bot_rights() {
    let expected_struct = BusinessBotRights::default();
    insta::assert_json_snapshot!(expected_struct);
    insta::assert_json_snapshot!(
        expected_struct
            .with_can_change_gift_settings(true)
            .with_can_convert_gifts_to_stars(true)
            .with_can_delete_all_messages(true)
            .with_can_delete_outgoing_messages(true)
            .with_can_edit_bio(true)
            .with_can_edit_name(true)
            .with_can_edit_profile_photo(true)
            .with_can_edit_username(true)
            .with_can_manage_stories(true)
            .with_can_read_messages(true)
            .with_can_reply(true)
            .with_can_transfer_and_upgrade_gifts(true)
            .with_can_transfer_stars(true)
            .with_can_view_gifts_and_stars(true)
    );
}

#[test]
fn business_connection() {
    let expected_struct = BusinessConnection::new(0, "id", User::new(1, "test", false), 2);
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_rights(BusinessBotRights::default())
            .with_is_enabled(true)
    );
}

#[test]
fn business_intro() {
    insta::assert_json_snapshot!(BusinessIntro::default());
    insta::assert_json_snapshot!(
        BusinessIntro::default()
            .with_message("msg")
            .with_sticker(Sticker::new(
                "file-id",
                "file-unique-id",
                StickerType::Regular,
                512,
                512,
            ))
            .with_title("title")
    );
}

#[test]
fn business_location() {
    insta::assert_json_snapshot!(BusinessLocation::new("address"));
    insta::assert_json_snapshot!(BusinessLocation::new("address").with_location(Location::new(1.0, 2.0)));
}

#[test]
fn business_messages_deleted() {
    insta::assert_json_snapshot!(BusinessMessagesDeleted::new("id", PrivateChat::new(1, "test"), [2]));
}

#[test]
fn business_opening_hours() {
    insta::assert_json_snapshot!(BusinessOpeningHours::new("UTC", [(1, 2), (3, 4)]));
}

#[test]
fn convert_gift_to_stars() {
    assert_payload_eq!(POST JSON "convertGiftToStars" => ConvertGiftToStars::new("id", "id"));
}

#[test]
fn delete_business_messages() {
    assert_payload_eq!(POST JSON "deleteBusinessMessages" => DeleteBusinessMessages::new("id", [1, 2, 3]));
}

#[test]
fn delete_story() {
    assert_payload_eq!(POST JSON "deleteStory" => DeleteStory::new("id", 1));
}

#[test]
fn edit_story() {
    let method = EditStory::new("id", InputStoryContentPhoto::new(InputFile::url("url")), 1).unwrap();
    assert_payload_eq!(POST FORM "editStory" => method);
    let method = EditStory::new("id", InputStoryContentPhoto::new(InputFile::url("url")), 1)
        .unwrap()
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
        .unwrap()
        .with_caption("test")
        .with_parse_mode(ParseMode::Markdown)
        .with_caption_entities([TextEntity::bold(0..2)])
        .unwrap();
    assert_payload_eq!(POST FORM "editStory" => method);
}

#[test]
fn get_business_account_gifts() {
    let method = GetBusinessAccountGifts::new("id");
    assert_payload_eq!(POST JSON "getBusinessAccountGifts" => method.clone());
    let method = method
        .with_exclude_limited_non_upgradable(true)
        .with_exclude_limited_upgradable(false)
        .with_exclude_saved(true)
        .with_exclude_unique(false)
        .with_exclude_unlimited(true)
        .with_exclude_unsaved(false)
        .with_limit(10)
        .with_offset("test")
        .with_sort_by_price(true);
    assert_payload_eq!(POST JSON "getBusinessAccountGifts" => method);
}

#[test]
fn get_business_account_star_balance() {
    assert_payload_eq!(POST JSON "getBusinessAccountStarBalance" => GetBusinessAccountStarBalance::new("id"));
}

#[test]
fn get_business_connection() {
    assert_payload_eq!(POST JSON "getBusinessConnection" => GetBusinessConnection::new("id"));
}

#[test]
fn post_story() {
    let method = PostStory::new(60, "id", InputStoryContentPhoto::new(InputFile::url("url"))).unwrap();
    assert_payload_eq!(POST FORM "postStory" => method);
    let method = PostStory::new(60, "id", InputStoryContentPhoto::new(InputFile::url("url")))
        .unwrap()
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
        .unwrap()
        .with_caption("test")
        .with_parse_mode(ParseMode::Markdown)
        .with_caption_entities([TextEntity::bold(0..2)])
        .unwrap()
        .with_post_to_chat_page(true)
        .with_protect_content(true);
    assert_payload_eq!(POST FORM "postStory" => method);
}

#[test]
fn read_business_message() {
    assert_payload_eq!(POST JSON "readBusinessMessage" => ReadBusinessMessage::new("id", 1, 2));
}

#[test]
fn remove_business_account_profile_photo() {
    let method = RemoveBusinessAccountProfilePhoto::new("id");
    assert_payload_eq!(POST JSON "removeBusinessAccountProfilePhoto" => method.clone());
    assert_payload_eq!(POST JSON "removeBusinessAccountProfilePhoto" => method.with_is_public(true));
}

#[test]
fn set_business_account_bio() {
    let method = SetBusinessAccountBio::new("id");
    assert_payload_eq!(POST JSON "setBusinessAccountBio" => method.clone());
    assert_payload_eq!(POST JSON "setBusinessAccountBio" => method.with_bio("Test"));
}

#[test]
fn set_business_account_gift_settings() {
    let method = SetBusinessAccountGiftSettings::new("id", true, AcceptedGiftTypes::default());
    assert_payload_eq!(POST JSON "setBusinessAccountGiftSettings" => method.clone());
}

#[test]
fn set_business_account_name() {
    let method = SetBusinessAccountName::new("id", "John");
    assert_payload_eq!(POST JSON "setBusinessAccountName" => method.clone());
    assert_payload_eq!(POST JSON "setBusinessAccountName" => method.with_last_name("Doe"));
}

#[test]
fn set_business_account_profile_photo() {
    let method =
        SetBusinessAccountProfilePhoto::new("id", InputProfilePhotoStatic::new(InputFile::url("test"))).unwrap();
    assert_payload_eq!(POST FORM "setBusinessAccountProfilePhoto" => method);

    let method = SetBusinessAccountProfilePhoto::new("id", InputProfilePhotoStatic::new(InputFile::url("test")))
        .unwrap()
        .with_is_public(true);
    assert_payload_eq!(POST FORM "setBusinessAccountProfilePhoto" => method);
}

#[test]
fn set_business_account_username() {
    let method = SetBusinessAccountUsername::new("id");
    assert_payload_eq!(POST JSON "setBusinessAccountUsername" => method.clone());
    assert_payload_eq!(POST JSON "setBusinessAccountUsername" => method.with_username("johndoe"));
}

#[test]
fn transfer_business_account_stars() {
    let method = TransferBusinessAccountStars::new("id", 1);
    assert_payload_eq!(POST JSON "transferBusinessAccountStars" => method);
}

#[test]
fn transfer_gift() {
    let method = TransferGift::new("id", "id", 1);
    assert_payload_eq!(POST JSON "transferGift" => method.clone());
    assert_payload_eq!(POST JSON "transferGift" => method.with_star_count(1));
}

#[test]
fn upgrade_gift() {
    let method = UpgradeGift::new("id", "id");
    assert_payload_eq!(POST JSON "upgradeGift" => method.clone());
    let method = method.with_keep_original_details(true).with_star_count(1);
    assert_payload_eq!(POST JSON "upgradeGift" => method);
}

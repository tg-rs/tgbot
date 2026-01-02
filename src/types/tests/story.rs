use crate::types::*;

#[test]
fn story() {
    let chat = PrivateChat::new(1, "test");
    insta::assert_json_snapshot!(Story::new(chat, 1));
}

#[test]
fn story_area() {
    let expected_struct = StoryArea::new(
        StoryAreaTypeLink::new("url"),
        StoryAreaPosition {
            corner_radius_percentage: 1.0,
            height_percentage: 2.0,
            rotation_angle: 3.0,
            width_percentage: 4.0,
            x_percentage: 5.0,
            y_percentage: 6.0,
        },
    );
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn story_area_type() {
    let expected_struct = StoryAreaType::from(StoryAreaTypeLink::new("url"));
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = StoryAreaType::from(StoryAreaTypeLocation::new(1.0, 2.0));
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct =
        StoryAreaType::from(StoryAreaTypeLocation::new(1.0, 2.0).with_address(LocationAddress::new("RU")));
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = StoryAreaType::from(StoryAreaTypeSuggestedReaction::new(ReactionType::Paid));
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = StoryAreaType::from(
        StoryAreaTypeSuggestedReaction::new(ReactionType::Paid)
            .with_is_dark(true)
            .with_is_flipped(true),
    );
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = StoryAreaType::from(StoryAreaTypeUniqueGift::new("test"));
    insta::assert_json_snapshot!(expected_struct);
    let expected_struct = StoryAreaType::from(StoryAreaTypeWeather::new(1, "test", 2.0));
    insta::assert_json_snapshot!(expected_struct);
}

#[test]
fn repost_story() {
    let method = RepostStory::new(1, "test", 2, 3);
    assert_payload_eq!(POST JSON "repostStory" => method.clone());
    let method = method.with_post_to_chat_page(true).with_protect_content(true);
    assert_payload_eq!(POST JSON "repostStory" => method);
}

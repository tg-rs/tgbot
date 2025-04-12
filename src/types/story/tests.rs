use crate::types::{
    LocationAddress,
    PrivateChat,
    ReactionType,
    Story,
    StoryArea,
    StoryAreaPosition,
    StoryAreaType,
    StoryAreaTypeLink,
    StoryAreaTypeLocation,
    StoryAreaTypeSuggestedReaction,
    StoryAreaTypeUniqueGift,
    StoryAreaTypeWeather,
    tests::assert_json_eq,
};

#[test]
fn story() {
    let chat = PrivateChat::new(1, "test");
    assert_json_eq(
        Story::new(chat, 1),
        serde_json::json!({
            "chat": {
                "first_name": "test",
                "id": 1,
                "type": "private"
            },
            "id": 1
        }),
    );
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
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": {
                "type": "link",
                "url": "url",
            },
            "position": {
                "corner_radius_percentage": 1.0,
                "height_percentage": 2.0,
                "rotation_angle": 3.0,
                "width_percentage": 4.0,
                "x_percentage": 5.0,
                "y_percentage": 6.0,
            },
        }),
    );
}

#[test]
fn story_area_type() {
    let expected_struct = StoryAreaType::from(StoryAreaTypeLink::new("url"));
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "link",
            "url": "url",
        }),
    );
    let expected_struct = StoryAreaType::from(StoryAreaTypeLocation::new(1.0, 2.0));
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "location",
            "latitude": 1.0,
            "longitude": 2.0,
        }),
    );
    let expected_struct =
        StoryAreaType::from(StoryAreaTypeLocation::new(1.0, 2.0).with_address(LocationAddress::new("RU")));
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "location",
            "latitude": 1.0,
            "longitude": 2.0,
            "address": {
                "country_code": "RU",
            },
        }),
    );
    let expected_struct = StoryAreaType::from(StoryAreaTypeSuggestedReaction::new(ReactionType::Paid));
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "suggested_reaction",
            "reaction_type": {
                "type": "paid"
            },
        }),
    );
    let expected_struct = StoryAreaType::from(
        StoryAreaTypeSuggestedReaction::new(ReactionType::Paid)
            .with_is_dark(true)
            .with_is_flipped(true),
    );
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "suggested_reaction",
            "reaction_type": {
                "type": "paid"
            },
            "is_dark": true,
            "is_flipped": true,
        }),
    );
    let expected_struct = StoryAreaType::from(StoryAreaTypeUniqueGift::new("test"));
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "unique_gift",
            "name": "test",
        }),
    );
    let expected_struct = StoryAreaType::from(StoryAreaTypeWeather::new(1, "test", 2.0));
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "type": "weather",
            "background_color": 1,
            "emoji": "test",
            "temperature": 2.0,
        }),
    );
}

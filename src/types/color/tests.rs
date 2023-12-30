use crate::types::{tests::assert_json_eq, ForumTopicIconColor, Integer};

#[test]
fn forum_topic_icon_color() {
    for (expected_struct, expected_value) in [
        (ForumTopicIconColor::BakerMillerPink, 16749490),
        (ForumTopicIconColor::Bittersweet, 16478047),
        (ForumTopicIconColor::BrightLavender, 13338331),
        (ForumTopicIconColor::Jasmine, 16766590),
        (ForumTopicIconColor::LightGreen, 9367192),
        (ForumTopicIconColor::VeryLightAzure, 7322096),
        (ForumTopicIconColor::Unknown(0), 0),
    ] {
        assert_eq!(Integer::from(expected_struct), expected_value);
        assert_eq!(ForumTopicIconColor::from(expected_value), expected_struct);
        assert_json_eq(expected_struct, serde_json::json!(expected_value));
    }
}

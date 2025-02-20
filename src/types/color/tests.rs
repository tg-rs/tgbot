use crate::types::{AccentColor, ForumTopicIconColor, Integer, ProfileAccentColor, tests::assert_json_eq};

#[test]
fn accent_color() {
    for (expected_struct, expected_value) in [
        (AccentColor::Red, 0),
        (AccentColor::Orange, 1),
        (AccentColor::Purple, 2),
        (AccentColor::Green, 3),
        (AccentColor::Cyan, 4),
        (AccentColor::Blue, 5),
        (AccentColor::Pink, 6),
    ] {
        assert_eq!(Integer::from(expected_struct), expected_value);
        assert_eq!(AccentColor::try_from(expected_value).unwrap(), expected_struct);
        assert_json_eq(expected_struct, serde_json::json!(expected_value))
    }
    for i in 7..=20 {
        let expected_struct = AccentColor::try_from(i).unwrap();
        assert_eq!(Integer::from(expected_struct), i);
        assert_json_eq(expected_struct, serde_json::json!(i));
    }
    assert!(AccentColor::try_from(-1).is_err());
    assert!(AccentColor::try_from(20).is_ok());
    assert!(AccentColor::try_from(21).is_err());
}

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

#[test]
fn profile_accent_color() {
    for i in 0..=15 {
        let expected_struct = ProfileAccentColor::try_from(i).unwrap();
        assert_eq!(Integer::from(expected_struct), i);
        assert_json_eq(expected_struct, serde_json::json!(i));
    }
    assert!(ProfileAccentColor::try_from(-1).is_err());
    assert!(ProfileAccentColor::try_from(16).is_err());
}

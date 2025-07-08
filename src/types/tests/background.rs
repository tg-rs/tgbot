use crate::types::*;

#[test]
fn background_fill() {
    insta::assert_json_snapshot!(BackgroundFill::FreeformGradient { colors: vec![1, 2, 3] });
    insta::assert_json_snapshot!(BackgroundFill::Gradient {
        bottom_color: 1,
        rotation_angle: 90,
        top_color: 2,
    });
    insta::assert_json_snapshot!(BackgroundFill::Solid { color: 1 });
}

#[test]
fn background_type() {
    insta::assert_json_snapshot!(BackgroundType::ChatTheme {
        theme_name: String::from("test"),
    });
    insta::assert_json_snapshot!(BackgroundType::Fill {
        dark_theme_dimming: 4,
        fill: BackgroundFill::Solid { color: 1 },
    });
    insta::assert_json_snapshot!(BackgroundType::Pattern {
        document: Document::new("file-id", "file-unique-id"),
        fill: BackgroundFill::Solid { color: 1 },
        intensity: 0,
        is_inverted: None,
        is_moving: None,
    });
    insta::assert_json_snapshot!(BackgroundType::Pattern {
        document: Document::new("file-id", "file-unique-id"),
        fill: BackgroundFill::Solid { color: 1 },
        intensity: 0,
        is_inverted: Some(true),
        is_moving: Some(false),
    });
    insta::assert_json_snapshot!(BackgroundType::Wallpaper {
        dark_theme_dimming: 100,
        document: Document::new("file-id", "file-unique-id"),
        is_blurred: None,
        is_moving: None,
    });
    insta::assert_json_snapshot!(BackgroundType::Wallpaper {
        dark_theme_dimming: 100,
        document: Document::new("file-id", "file-unique-id"),
        is_blurred: Some(true),
        is_moving: Some(false),
    });
}

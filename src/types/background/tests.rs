use crate::types::{BackgroundFill, BackgroundType, Document, tests::assert_json_eq};

#[test]
fn background_fill() {
    assert_json_eq(
        BackgroundFill::FreeformGradient { colors: vec![1, 2, 3] },
        serde_json::json!({
            "type": "freeform_gradient",
            "colors": [1, 2, 3]
        }),
    );
    assert_json_eq(
        BackgroundFill::Gradient {
            bottom_color: 1,
            rotation_angle: 90,
            top_color: 2,
        },
        serde_json::json!({
            "type": "gradient",
            "bottom_color": 1,
            "rotation_angle": 90,
            "top_color": 2
        }),
    );
    assert_json_eq(
        BackgroundFill::Solid { color: 1 },
        serde_json::json!({
            "type": "solid",
            "color": 1,
        }),
    );
}

#[test]
fn background_type() {
    assert_json_eq(
        BackgroundType::ChatTheme {
            theme_name: String::from("test"),
        },
        serde_json::json!({
            "type": "chat_theme",
            "theme_name": "test",
        }),
    );
    assert_json_eq(
        BackgroundType::Fill {
            dark_theme_dimming: 4,
            fill: BackgroundFill::Solid { color: 1 },
        },
        serde_json::json!({
            "type": "fill",
            "dark_theme_dimming": 4,
            "fill": {
                "type": "solid",
                "color": 1,
            },
        }),
    );
    assert_json_eq(
        BackgroundType::Pattern {
            document: Document::new("file-id", "file-unique-id"),
            fill: BackgroundFill::Solid { color: 1 },
            intensity: 0,
            is_inverted: None,
            is_moving: None,
        },
        serde_json::json!({
            "type": "pattern",
            "document": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
            },
            "fill": {
                "type": "solid",
                "color": 1,
            },
            "intensity": 0,
        }),
    );
    assert_json_eq(
        BackgroundType::Pattern {
            document: Document::new("file-id", "file-unique-id"),
            fill: BackgroundFill::Solid { color: 1 },
            intensity: 0,
            is_inverted: Some(true),
            is_moving: Some(false),
        },
        serde_json::json!({
            "type": "pattern",
            "document": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
            },
            "fill": {
                "type": "solid",
                "color": 1,
            },
            "intensity": 0,
            "is_inverted": true,
            "is_moving": false,
        }),
    );
    assert_json_eq(
        BackgroundType::Wallpaper {
            dark_theme_dimming: 100,
            document: Document::new("file-id", "file-unique-id"),
            is_blurred: None,
            is_moving: None,
        },
        serde_json::json!({
            "type": "wallpaper",
            "dark_theme_dimming": 100,
            "document": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
            },
        }),
    );
    assert_json_eq(
        BackgroundType::Wallpaper {
            dark_theme_dimming: 100,
            document: Document::new("file-id", "file-unique-id"),
            is_blurred: Some(true),
            is_moving: Some(false),
        },
        serde_json::json!({
            "type": "wallpaper",
            "dark_theme_dimming": 100,
            "document": {
                "file_id": "file-id",
                "file_unique_id": "file-unique-id",
            },
            "is_blurred": true,
            "is_moving": false,
        }),
    );
}

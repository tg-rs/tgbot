use std::io::Cursor;

use serde_json::Value;

use crate::{
    api::Form,
    types::{InputFile, InputSticker, InputStickers, MaskPosition, MaskPositionPoint},
};

#[test]
fn input_sticker() {
    let value = InputSticker::new(InputFile::file_id("file-id"), ["😻"]);
    let form: Form = value.try_into().unwrap();
    let actual_value: Value = serde_json::from_str(form.get_field("sticker").unwrap().get_text().unwrap()).unwrap();
    let expected_value = serde_json::json!({
        "sticker": "file-id",
        "emoji_list": ["😻"]
    });
    assert_eq!(actual_value, expected_value);

    let value = InputSticker::new(InputFile::url("https://google.com/favicon.ico"), ["😻"])
        .with_keywords(["kw"])
        .with_mask_position(MaskPosition {
            point: MaskPositionPoint::Forehead,
            x_shift: 1.0,
            y_shift: 2.0,
            scale: 3.0,
        });
    let form: Form = value.try_into().unwrap();
    let actual_value: Value = serde_json::from_str(form.get_field("sticker").unwrap().get_text().unwrap()).unwrap();
    let expected_value = serde_json::json!({
        "sticker": "https://google.com/favicon.ico",
        "emoji_list": ["😻"],
        "keywords": ["kw"],
        "mask_position": {
            "point": "forehead",
            "x_shift": 1.0,
            "y_shift": 2.0,
            "scale": 3.0
        }
    });
    assert_eq!(actual_value, expected_value);

    let value = InputSticker::new(Cursor::new("test"), ["😻"]);
    let form: Form = value.try_into().unwrap();
    assert!(form.has_field("tgbot_input_sticker"));
    let actual_value: Value = serde_json::from_str(form.get_field("sticker").unwrap().get_text().unwrap()).unwrap();
    let expected_value = serde_json::json!({
        "sticker": "attach://tgbot_input_sticker",
        "emoji_list": ["😻"]
    });
    assert_eq!(actual_value, expected_value);
}

#[test]
fn input_stickers() {
    let value = InputStickers::default()
        .with(InputSticker::new(InputFile::file_id("file-id"), ["😻"]))
        .with(InputSticker::new(
            InputFile::url("https://google.com/favicon.ico"),
            ["😻"],
        ));
    let form: Form = value.try_into().unwrap();
    let actual_value: Value = serde_json::from_str(form.get_field("stickers").unwrap().get_text().unwrap()).unwrap();
    let expected_value = serde_json::json!([
        {
            "sticker": "file-id",
            "emoji_list": ["😻"]
        },
        {
            "sticker": "https://google.com/favicon.ico",
            "emoji_list": ["😻"]
        }
    ]);
    assert_eq!(expected_value, actual_value);
}

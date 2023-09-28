use serde_json::Value;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{
        AddStickerToSet,
        CreateNewStickerSet,
        DeleteStickerFromSet,
        GetStickerSet,
        InputFile,
        MaskPosition,
        MaskPositionPoint,
        NewSticker,
        SetStickerPositionInSet,
        SetStickerSetThumb,
        StickerSet,
    },
};

#[test]
fn sticker_set_deserialize() {
    let data: StickerSet = serde_json::from_value(serde_json::json!({
        "name": "test",
        "title": "test",
        "contains_masks": false,
        "stickers": [],
        "is_animated": false,
        "is_video": false,
        "thumb": {
            "file_id": "thumb-file-id",
            "file_unique_id": "thumb-file-unique-id",
            "width": 512,
            "height": 512,
            "file_size": 2048,
        }
    }))
    .unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.title, "test");
    assert!(!data.is_animated);
    assert!(!data.is_video);
    assert!(!data.contains_masks);
    assert!(data.stickers.is_empty());

    let thumb = data.thumb.unwrap();
    assert_eq!(thumb.file_id, "thumb-file-id");
    assert_eq!(thumb.file_unique_id, "thumb-file-unique-id");
    assert_eq!(thumb.width, 512);
    assert_eq!(thumb.height, 512);
    assert_eq!(thumb.file_size.unwrap(), 2048);
}

#[test]
fn add_sticker_to_set_png() {
    let request = AddStickerToSet::new(1, "name", NewSticker::png(InputFile::file_id("sticker-id")), "^_^")
        .mask_position(MaskPosition {
            point: MaskPositionPoint::Forehead,
            x_shift: 1.0,
            y_shift: 2.0,
            scale: 3.0,
        })
        .unwrap()
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/addStickerToSet"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
        assert_eq!(form.fields["name"].get_text().unwrap(), "name");
        assert!(form.fields["png_sticker"].get_file().is_some());
        assert_eq!(form.fields["emojis"].get_text().unwrap(), "^_^");
        assert!(form.fields["mask_position"].get_text().is_some());
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn add_sticker_to_set_tgs() {
    let request =
        AddStickerToSet::new(1, "name", NewSticker::tgs(InputFile::file_id("sticker-id")), "^_^").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/addStickerToSet"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
        assert_eq!(form.fields["name"].get_text().unwrap(), "name");
        assert!(form.fields["tgs_sticker"].get_file().is_some());
        assert_eq!(form.fields["emojis"].get_text().unwrap(), "^_^");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn delete_sticker_from_set() {
    let request = DeleteStickerFromSet::new("sticker").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/deleteStickerFromSet"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["sticker"], "sticker");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn get_sticker_set() {
    let request = GetStickerSet::new("name").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/getStickerSet"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["name"], "name");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn create_new_sticker_set_with_png() {
    let request = CreateNewStickerSet::new(
        1,
        "name",
        "title",
        NewSticker::png(InputFile::file_id("sticker-id")),
        "^_^",
    )
    .contains_masks(true)
    .mask_position(MaskPosition {
        point: MaskPositionPoint::Forehead,
        x_shift: 1.0,
        y_shift: 2.0,
        scale: 3.0,
    })
    .unwrap()
    .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/createNewStickerSet"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
        assert_eq!(form.fields["name"].get_text().unwrap(), "name");
        assert_eq!(form.fields["title"].get_text().unwrap(), "title");
        assert!(form.fields["png_sticker"].get_file().is_some());
        assert_eq!(form.fields["emojis"].get_text().unwrap(), "^_^");
        assert!(form.fields["mask_position"].get_text().is_some());
        assert_eq!(form.fields["contains_masks"].get_text().unwrap(), "true");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn create_new_sticker_set_with_tgs() {
    let request = CreateNewStickerSet::new(
        1,
        "name",
        "title",
        NewSticker::tgs(InputFile::file_id("sticker-id")),
        "^_^",
    )
    .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/createNewStickerSet"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
        assert_eq!(form.fields["name"].get_text().unwrap(), "name");
        assert_eq!(form.fields["title"].get_text().unwrap(), "title");
        assert!(form.fields["tgs_sticker"].get_file().is_some());
        assert_eq!(form.fields["emojis"].get_text().unwrap(), "^_^");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn set_sticker_position_in_set() {
    let request = SetStickerPositionInSet::new("sticker", 1).into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/setStickerPositionInSet"
    );
    if let RequestBody::Json(data) = request.into_body() {
        let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(data["sticker"], "sticker");
        assert_eq!(data["position"], 1);
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn set_sticker_set_thumb() {
    let request = SetStickerSetThumb::new("name", 1)
        .thumb(InputFile::file_id("file-id"))
        .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/setStickerSetThumb"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["name"].get_text().unwrap(), "name");
        assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
        assert!(form.fields["thumb"].get_file().is_some());
    } else {
        panic!("Unexpected request body");
    }
}

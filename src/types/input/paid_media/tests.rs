use std::io::Cursor;

use crate::{
    api::Form,
    types::{InputFile, InputPaidMediaGroup, InputPaidMediaGroupError, InputPaidMediaGroupItem, InputPaidMediaVideo},
};

#[test]
fn input_paid_media_group_error() {
    let err = InputPaidMediaGroup::new(vec![]).unwrap_err();
    assert!(matches!(err, InputPaidMediaGroupError::NotEnoughItems(1)));
    let err = InputPaidMediaGroup::new(vec![
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id")),
    ])
    .unwrap_err();
    assert!(matches!(err, InputPaidMediaGroupError::TooManyItems(10)));
}

#[test]
fn input_paid_media_group() {
    let group = InputPaidMediaGroup::new(vec![
        InputPaidMediaGroupItem::for_photo(InputFile::file_id("photo-file-id")),
        InputPaidMediaGroupItem::for_video(
            InputFile::url("https://example.com/video-file-1.mp4"),
            InputPaidMediaVideo::default(),
        )
        .with_thumbnail(InputFile::file_id("video-1-thumbnail-id")),
        InputPaidMediaGroupItem::for_video(
            InputFile::from(Cursor::new("video-file-2-data")),
            InputPaidMediaVideo::default()
                .with_duration(1)
                .with_height(2)
                .with_start_timestamp(20)
                .with_supports_streaming(true)
                .with_width(3),
        )
        .with_cover(InputFile::url("cover-url"))
        .with_thumbnail(InputFile::from(Cursor::new("video-2-thumbnail-data"))),
    ])
    .unwrap();
    let actual_form: Form = group.into();
    let mut media = String::from("[{\"type\":\"photo\",\"media\":\"photo-file-id\"}");
    media += ",{\"type\":\"video\",\"media\":\"https://example.com/video-file-1.mp4\",\"thumbnail\":\"video-1-thumbnail-id\"}";
    media += ",{\"type\":\"video\",\"media\":\"attach://tgbot_ipm_file_2\",\"cover\":\"cover-url\",";
    media += "\"thumbnail\":\"attach://tgbot_ipm_thumb_2\",\"duration\":1,\"height\":2,";
    media += "\"start_timestamp\":20,\"supports_streaming\":true,\"width\":3}]";
    let expected_form = Form::from([
        ("media", media.into()),
        (
            "tgbot_ipm_thumb_2",
            InputFile::from(Cursor::new("video-2-thumbnail-data")).into(),
        ),
        (
            "tgbot_ipm_file_2",
            InputFile::from(Cursor::new("video-file-2-data")).into(),
        ),
    ]);
    assert_eq!(actual_form, expected_form);
}

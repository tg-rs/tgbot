use std::io::Cursor;

use crate::{
    api::Form,
    types::{
        InputFile,
        InputFileReader,
        InputMedia,
        InputMediaAnimation,
        InputMediaAudio,
        InputMediaDocument,
        InputMediaPhoto,
        InputMediaVideo,
    },
};

#[test]
fn input_media_form() {
    let data: Form = InputMedia::new(
        InputFile::file_id("animation-file-id"),
        InputMediaAnimation::default().caption("test"),
    )
    .unwrap()
    .into();
    assert!(data.has_field("media"));

    let data: Form = InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("animation-file-data")),
        InputFileReader::from(Cursor::new("animation-thumb-data")),
        InputMediaAnimation::default(),
    )
    .unwrap()
    .into();
    assert!(data.has_field("tgbot_im_file"));
    assert!(data.has_field("tgbot_im_thumb"));
    assert!(data.has_field("media"));
}

#[test]
fn create_input_media() {
    InputMedia::new(
        InputFile::file_id("animation-file-id"),
        InputMediaAnimation::default().caption("test"),
    )
    .unwrap();

    InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("animation-file-data")),
        InputFileReader::from(Cursor::new("animation-thumb-data")),
        InputMediaAnimation::default(),
    )
    .unwrap();

    InputMedia::new(
        InputFile::file_id("audio-file-id"),
        InputMediaAudio::default().caption("test"),
    )
    .unwrap();

    InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("audio-file-data")),
        InputFileReader::from(Cursor::new("audio-thumb-data")),
        InputMediaAudio::default(),
    )
    .unwrap();

    InputMedia::new(
        InputFile::file_id("audio-file-id"),
        InputMediaDocument::default().caption("test"),
    )
    .unwrap();

    InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("document-file-data")),
        InputFileReader::from(Cursor::new("document-thumb-data")),
        InputMediaDocument::default(),
    )
    .unwrap();

    InputMedia::new(
        InputFile::file_id("photo-file-id"),
        InputMediaPhoto::default().caption("test"),
    )
    .unwrap();

    InputMedia::new(
        InputFile::file_id("video-file-id"),
        InputMediaVideo::default().caption("test"),
    )
    .unwrap();

    InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("video-file-data")),
        InputFileReader::from(Cursor::new("video-thumb-data")),
        InputMediaVideo::default(),
    )
    .unwrap();
}

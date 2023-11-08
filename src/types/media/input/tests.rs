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
        InputMediaAnimation::default().with_caption("test"),
    )
    .unwrap()
    .into();
    assert_eq!(
        Form::from([(
            "media",
            r#"{"type":"animation","media":"animation-file-id","caption":"test"}"#.into()
        )]),
        data
    );

    let data: Form = InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("animation-file-data")),
        InputMediaAnimation::default(),
        InputFileReader::from(Cursor::new("animation-thumb-data")),
    )
    .unwrap()
    .into();
    assert_eq!(
        Form::from([
            (
                "tgbot_im_thumb",
                InputFile::from(Cursor::new("animation-thumb-data")).into()
            ),
            (
                "tgbot_im_file",
                InputFile::from(Cursor::new("animation-file-data")).into()
            ),
            (
                "media",
                r#"{"type":"animation","media":"attach://tgbot_im_file","thumbnail":"attach://tgbot_im_thumb"}"#.into()
            )
        ]),
        data
    );
}

#[test]
fn create_input_media() {
    InputMedia::new(
        InputFile::file_id("animation-file-id"),
        InputMediaAnimation::default().with_caption("test"),
    )
    .unwrap();

    InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("animation-file-data")),
        InputMediaAnimation::default(),
        InputFileReader::from(Cursor::new("animation-thumb-data")),
    )
    .unwrap();

    InputMedia::new(
        InputFile::file_id("audio-file-id"),
        InputMediaAudio::default().with_caption("test"),
    )
    .unwrap();

    InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("audio-file-data")),
        InputMediaAudio::default(),
        InputFileReader::from(Cursor::new("audio-thumb-data")),
    )
    .unwrap();

    InputMedia::new(
        InputFile::file_id("audio-file-id"),
        InputMediaDocument::default().with_caption("test"),
    )
    .unwrap();

    InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("document-file-data")),
        InputMediaDocument::default(),
        InputFileReader::from(Cursor::new("document-thumb-data")),
    )
    .unwrap();

    InputMedia::new(
        InputFile::file_id("photo-file-id"),
        InputMediaPhoto::default().with_caption("test"),
    )
    .unwrap();

    InputMedia::new(
        InputFile::file_id("video-file-id"),
        InputMediaVideo::default().with_caption("test"),
    )
    .unwrap();

    InputMedia::with_thumbnail(
        InputFileReader::from(Cursor::new("video-file-data")),
        InputMediaVideo::default(),
        InputFileReader::from(Cursor::new("video-thumb-data")),
    )
    .unwrap();
}

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
        InputMediaError,
        InputMediaPhoto,
        InputMediaType,
        InputMediaVideo,
    },
};

#[test]
fn input_media_form() {
    let data: Form = InputMedia::new(InputMediaType::for_animation(
        InputFile::file_id("animation-file-id"),
        InputMediaAnimation::default().with_caption("test"),
    ))
    .unwrap()
    .into();
    assert_eq!(
        Form::from([(
            "media",
            r#"{"type":"animation","media":"animation-file-id","caption":"test"}"#.into()
        )]),
        data
    );

    let data: Form = InputMedia::new(
        InputMediaType::for_animation(
            InputFileReader::from(Cursor::new("animation-file-data")),
            InputMediaAnimation::default(),
        )
        .with_thumbnail(InputFileReader::from(Cursor::new("animation-thumb-data")))
        .unwrap(),
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
fn create_input_media_animation() {
    InputMedia::new(InputMediaType::for_animation(
        InputFile::file_id("animation-file-id"),
        InputMediaAnimation::default().with_caption("test"),
    ))
    .unwrap();

    InputMedia::new(
        InputMediaType::for_animation(
            InputFileReader::from(Cursor::new("animation-file-data")),
            InputMediaAnimation::default(),
        )
        .with_thumbnail(InputFileReader::from(Cursor::new("animation-thumb-data")))
        .unwrap(),
    )
    .unwrap();

    let err = InputMediaType::for_animation(
        InputFileReader::from(Cursor::new("animation-file-data")),
        InputMediaAnimation::default(),
    )
    .with_cover(InputFile::file_id("cover-id"))
    .unwrap_err();
    assert!(matches!(err, InputMediaError::CoverNotAcceptable));
}

#[test]
fn create_input_media_audio() {
    InputMedia::new(InputMediaType::for_audio(
        InputFile::file_id("audio-file-id"),
        InputMediaAudio::default().with_caption("test"),
    ))
    .unwrap();

    InputMedia::new(
        InputMediaType::for_audio(
            InputFileReader::from(Cursor::new("audio-file-data")),
            InputMediaAudio::default(),
        )
        .with_thumbnail(InputFileReader::from(Cursor::new("audio-thumb-data")))
        .unwrap(),
    )
    .unwrap();

    let err = InputMediaType::for_audio(
        InputFile::file_id("audio-file-id"),
        InputMediaAudio::default().with_caption("test"),
    )
    .with_cover(InputFile::file_id("cover-id"))
    .unwrap_err();
    assert!(matches!(err, InputMediaError::CoverNotAcceptable));
}

#[test]
fn create_input_media_document() {
    InputMedia::new(InputMediaType::for_document(
        InputFile::file_id("audio-file-id"),
        InputMediaDocument::default().with_caption("test"),
    ))
    .unwrap();

    InputMedia::new(
        InputMediaType::for_document(
            InputFileReader::from(Cursor::new("document-file-data")),
            InputMediaDocument::default(),
        )
        .with_thumbnail(InputFileReader::from(Cursor::new("document-thumb-data")))
        .unwrap(),
    )
    .unwrap();

    let err = InputMediaType::for_document(
        InputFile::file_id("audio-file-id"),
        InputMediaDocument::default().with_caption("test"),
    )
    .with_cover(InputFile::file_id("cover-id"))
    .unwrap_err();
    assert!(matches!(err, InputMediaError::CoverNotAcceptable));
}

#[test]
fn create_input_media_photo() {
    InputMedia::new(InputMediaType::for_photo(
        InputFile::file_id("photo-file-id"),
        InputMediaPhoto::default().with_caption("test"),
    ))
    .unwrap();
    let err = InputMediaType::for_photo(
        InputFile::file_id("photo-file-id"),
        InputMediaPhoto::default().with_caption("test"),
    )
    .with_thumbnail(InputFileReader::from(Cursor::new("photo-thumb-data")))
    .unwrap_err();
    assert!(matches!(err, InputMediaError::ThumbnailNotAcceptable));

    let err = InputMediaType::for_photo(
        InputFile::file_id("photo-file-id"),
        InputMediaPhoto::default().with_caption("test"),
    )
    .with_cover(InputFile::file_id("cover-id"))
    .unwrap_err();
    assert!(matches!(err, InputMediaError::CoverNotAcceptable));
}

#[test]
fn create_input_media_video() {
    InputMedia::new(InputMediaType::for_video(
        InputFile::file_id("video-file-id"),
        InputMediaVideo::default().with_caption("test"),
    ))
    .unwrap();

    InputMedia::new(
        InputMediaType::for_video(
            InputFileReader::from(Cursor::new("video-file-data")),
            InputMediaVideo::default(),
        )
        .with_thumbnail(InputFileReader::from(Cursor::new("video-thumb-data")))
        .unwrap()
        .with_cover(InputFile::file_id("cover-id"))
        .unwrap(),
    )
    .unwrap();
}

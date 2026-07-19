use std::io::Cursor;

use crate::{api::Form, types::*};

#[tokio::test]
async fn input_file() {
    let id = InputFile::file_id("file-id");
    assert_eq!(format!("{id:?}"), r#"Id("file-id")"#);

    let url = InputFile::url("http://example.com/archive.zip");
    assert_eq!(format!("{url:?}"), r#"Url("http://example.com/archive.zip")"#);

    // NOTE: you must be sure that file exists in current working directory (usually it exists)
    // otherwise test will fail
    let path = InputFile::path("LICENSE").await.unwrap();
    assert_eq!(
        format!("{path:?}"),
        r#"Reader(InputFileReader { file_name: Some("LICENSE"), mime_type: Some("application/octet-stream") })"#,
    );

    let reader = InputFileReader::from(Cursor::new(b"data"))
        .with_file_name("name")
        .with_mime_type(mime::TEXT_PLAIN);
    assert_eq!(reader.file_name().unwrap(), "name");
    assert_eq!(reader.mime_type().unwrap(), &mime::TEXT_PLAIN);
    let reader = InputFile::from(reader);
    assert_eq!(
        format!("{reader:?}"),
        r#"Reader(InputFileReader { file_name: Some("name"), mime_type: Some("text/plain") })"#,
    );

    let reader = InputFile::from(Cursor::new(b"data"));
    assert_eq!(
        format!("{reader:?}"),
        "Reader(InputFileReader { file_name: None, mime_type: None })",
    );
}

macro_rules! assert_input_media_eq {
    ($media:expr, $expected_form:expr) => {{
        let (form, data) = $media.into_parts(&[0]);
        insta::assert_json_snapshot!(data);
        assert_eq!(form, $expected_form);
    }};
}

fn assert_input_media_parse_mode(media: InputMedia) {
    let (form, data) = media.into_parts(&[0]);
    let actual = serde_json::to_value(&data).unwrap();
    assert_eq!(actual["parse_mode"], serde_json::json!("Markdown"));
    assert!(actual.get("caption_entities").is_none());
    assert_eq!(Form::default(), form);
}

fn assert_input_media_caption_entities(media: InputMedia) {
    let (form, data) = media.into_parts(&[0]);
    let actual = serde_json::to_value(data).unwrap();
    assert_eq!(
        actual["caption_entities"],
        serde_json::json!([{"offset": 0, "length": 10, "type": "bold"}])
    );
    assert!(actual.get("parse_mode").is_none());
    assert_eq!(Form::default(), form);
}

#[test]
fn input_media_animation() {
    let media =
        InputMedia::from(InputMediaAnimation::from(InputFile::file_id("animation-file-id")).with_caption("test"));
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(
        InputMediaAnimation::from(InputFileReader::from(Cursor::new("animation-file-data")))
            .with_thumbnail(InputFileReader::from(Cursor::new("animation-thumb-data")))
            .with_caption("caption")
            .with_duration(10)
            .with_has_spoiler(true)
            .with_height(200)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_show_caption_above_media(true)
            .with_width(200),
    );
    assert_input_media_eq!(
        media,
        Form::from([
            ("im_tmb", InputFile::from(Cursor::new("animation-thumb-data")).into()),
            ("im_media", InputFile::from(Cursor::new("animation-file-data")).into()),
        ])
    );

    let media = InputMedia::from(
        InputMediaAnimation::from(InputFile::url("test"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_input_media_parse_mode(media);

    let media = InputMedia::from(
        InputMediaAnimation::from(InputFile::url("test"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)]),
    );
    assert_input_media_caption_entities(media);
}

#[test]
fn input_media_audio() {
    let media = InputMedia::from(InputMediaAudio::from(InputFile::file_id("audio-file-id")).with_caption("test"));
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(
        InputMediaAudio::from(InputFileReader::from(Cursor::new("audio-file-data")))
            .with_thumbnail(InputFileReader::from(Cursor::new("audio-thumb-data")))
            .with_caption("caption")
            .with_duration(10)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_performer("test performer")
            .with_title("test title"),
    );
    assert_input_media_eq!(
        media,
        Form::from([
            ("im_media", InputFile::from(Cursor::new("audio-file-data")).into()),
            ("im_tmb", InputFile::from(Cursor::new("audio-thumb-data")).into()),
        ])
    );

    let media = InputMedia::from(
        InputMediaAudio::from(InputFile::file_id("parse-mode"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_input_media_parse_mode(media);

    let media = InputMedia::from(
        InputMediaAudio::from(InputFile::file_id("parse-mode"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)]),
    );
    assert_input_media_caption_entities(media);
}

#[test]
fn input_media_document() {
    let media = InputMedia::from(InputMediaDocument::from(InputFile::file_id("audio-file-id")).with_caption("test"));
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(
        InputMediaDocument::from(InputFileReader::from(Cursor::new("document-file-data")))
            .with_thumbnail(InputFileReader::from(Cursor::new("document-thumb-data")))
            .with_caption("caption")
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_disable_content_type_detection(true),
    );
    assert_input_media_eq!(
        media,
        Form::from([
            ("im_media", InputFile::from(Cursor::new("document-file-data")).into()),
            ("im_tmb", InputFile::from(Cursor::new("document-thumb-data")).into()),
        ])
    );

    let media = InputMedia::from(
        InputMediaDocument::from(InputFile::file_id("file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_input_media_parse_mode(media);

    let media = InputMedia::from(
        InputMediaDocument::from(InputFile::file_id("file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)]),
    );
    assert_input_media_caption_entities(media);
}

#[test]
fn input_media_link() {
    let media = InputMedia::link("https://example.com");
    assert_input_media_eq!(media, Form::default());
}

#[test]
fn input_media_live_photo() {
    let media = InputMedia::from(
        InputMediaLivePhoto::from((
            InputFile::url("https://example.com/video.mp4"),
            InputFile::url("https://example.com/photo.png"),
        ))
        .with_caption("test")
        .with_parse_mode(ParseMode::Markdown)
        .with_show_caption_above_media(true)
        .with_has_spoiler(false),
    );
    assert_input_media_eq!(media, Form::default());
}

#[test]
fn input_media_location() {
    let media = InputMedia::from(InputMediaLocation::new(1.0, 2.0));
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(InputMediaLocation::new(1.0, 2.0).with_horizontal_accuracy(3.0));
    assert_input_media_eq!(media, Form::default());
}

#[test]
fn input_media_photo() {
    let media = InputMedia::from(
        InputMediaPhoto::from(InputFile::file_id("photo-file-id"))
            .with_caption("test")
            .with_has_spoiler(true)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_show_caption_above_media(true),
    );
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(
        InputMediaPhoto::from(InputFile::file_id("photo-file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_input_media_parse_mode(media);

    let media = InputMedia::from(
        InputMediaPhoto::from(InputFile::file_id("photo-file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)]),
    );
    assert_input_media_caption_entities(media);
}

#[test]
fn input_media_sticker() {
    let media = InputMedia::from(InputMediaSticker::from(InputFile::file_id("sticker-file-id")));
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(InputMediaSticker::from(InputFile::file_id("sticker-file-id")).with_emoji("🤡"));
    assert_input_media_eq!(media, Form::default());
}

#[test]
fn input_media_venue() {
    let media = InputMedia::from(InputMediaVenue::new(1.0, 2.0, "test", "addr"));
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(
        InputMediaVenue::new(1.0, 2.0, "test", "addr")
            .with_foursquare_id("f-id")
            .with_foursquare_type("f-type")
            .with_google_place_id("g-id")
            .with_google_place_type("g-type"),
    );
    assert_input_media_eq!(media, Form::default());
}

#[test]
fn input_media_video() {
    let media = InputMedia::from(InputMediaVideo::from(InputFile::file_id("video-file-id")).with_caption("test"));
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(
        InputMediaVideo::from(InputFileReader::from(Cursor::new("video-file-data")))
            .with_thumbnail(InputFileReader::from(Cursor::new("video-thumb-data")))
            .with_cover(InputFile::file_id("cover-id"))
            .with_caption("caption")
            .with_duration(100)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_has_spoiler(true)
            .with_height(200)
            .with_show_caption_above_media(true)
            .with_start_timestamp(450)
            .with_supports_streaming(true)
            .with_width(200),
    );
    assert_input_media_eq!(
        media,
        Form::from([
            ("im_media", InputFile::from(Cursor::new("video-file-data")).into()),
            ("im_tmb", InputFile::from(Cursor::new("video-thumb-data")).into()),
        ])
    );

    let media = InputMedia::from(
        InputMediaVideo::from(InputFile::file_id("video-file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_input_media_parse_mode(media);

    let media = InputMedia::from(
        InputMediaVideo::from(InputFile::file_id("video-file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)]),
    );
    assert_input_media_caption_entities(media);
}

#[test]
fn input_media_voice_note() {
    let media = InputMedia::from(InputMediaVoiceNote::from(InputFile::url("test")));
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(
        InputMediaVoiceNote::from(InputFile::url("test"))
            .with_caption("test")
            .with_caption_entities([TextEntity::bold(0..2)])
            .with_duration(1),
    );
    assert_input_media_eq!(media, Form::default());

    let media = InputMedia::from(
        InputMediaVoiceNote::from(InputFile::file_id("voice-note-file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_input_media_parse_mode(media);

    let media = InputMedia::from(
        InputMediaVoiceNote::from(InputFile::file_id("voice-note-file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)]),
    );
    assert_input_media_caption_entities(media);
}

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
        InputPaidMediaGroupItem::for_live_photo(InputFile::file_id("lp-id"), InputFile::file_id("lps-id")),
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
    let mut media = String::from("[{\"type\":\"live_photo\",\"media\":\"lp-id\",\"photo\":\"lps-id\"}");
    media += ",{\"type\":\"photo\",\"media\":\"photo-file-id\"}";
    media += ",{\"type\":\"video\",\"media\":\"https://example.com/video-file-1.mp4\",\"thumbnail\":\"video-1-thumbnail-id\"}";
    media += ",{\"type\":\"video\",\"media\":\"attach://tgbot_ipm_file_3\",\"cover\":\"cover-url\",";
    media += "\"thumbnail\":\"attach://tgbot_ipm_thumb_3\",\"duration\":1,\"height\":2,";
    media += "\"start_timestamp\":20,\"supports_streaming\":true,\"width\":3}]";
    let expected_form = Form::from([
        ("media", media.into()),
        (
            "tgbot_ipm_thumb_3",
            InputFile::from(Cursor::new("video-2-thumbnail-data")).into(),
        ),
        (
            "tgbot_ipm_file_3",
            InputFile::from(Cursor::new("video-file-2-data")).into(),
        ),
    ]);
    assert_eq!(actual_form, expected_form);
}

#[test]
fn input_profile_photo() {
    let animated = InputProfilePhotoAnimated::new(InputFile::url("url"));
    let photo = InputProfilePhoto::from(animated);
    assert!(matches!(photo, InputProfilePhoto::Animated(_)));
    assert_eq!(
        Form::from([("photo", r#"{"type":"animated","animation":"url"}"#.into())]),
        Form::try_from(photo).unwrap()
    );

    let animated = InputProfilePhotoAnimated::new(Cursor::new("test"));
    let photo = InputProfilePhoto::from(animated);
    assert!(matches!(photo, InputProfilePhoto::Animated(_)));
    assert_eq!(
        Form::from([
            (
                "photo",
                r#"{"type":"animated","animation":"attach://tgbot_ipp_file"}"#.into()
            ),
            ("tgbot_ipp_file", InputFile::from(Cursor::new("test")).into())
        ]),
        Form::try_from(photo).unwrap()
    );

    let animated = InputProfilePhotoAnimated::new(InputFile::url("url")).with_main_frame_timestamp(1.0);
    let photo = InputProfilePhoto::from(animated);
    assert!(matches!(photo, InputProfilePhoto::Animated(_)));
    assert_eq!(
        Form::from([(
            "photo",
            r#"{"type":"animated","animation":"url","main_frame_timestamp":1.0}"#.into()
        )]),
        Form::try_from(photo).unwrap()
    );

    let photo = InputProfilePhoto::from(InputProfilePhotoStatic::new(InputFile::url("url")));
    assert!(matches!(photo, InputProfilePhoto::Static(_)));
    assert_eq!(
        Form::from([("photo", r#"{"type":"static","photo":"url"}"#.into())]),
        Form::try_from(photo).unwrap()
    );

    let photo = InputProfilePhoto::from(InputProfilePhotoStatic::new(Cursor::new("test")));
    assert!(matches!(photo, InputProfilePhoto::Static(_)));
    assert_eq!(
        Form::from([
            ("photo", r#"{"type":"static","photo":"attach://tgbot_ipp_file"}"#.into()),
            ("tgbot_ipp_file", InputFile::from(Cursor::new("test")).into())
        ]),
        Form::try_from(photo).unwrap()
    );
}

#[test]
fn input_story_content() {
    let content = InputStoryContent::from(InputStoryContentPhoto::new(InputFile::url("url")));
    assert!(matches!(content, InputStoryContent::Photo(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([("content", r#"{"type":"photo","photo":"url"}"#.into())]),
        form
    );

    let content = InputStoryContent::from(InputStoryContentPhoto::new(Cursor::new("test")));
    assert!(matches!(content, InputStoryContent::Photo(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([
            (
                "content",
                r#"{"type":"photo","photo":"attach://tgbot_isc_file"}"#.into()
            ),
            ("tgbot_isc_file", InputFile::from(Cursor::new("test")).into())
        ]),
        form
    );

    let content = InputStoryContent::from(InputStoryContentVideo::new(InputFile::url("url")));
    assert!(matches!(content, InputStoryContent::Video(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([("content", r#"{"type":"video","video":"url"}"#.into())]),
        form
    );

    let content = InputStoryContent::from(InputStoryContentVideo::new(Cursor::new("test")));
    assert!(matches!(content, InputStoryContent::Video(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([
            (
                "content",
                r#"{"type":"video","video":"attach://tgbot_isc_file"}"#.into()
            ),
            ("tgbot_isc_file", InputFile::from(Cursor::new("test")).into())
        ]),
        form
    );

    let content = InputStoryContent::from(
        InputStoryContentVideo::new(InputFile::url("url"))
            .with_cover_frame_timestamp(1.0)
            .with_duration(1.0)
            .with_is_animation(true),
    );
    assert!(matches!(content, InputStoryContent::Video(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([(
            "content",
            r#"{"type":"video","video":"url","cover_frame_timestamp":1.0,"duration":1.0,"is_animation":true}"#.into()
        )]),
        form
    );
}

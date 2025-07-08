use std::io::Cursor;

use serde::Serialize;

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

#[test]
fn input_media_animation() {
    insta::assert_json_snapshot!(
        InputMediaAnimation::default()
            .with_caption("caption")
            .with_duration(10)
            .with_has_spoiler(true)
            .with_height(200)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_show_caption_above_media(true)
            .with_width(200)
    );
    insta::assert_json_snapshot!(InputMediaAnimation::default());
}

#[test]
fn input_media_animation_entities_vs_parse_mode() {
    let mut data = InputMediaAnimation::default();
    data = data.with_caption_parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&data).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    data = data.with_caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(data).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

#[test]
fn input_media_audio() {
    insta::assert_json_snapshot!(
        InputMediaAudio::default()
            .with_caption("caption")
            .with_duration(10)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_performer("test performer")
            .with_title("test title")
    );
    insta::assert_json_snapshot!(InputMediaAudio::default());
}

#[test]
fn input_media_audio_entities_vs_parse_mode() {
    let mut data = InputMediaAudio::default();
    data = data.with_caption_parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&data).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    data = data.with_caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(data).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

#[test]
fn input_media_document() {
    insta::assert_json_snapshot!(
        InputMediaDocument::default()
            .with_caption("caption")
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_disable_content_type_detection(true)
    );
    insta::assert_json_snapshot!(InputMediaDocument::default());
}

#[test]
fn input_media_document_entities_vs_parse_mode() {
    let mut method = InputMediaDocument::default();
    method = method.with_caption_parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&method).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    method = method.with_caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(method).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

#[test]
fn input_media_photo() {
    insta::assert_json_snapshot!(
        InputMediaPhoto::default()
            .with_caption("caption")
            .with_has_spoiler(true)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_show_caption_above_media(true)
    );
    insta::assert_json_snapshot!(InputMediaPhoto::default());
}

#[test]
fn input_media_photo_entities_vs_parse_mode() {
    let mut method = InputMediaPhoto::default();
    method = method.with_caption_parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&method).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    method = method.with_caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(method).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

#[test]
fn input_media_video() {
    insta::assert_json_snapshot!(
        InputMediaVideo::default()
            .with_caption("caption")
            .with_duration(100)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_has_spoiler(true)
            .with_height(200)
            .with_show_caption_above_media(true)
            .with_start_timestamp(450)
            .with_supports_streaming(true)
            .with_width(200)
    );
    insta::assert_json_snapshot!(InputMediaVideo::default());
}

#[test]
fn input_media_video_entities_vs_parse_mode() {
    let mut method = InputMediaVideo::default();
    method = method.with_caption_parse_mode(ParseMode::Markdown);
    assert_eq!(
        serde_json::to_value(&method).unwrap(),
        serde_json::json!({"parse_mode": "Markdown"})
    );
    method = method.with_caption_entities(vec![TextEntity::bold(0..10)]);
    assert_eq!(
        serde_json::to_value(method).unwrap(),
        serde_json::json!({"caption_entities": [{"offset": 0, "length": 10, "type": "bold"}]})
    );
}

#[test]
fn input_message_content_contact() {
    let content = InputMessageContentContact::new("V", "+79001231212");
    insta::assert_json_snapshot!(InputMessageContent::from(
        content.clone().with_last_name("P").with_vcard("vcard")
    ));
    insta::assert_json_snapshot!(content);
}

#[derive(Serialize)]
struct InvoiceProviderData {
    key: String,
}

#[test]
fn input_message_content_invoice() {
    insta::assert_json_snapshot!(InputMessageContent::from(
        InputMessageContentInvoice::new(
            "RUB",
            "description",
            "payload",
            [LabeledPrice::new(100, "item")],
            "title",
        )
        .with_is_flexible(true)
        .with_need_email(false)
        .with_need_name(true)
        .with_need_phone_number(true)
        .with_need_shipping_address(false)
        .with_provider_data(&InvoiceProviderData {
            key: String::from("value"),
        })
        .unwrap()
        .with_provider_token("provider-token")
        .with_photo_height(24)
        .with_photo_size(100)
        .with_photo_width(24)
        .with_photo_url("https://google.com/favicon.ico")
        .with_max_tip_amount(1)
        .with_send_email_to_provider(false)
        .with_send_phone_number_to_provider(true)
        .with_suggested_tip_amounts([2]),
    ));
    insta::assert_json_snapshot!(InputMessageContent::from(InputMessageContentInvoice::new(
        "RUB",
        "description",
        "payload",
        [LabeledPrice::new(100, "item")],
        "title",
    )));
}

#[test]
fn input_message_content_location() {
    insta::assert_json_snapshot!(InputMessageContent::from(
        InputMessageContentLocation::new(1.0, 2.0)
            .with_heading(90)
            .with_horizontal_accuracy(1.5)
            .with_live_period(100)
            .with_proximity_alert_radius(100),
    ));
    insta::assert_json_snapshot!(InputMessageContent::from(InputMessageContentLocation::new(1.0, 2.0)));
}

#[test]
fn input_message_content_text() {
    insta::assert_json_snapshot!(InputMessageContent::from(
        InputMessageContentText::new("text")
            .with_link_preview_options(LinkPreviewOptions::default().with_is_disabled(true))
            .with_entities(vec![TextEntity::bold(0..10)])
            .with_parse_mode(ParseMode::Html),
    ));
    insta::assert_json_snapshot!(InputMessageContent::from(
        InputMessageContentText::new("text")
            .with_parse_mode(ParseMode::Markdown)
            .with_entities(vec![TextEntity::bold(0..10)]),
    ));
}

#[test]
fn input_message_content_venue() {
    let content = InputMessageContentVenue::new("addr", 1.0, 2.0, "title");
    insta::assert_json_snapshot!(InputMessageContent::from(
        content
            .clone()
            .with_foursquare_id("f-id")
            .with_foursquare_type("f-type")
            .with_google_place_id("g-id")
            .with_google_place_type("g-type"),
    ));
    insta::assert_json_snapshot!(InputMessageContent::from(content));
}

#[test]
fn convert_message_content() {
    let contact = Contact::new("User", "+79001234567");
    let content = InputMessageContent::from(contact.clone());
    assert_eq!(
        content,
        InputMessageContent::Contact(InputMessageContentContact::new(
            contact.first_name,
            contact.phone_number,
        ))
    );

    let location = Location::new(0.0, 0.0);
    let content = InputMessageContent::from(location);
    assert_eq!(
        content,
        InputMessageContent::Location(InputMessageContentLocation::new(location.latitude, location.longitude))
    );

    let content = InputMessageContent::from("text");
    assert_eq!(content, InputMessageContent::Text(InputMessageContentText::new("text")));

    let content = InputMessageContent::from(Text::from("text"));
    assert_eq!(content, InputMessageContent::Text(InputMessageContentText::new("text")));

    let venue = Venue::new("Venue", "Address", location);
    let content = InputMessageContent::from(venue.clone());
    assert_eq!(
        content,
        InputMessageContent::Venue(InputMessageContentVenue::new(
            venue.address,
            venue.location.latitude,
            venue.location.longitude,
            venue.title,
        ))
    );
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

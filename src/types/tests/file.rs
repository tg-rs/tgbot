use crate::types::*;

#[test]
fn file() {
    insta::assert_json_snapshot!(
        File::new("file-id", "file-unique-id")
            .with_file_path("file-path")
            .with_file_size(1024)
    );
    insta::assert_json_snapshot!(File::new("file-id", "file-unique-id"));
}

#[test]
fn get_file() {
    assert_payload_eq!(POST JSON "getFile" => GetFile::new("file-id"));
}

#[test]
fn animation() {
    insta::assert_json_snapshot!(
        Animation::new(243, "file-id", "file-unique-id", 200, 200)
            .with_file_name("File Name")
            .with_file_size(20480)
            .with_mime_type("image/gif")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024)),
    );
    insta::assert_json_snapshot!(Animation::new(30, "file-id", "file-unique-id", 200, 200));
}

#[test]
fn send_animation() {
    let method = SendAnimation::new(InputFile::file_id("file-id"), 1);
    assert_payload_eq!(POST FORM "sendAnimation" => method);
    let method = SendAnimation::new(InputFile::file_id("file-id"), 1)
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_caption("Caption")
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(100)
        .with_has_spoiler(true)
        .with_height(300)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_show_caption_above_media(true)
        .with_thumbnail(InputFile::url("https://google.com/favicon.ico"))
        .unwrap()
        .with_width(200);
    assert_payload_eq!(POST FORM "sendAnimation" => method);
}

#[test]
fn send_animation_with_thumbnail() {
    let err = SendAnimation::new(InputFile::file_id("file-id"), 1)
        .with_thumbnail(InputFile::file_id("file-id"))
        .unwrap_err();
    assert!(matches!(err, SendAnimationError::InvalidThumbnail));
}

#[test]
fn send_animation_entities_vs_parse_mode() {
    let method = SendAnimation::new(InputFile::file_id("file-id"), 1)
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap();
    assert_payload_eq!(POST FORM "sendAnimation" => method);
    let method = SendAnimation::new(InputFile::file_id("file-id"), 1)
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap()
        .with_caption_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST FORM "sendAnimation" => method);
}

#[test]
fn audio() {
    insta::assert_json_snapshot!(
        Audio::new(243, "file-id", "file-unique-id")
            .with_file_name("File Name")
            .with_file_size(10240)
            .with_mime_type("audio/mpeg")
            .with_performer("Performer")
            .with_title("Title")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-unique-file-id", 24, 24).with_file_size(1024)),
    );
    insta::assert_json_snapshot!(Audio::new(243, "file-id", "file-unique-id"));
}

#[test]
fn send_audio() {
    let method = SendAudio::new(1, InputFile::file_id("file-id"));
    assert_payload_eq!(POST FORM "sendAudio" => method);
    let method = SendAudio::new(1, InputFile::file_id("file-id"))
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_caption("Caption")
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(100)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_performer("Performer")
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_thumbnail(InputFile::url("https://google.com/favicon.ico"))
        .unwrap()
        .with_title("Title");
    assert_payload_eq!(POST FORM "sendAudio" => method);
}

#[test]
fn send_audio_with_thumbnail() {
    let err = SendAudio::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("thumb-id"))
        .unwrap_err();
    assert!(matches!(err, SendAudioError::InvalidThumbnail));
}

#[test]
fn send_audio_entities_vs_parse_mode() {
    let method = SendAudio::new(1, InputFile::file_id("file-id"))
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap()
        .with_caption_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST FORM "sendAudio" => method);
    let method = SendAudio::new(1, InputFile::file_id("file-id"))
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap();
    assert_payload_eq!(POST FORM "sendAudio" => method);
}

#[test]
fn document() {
    insta::assert_json_snapshot!(
        Document::new("file-id", "file-unique-id")
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_name("File Name")
            .with_mime_type("image/jpeg")
            .with_file_size(10240),
    );
    insta::assert_json_snapshot!(Document::new("file-id", "file-unique-id"));
}

#[test]
fn send_document() {
    let method = SendDocument::new(1, InputFile::file_id("file-id"));
    assert_payload_eq!(POST FORM "sendDocument" => method);
    let method = SendDocument::new(1, InputFile::file_id("file-id"))
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_caption("Caption")
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_disable_content_type_detection(true)
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_thumbnail(InputFile::url("https://example.com/image.jpg"))
        .unwrap();
    assert_payload_eq!(POST FORM "sendDocument" => method);
}

#[test]
fn send_document_with_thumbnail() {
    let err = SendDocument::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("file-id"))
        .unwrap_err();
    assert!(matches!(err, SendDocumentError::InvalidThumbnail));
}

#[test]
fn send_document_entities_vs_parse_mode() {
    let method = SendDocument::new(1, InputFile::file_id("file-id"))
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap()
        .with_caption_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST FORM "sendDocument" => method);
    let method = SendDocument::new(1, InputFile::file_id("file-id"))
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap();
    assert_payload_eq!(POST FORM "sendDocument" => method);
}

#[test]
fn photo_size() {
    insta::assert_json_snapshot!(PhotoSize::new("file-id", "file-unique-id", 200, 200).with_file_size(1024));
    insta::assert_json_snapshot!(PhotoSize::new("file-id", "file-unique-id", 200, 200));
}

#[test]
fn send_photo() {
    let method = SendPhoto::new(1, InputFile::file_id("file-id"));
    assert_payload_eq!(POST FORM "sendPhoto" => method);
    let method = SendPhoto::new(1, InputFile::file_id("file-id"))
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_caption("Caption")
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_has_spoiler(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_show_caption_above_media(true);
    assert_payload_eq!(POST FORM "sendPhoto" => method);
}

#[test]
fn send_photo_entities_vs_parse_mode() {
    let method = SendPhoto::new(1, InputFile::file_id("file-id"))
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap()
        .with_caption_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST FORM "sendPhoto" => method);
    let method = SendPhoto::new(1, InputFile::file_id("file-id"))
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap();
    assert_payload_eq!(POST FORM "sendPhoto" => method);
}

#[test]
fn video() {
    insta::assert_json_snapshot!(
        Video::new(3, "file-id", "file-unique-id", 2, 1)
            .with_cover([PhotoSize::new("cover-file-id", "cover-file-unique-id", 24, 24)])
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_name("File Name")
            .with_mime_type("video/mpeg")
            .with_file_size(10240)
            .with_start_timestamp(20),
    );
    insta::assert_json_snapshot!(Video::new(3, "file-id", "file-unique-id", 2, 1));
}

#[test]
fn send_video() {
    let method = SendVideo::new(1, InputFile::file_id("file-id"));
    assert_payload_eq!(POST FORM "sendVideo" => method);
    let method = SendVideo::new(1, InputFile::file_id("file-id"))
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_caption("Caption")
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_cover(InputFile::file_id("cover-id"))
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(100)
        .with_has_spoiler(true)
        .with_height(300)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_show_caption_above_media(true)
        .with_start_timestamp(20)
        .with_supports_streaming(true)
        .with_thumbnail(InputFile::url("https://example.com/image.jpg"))
        .unwrap()
        .with_width(200);
    assert_payload_eq!(POST FORM "sendVideo" => method);
}

#[test]
fn send_video_with_thumbnail() {
    let err = SendVideo::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("file-id"))
        .unwrap_err();
    assert!(matches!(err, SendVideoError::InvalidThumbnail));
}

#[test]
fn send_video_entities_vs_parse_mode() {
    let method = SendVideo::new(1, InputFile::file_id("file-id"))
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap()
        .with_caption_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST FORM "sendVideo" => method);
    let method = SendVideo::new(1, InputFile::file_id("file-id"))
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap();
    assert_payload_eq!(POST FORM "sendVideo" => method);
}

#[test]
fn video_note() {
    insta::assert_json_snapshot!(
        VideoNote::new(1234, "file-id", "file-unique-id", 124)
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_size(10240)
    );
    insta::assert_json_snapshot!(VideoNote::new(1234, "file-id", "file-unique-id", 124));
}

#[test]
fn send_video_note() {
    let method = SendVideoNote::new(1, InputFile::file_id("file-id"));
    assert_payload_eq!(POST FORM "sendVideoNote" => method);
    let method = SendVideoNote::new(1, InputFile::file_id("file-id"))
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(50)
        .with_length(100)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap()
        .with_thumbnail(InputFile::url("https://example.com/image.jpg"))
        .unwrap();
    assert_payload_eq!(POST FORM "sendVideoNote" => method);
}

#[test]
fn send_video_note_with_thumbnail() {
    let err = SendVideoNote::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("thumb-file-id"))
        .unwrap_err();
    assert!(matches!(err, SendVideoNoteError::InvalidThumbnail));
}

#[test]
fn voice() {
    insta::assert_json_snapshot!(
        Voice::new(500, "file-id", "file-unique-id")
            .with_mime_type("audio/ogg")
            .with_file_size(40960)
    );
    insta::assert_json_snapshot!(Voice::new(500, "file-id", "file-unique-id"));
}

#[test]
fn send_voice() {
    let method = SendVoice::new(1, InputFile::file_id("file-id"));
    assert_payload_eq!(POST FORM "sendVoice" => method);
    let method = SendVoice::new(1, InputFile::file_id("file-id"))
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_caption("Caption")
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(100)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_markup(ForceReply::new(true))
        .unwrap()
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap();
    assert_payload_eq!(POST FORM "sendVoice" => method);
}

#[test]
fn send_voice_entities_vs_parse_mode() {
    let method = SendVoice::new(1, InputFile::file_id("file-id"))
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap()
        .with_caption_parse_mode(ParseMode::Markdown);
    assert_payload_eq!(POST FORM "sendVoice" => method);
    let method = SendVoice::new(1, InputFile::file_id("file-id"))
        .with_caption_parse_mode(ParseMode::Markdown)
        .with_caption_entities(vec![TextEntity::bold(0..10)])
        .unwrap();
    assert_payload_eq!(POST FORM "sendVoice" => method);
}

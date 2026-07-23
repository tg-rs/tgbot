use std::io::Cursor;

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
        .with_caption(("Caption", ParseMode::Markdown))
        .with_callback_query_id("cqid")
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(100)
        .with_has_spoiler(true)
        .with_height(300)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_receiver_user_id(999)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_show_caption_above_media(true)
        .with_suggested_post_parameters(SuggestedPostParameters::default())
        .with_thumbnail_file(Cursor::new(b"file-data"))
        .with_thumbnail_url("https://google.com/favicon.ico")
        .with_width(200);
    assert_payload_eq!(POST FORM "sendAnimation" => method);

    let method = SendAnimation::new(InputFile::file_id("file-id"), 1)
        .with_thumbnail_url("https://google.com/favicon.ico")
        .with_thumbnail_file(Cursor::new(b"file-data"));
    assert_payload_eq!(POST FORM "sendAnimation" => method);

    let method = SendAnimation::new(InputFile::file_id("file-id"), 1).with_caption(
        InputText::from("caption")
            .with_format(ParseMode::Markdown)
            .with_format([TextEntity::bold(0..10)]),
    );
    assert_payload_eq!(POST FORM "sendAnimation" => method);

    let method = SendAnimation::new(InputFile::file_id("file-id"), 1).with_caption(
        InputText::from("caption")
            .with_format([TextEntity::bold(0..10)])
            .with_format(ParseMode::Markdown),
    );
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
        .with_callback_query_id("cqid")
        .with_caption(("Caption", ParseMode::Markdown))
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(100)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_performer("Performer")
        .with_protect_content(true)
        .with_receiver_user_id(999)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_suggested_post_parameters(SuggestedPostParameters::default())
        .with_thumbnail_url("https://google.com/favicon.ico")
        .with_thumbnail_file(Cursor::new(b"thumbnail-file-data"))
        .with_title("Title");
    assert_payload_eq!(POST FORM "sendAudio" => method);

    let method = SendAudio::new(1, InputFile::file_id("file-id"))
        .with_thumbnail_file(Cursor::new(b"thumbnail-file-data"))
        .with_thumbnail_url("https://google.com/favicon.ico");
    assert_payload_eq!(POST FORM "sendAudio" => method);

    let method = SendAudio::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format(ParseMode::Markdown)
            .with_format([TextEntity::bold(0..10)]),
    );
    assert_payload_eq!(POST FORM "sendAudio" => method);

    let method = SendAudio::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format([TextEntity::bold(0..10)])
            .with_format(ParseMode::Markdown),
    );
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
        .with_callback_query_id("cqid")
        .with_caption(("Caption", ParseMode::Markdown))
        .with_disable_content_type_detection(true)
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_receiver_user_id(999)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_suggested_post_parameters(SuggestedPostParameters::default())
        .with_thumbnail_file(Cursor::new(b"file-data"))
        .with_thumbnail_url("https://example.com/image.jpg");
    assert_payload_eq!(POST FORM "sendDocument" => method);

    let method = SendDocument::new(1, InputFile::file_id("file-id"))
        .with_thumbnail_url("https://example.com/image.jpg")
        .with_thumbnail_file(Cursor::new(b"file-data"));
    assert_payload_eq!(POST FORM "sendDocument" => method);

    let method = SendDocument::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format(ParseMode::Markdown)
            .with_format([TextEntity::bold(0..10)]),
    );
    assert_payload_eq!(POST FORM "sendDocument" => method);

    let method = SendDocument::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format([TextEntity::bold(0..10)])
            .with_format(ParseMode::Markdown),
    );
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
        .with_callback_query_id("cqid")
        .with_caption(("Caption", ParseMode::Markdown))
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_has_spoiler(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_receiver_user_id(999)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_show_caption_above_media(true)
        .with_suggested_post_parameters(SuggestedPostParameters::default());
    assert_payload_eq!(POST FORM "sendPhoto" => method);

    let method = SendPhoto::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format(ParseMode::Markdown)
            .with_format([TextEntity::bold(0..10)]),
    );
    assert_payload_eq!(POST FORM "sendPhoto" => method);

    let method = SendPhoto::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format([TextEntity::bold(0..10)])
            .with_format(ParseMode::Markdown),
    );
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
            .with_qualities([VideoQuality {
                file_id: String::from("test"),
                file_unique_id: String::from("test-uniq"),
                codec: String::from("av"),
                height: 200,
                width: 200,
                file_size: None,
            }])
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
        .with_callback_query_id("cqid")
        .with_caption(("Caption", ParseMode::Markdown))
        .with_cover(InputFile::file_id("cover-id"))
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(100)
        .with_has_spoiler(true)
        .with_height(300)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_receiver_user_id(999)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_show_caption_above_media(true)
        .with_start_timestamp(20)
        .with_suggested_post_parameters(SuggestedPostParameters::default())
        .with_supports_streaming(true)
        .with_thumbnail_file(Cursor::new(b"file-data"))
        .with_thumbnail_url("https://example.com/image.jpg")
        .with_width(200);
    assert_payload_eq!(POST FORM "sendVideo" => method);

    let method = SendVideo::new(1, InputFile::file_id("file-id"))
        .with_thumbnail_url("https://example.com/image.jpg")
        .with_thumbnail_file(Cursor::new(b"file-data"));
    assert_payload_eq!(POST FORM "sendVideo" => method);

    let method = SendVideo::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format(ParseMode::Markdown)
            .with_format([TextEntity::bold(0..10)]),
    );
    assert_payload_eq!(POST FORM "sendVideo" => method);

    let method = SendVideo::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format([TextEntity::bold(0..10)])
            .with_format(ParseMode::Markdown),
    );
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
        .with_callback_query_id("cqid")
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(50)
        .with_length(100)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_receiver_user_id(999)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_suggested_post_parameters(SuggestedPostParameters::default())
        .with_thumbnail_file(Cursor::new(b"file-data"))
        .with_thumbnail_url("https://example.com/image.jpg");
    assert_payload_eq!(POST FORM "sendVideoNote" => method);

    let method = SendVideoNote::new(1, InputFile::file_id("file-id"))
        .with_thumbnail_url("https://example.com/image.jpg")
        .with_thumbnail_file(Cursor::new(b"file-data"));
    assert_payload_eq!(POST FORM "sendVideoNote" => method);
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
        .with_callback_query_id("cqid")
        .with_caption(("Caption", ParseMode::Markdown))
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_duration(100)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_receiver_user_id(999)
        .with_reply_markup(ForceReply::new(true))
        .with_reply_parameters(ReplyParameters::new(1))
        .with_suggested_post_parameters(SuggestedPostParameters::default());
    assert_payload_eq!(POST FORM "sendVoice" => method);

    let method = SendVoice::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format(ParseMode::Markdown)
            .with_format([TextEntity::bold(0..10)]),
    );
    assert_payload_eq!(POST FORM "sendVoice" => method);

    let method = SendVoice::new(1, InputFile::file_id("file-id")).with_caption(
        InputText::from("caption")
            .with_format([TextEntity::bold(0..10)])
            .with_format(ParseMode::Markdown),
    );
    assert_payload_eq!(POST FORM "sendVoice" => method);
}

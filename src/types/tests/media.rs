use std::io::Cursor;

use crate::types::*;

#[test]
fn send_media_group() {
    let media_group = MediaGroup::new(vec![
        MediaGroupItem::from(InputMediaAudio::from(InputFileReader::from(Cursor::new("test")))),
        MediaGroupItem::from(InputMediaDocument::from(InputFileReader::from(Cursor::new("test")))),
        MediaGroupItem::from(InputMediaLivePhoto::from((
            InputFile::file_id("file-id"),
            InputFile::file_id("photo-id"),
        ))),
        MediaGroupItem::from(InputMediaPhoto::from(InputFileReader::from(Cursor::new("test")))),
        MediaGroupItem::from(InputMediaPhoto::from(Cursor::new("test")).with_caption("caption")),
        MediaGroupItem::from(
            InputMediaVideo::from(InputFileReader::from(Cursor::new("test"))).with_cover(InputFile::url("cover-url")),
        ),
        MediaGroupItem::from(
            InputMediaAudio::from(InputFile::file_id("file-id")).with_thumbnail(InputFile::url("thumb-url")),
        ),
        MediaGroupItem::from(
            InputMediaDocument::from(InputFile::file_id("file-id")).with_thumbnail(InputFile::url("thumb-url")),
        ),
        MediaGroupItem::from(
            InputMediaVideo::from(InputFile::file_id("file-id")).with_thumbnail(InputFile::url("thumb-url")),
        ),
    ])
    .unwrap();
    let method = SendMediaGroup::new(1, media_group)
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("id")
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_message_effect_id("effect-id")
        .with_message_thread_id(1)
        .with_protect_content(true)
        .with_reply_parameters(ReplyParameters::new(1))
        .unwrap();
    assert_payload_eq!(POST FORM "sendMediaGroup" => method);
}

#[test]
fn paid_media_purchased() {
    insta::assert_json_snapshot!(PaidMediaPurchased::new(User::new(1, "John", false), "payload"));
}

#[test]
fn paid_media_info() {
    insta::assert_json_snapshot!(PaidMediaInfo::new(
        1,
        [
            PaidMedia::Photo(vec![PhotoSize::new("file-id", "file-unique-id", 200, 300)]),
            PaidMedia::Preview(
                PaidMediaPreview::default()
                    .with_duration(100)
                    .with_height(200)
                    .with_width(300),
            ),
        ],
    ));
}

#[test]
fn paid_media_live_photo() {
    insta::assert_json_snapshot!(PaidMedia::LivePhoto(LivePhoto::new(1, "fid", "fuid", 200, 300)));
}

#[test]
fn paid_media_photo() {
    insta::assert_json_snapshot!(PaidMedia::Photo(vec![PhotoSize::new(
        "file-id",
        "file-unique-id",
        200,
        300
    )]));
}

#[test]
fn paid_media_preview() {
    insta::assert_json_snapshot!(PaidMedia::Preview(PaidMediaPreview::default()));
    insta::assert_json_snapshot!(PaidMedia::Preview(
        PaidMediaPreview::default()
            .with_duration(100)
            .with_height(200)
            .with_width(300),
    ));
}

#[test]
fn paid_media_video() {
    insta::assert_json_snapshot!(PaidMedia::Video(Box::new(Video::new(
        100,
        "file-id",
        "file-unique-id",
        200,
        300
    ))));
}

#[test]
fn send_paid_media() {
    let media = InputPaidMediaGroup::new([InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id"))]).unwrap();
    let method = SendPaidMedia::new(1, media, 100);
    assert_payload_eq!(POST FORM "sendPaidMedia" => method);

    let media = InputPaidMediaGroup::new([InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id"))]).unwrap();
    let caption_entities = vec![TextEntity::bold(0..1)];
    let reply_parameters = ReplyParameters::new(1);
    let reply_markup = ForceReply::new(true);
    let method = SendPaidMedia::new(1, media, 100)
        .with_allow_paid_broadcast(true)
        .with_business_connection_id("c-id")
        .with_caption("caption")
        .with_caption_entities(caption_entities.clone())
        .unwrap()
        .with_direct_messages_topic_id(1)
        .with_disable_notification(true)
        .with_message_thread_id(1)
        .with_payload("payload")
        .with_protect_content(true)
        .with_reply_parameters(reply_parameters.clone())
        .unwrap()
        .with_reply_markup(reply_markup.clone())
        .unwrap()
        .with_suggested_post_parameters(&SuggestedPostParameters::default())
        .unwrap()
        .with_show_caption_above_media(true);
    assert_payload_eq!(POST FORM "sendPaidMedia" => method);
}

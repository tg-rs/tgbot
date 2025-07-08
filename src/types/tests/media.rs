use std::io::Cursor;

use crate::{
    api::{Form, Payload, assert_payload_eq},
    types::*,
};

fn create_media_group() -> MediaGroup {
    MediaGroup::new(vec![
        MediaGroupItem::for_photo(InputFileReader::from(Cursor::new("test")), InputMediaPhoto::default()),
        MediaGroupItem::for_video(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default()),
        MediaGroupItem::for_video(InputFile::file_id("file-id"), InputMediaVideo::default())
            .with_thumbnail(InputFile::url("thumb-url")),
    ])
    .unwrap()
}

#[test]
fn send_media_group() {
    let reply_parameters = ReplyParameters::new(1);
    let mut form: Form = create_media_group().into();
    form.insert_field("chat_id", 1);
    form.insert_field("allow_paid_broadcast", true);
    form.insert_field("business_connection_id", "id");
    form.insert_field("disable_notification", true);
    form.insert_field("protect_content", true);
    form.insert_field("message_effect_id", "effect-id");
    form.insert_field("message_thread_id", 1);
    form.insert_field("reply_parameters", reply_parameters.serialize().unwrap());
    assert_payload_eq(
        Payload::form("sendMediaGroup", form),
        SendMediaGroup::new(1, create_media_group())
            .with_allow_paid_broadcast(true)
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_message_effect_id("effect-id")
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_parameters(reply_parameters)
            .unwrap(),
    );
}

#[test]
fn media_group_new() {
    MediaGroup::new(vec![
        MediaGroupItem::for_audio(InputFileReader::from(Cursor::new("test")), InputMediaAudio::default()),
        MediaGroupItem::for_document(
            InputFileReader::from(Cursor::new("test")),
            InputMediaDocument::default(),
        ),
        MediaGroupItem::for_photo(
            InputFileReader::from(Cursor::new("test")),
            InputMediaPhoto::default().with_caption("caption"),
        ),
        MediaGroupItem::for_video(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default())
            .with_cover(InputFile::url("cover-url")),
        MediaGroupItem::for_audio(InputFile::file_id("file-id"), InputMediaAudio::default())
            .with_thumbnail(InputFile::url("thumb-url")),
        MediaGroupItem::for_document(InputFile::file_id("file-id"), InputMediaDocument::default())
            .with_thumbnail(InputFile::url("thumb-url")),
        MediaGroupItem::for_video(InputFile::file_id("file-id"), InputMediaVideo::default())
            .with_thumbnail(InputFile::url("thumb-url")),
    ])
    .unwrap();
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
    insta::assert_json_snapshot!(PaidMedia::Video(Video::new(100, "file-id", "file-unique-id", 200, 300)));
}

#[test]
fn send_paid_media() {
    let media = InputPaidMediaGroup::new([InputPaidMediaGroupItem::for_photo(InputFile::file_id("file-id"))]).unwrap();
    let method = SendPaidMedia::new(1, media, 100);
    let form = Form::from([
        ("chat_id", 1.into()),
        ("media", "[{\"type\":\"photo\",\"media\":\"file-id\"}]".into()),
        ("star_count", 100.into()),
    ]);
    assert_payload_eq(Payload::form("sendPaidMedia", form), method);

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
        .with_disable_notification(true)
        .with_payload("payload")
        .with_protect_content(true)
        .with_reply_parameters(reply_parameters.clone())
        .unwrap()
        .with_reply_markup(reply_markup.clone())
        .unwrap()
        .with_show_caption_above_media(true);
    let form = Form::from([
        ("chat_id", 1.into()),
        ("media", "[{\"type\":\"photo\",\"media\":\"file-id\"}]".into()),
        ("star_count", 100.into()),
        ("allow_paid_broadcast", true.into()),
        ("business_connection_id", "c-id".into()),
        ("caption", "caption".into()),
        (
            "caption_entities",
            serde_json::to_string(&TextEntities::from_iter(caption_entities))
                .unwrap()
                .into(),
        ),
        ("disable_notification", true.into()),
        ("payload", "payload".into()),
        ("protect_content", true.into()),
        (
            "reply_parameters",
            serde_json::to_string(&reply_parameters).unwrap().into(),
        ),
        (
            "reply_markup",
            serde_json::to_string(&ReplyMarkup::from(reply_markup)).unwrap().into(),
        ),
        ("show_caption_above_media", true.into()),
    ]);
    assert_payload_eq(Payload::form("sendPaidMedia", form), method);
}

use std::io::Cursor;

use crate::{
    api::{assert_payload_eq, Form, Payload},
    types::{
        InputFile,
        InputFileReader,
        InputMediaAudio,
        InputMediaDocument,
        InputMediaPhoto,
        InputMediaVideo,
        MediaGroup,
        MediaGroupItem,
        ReplyParameters,
        SendMediaGroup,
    },
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
    form.insert_field("business_connection_id", "id");
    form.insert_field("disable_notification", true);
    form.insert_field("protect_content", true);
    form.insert_field("message_effect_id", "effect-id");
    form.insert_field("message_thread_id", 1);
    form.insert_field("reply_parameters", reply_parameters.serialize().unwrap());
    assert_payload_eq(
        Payload::form("sendMediaGroup", form),
        SendMediaGroup::new(1, create_media_group())
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
        MediaGroupItem::for_video(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default()),
        MediaGroupItem::for_audio(InputFile::file_id("file-id"), InputMediaAudio::default())
            .with_thumbnail(InputFile::url("thumb-url")),
        MediaGroupItem::for_document(InputFile::file_id("file-id"), InputMediaDocument::default())
            .with_thumbnail(InputFile::url("thumb-url")),
        MediaGroupItem::for_video(InputFile::file_id("file-id"), InputMediaVideo::default())
            .with_thumbnail(InputFile::url("thumb-url")),
    ])
    .unwrap();
}

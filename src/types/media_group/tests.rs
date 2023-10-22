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
        SendMediaGroup,
    },
};

fn create_media_group() -> MediaGroup {
    MediaGroup::new(vec![
        MediaGroupItem::photo(InputFileReader::from(Cursor::new("test")), InputMediaPhoto::default()),
        MediaGroupItem::video(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default()),
        MediaGroupItem::video(InputFile::file_id("file-id"), InputMediaVideo::default())
            .with_thumbnail(InputFile::url("thumb-url")),
    ])
    .unwrap()
}

#[test]
fn send_media_group() {
    let mut form: Form = create_media_group().into();
    form.insert_field("chat_id", 1);
    form.insert_field("disable_notification", true);
    form.insert_field("protect_content", true);
    form.insert_field("reply_to_message_id", 1);
    form.insert_field("allow_sending_without_reply", true);
    form.insert_field("message_thread_id", 1);
    assert_payload_eq(
        Payload::form("sendMediaGroup", form),
        SendMediaGroup::new(1, create_media_group())
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .message_thread_id(1),
    );
}

#[test]
fn media_group_new() {
    MediaGroup::new(vec![
        MediaGroupItem::audio(InputFileReader::from(Cursor::new("test")), InputMediaAudio::default()),
        MediaGroupItem::document(
            InputFileReader::from(Cursor::new("test")),
            InputMediaDocument::default(),
        ),
        MediaGroupItem::photo(
            InputFileReader::from(Cursor::new("test")),
            InputMediaPhoto::default().caption("caption"),
        ),
        MediaGroupItem::video(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default()),
        MediaGroupItem::audio(InputFile::file_id("file-id"), InputMediaAudio::default())
            .with_thumbnail(InputFile::url("thumb-url")),
        MediaGroupItem::document(InputFile::file_id("file-id"), InputMediaDocument::default())
            .with_thumbnail(InputFile::url("thumb-url")),
        MediaGroupItem::video(InputFile::file_id("file-id"), InputMediaVideo::default())
            .with_thumbnail(InputFile::url("thumb-url")),
    ])
    .unwrap();
}

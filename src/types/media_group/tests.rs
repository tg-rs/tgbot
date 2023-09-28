use std::io::Cursor;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
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

#[test]
fn send_media_group() {
    let request = SendMediaGroup::new(
        1,
        MediaGroup::new(vec![
            MediaGroupItem::photo(InputFileReader::from(Cursor::new("test")), InputMediaPhoto::default()),
            MediaGroupItem::video(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default()),
            MediaGroupItem::video(InputFile::file_id("file-id"), InputMediaVideo::default())
                .with_thumb(InputFile::url("thumb-url")),
        ])
        .unwrap(),
    )
    .disable_notification(true)
    .protect_content(true)
    .reply_to_message_id(1)
    .allow_sending_without_reply(true)
    .into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(
        request.build_url("base-url", "token"),
        "base-url/bottoken/sendMediaGroup"
    );
    if let RequestBody::Form(form) = request.into_body() {
        assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
        assert!(form.fields.get("media").is_some());
        assert!(form.fields.get("tgbot_im_file_0").is_some());
        assert!(form.fields.get("tgbot_im_file_1").is_some());
        assert_eq!(form.fields["disable_notification"].get_text().unwrap(), "true");
        assert_eq!(form.fields["protect_content"].get_text().unwrap(), "true");
        assert_eq!(form.fields["reply_to_message_id"].get_text().unwrap(), "1");
        assert_eq!(form.fields["allow_sending_without_reply"].get_text().unwrap(), "true");
    } else {
        panic!("Unexpected request body");
    }
}

#[test]
fn create_media_group() {
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
            .with_thumb(InputFile::url("thumb-url")),
        MediaGroupItem::document(InputFile::file_id("file-id"), InputMediaDocument::default())
            .with_thumb(InputFile::url("thumb-url")),
        MediaGroupItem::video(InputFile::file_id("file-id"), InputMediaVideo::default())
            .with_thumb(InputFile::url("thumb-url")),
    ])
    .unwrap();
}

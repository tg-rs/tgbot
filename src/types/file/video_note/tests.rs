use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{
        tests::assert_json_eq,
        ForceReply,
        InputFile,
        PhotoSize,
        ReplyParameters,
        SendVideoNote,
        SendVideoNoteError,
        VideoNote,
    },
};

#[test]
fn video_note() {
    assert_json_eq(
        VideoNote::new(1234, "file-id", "file-unique-id", 124)
            .with_thumbnail(PhotoSize::new("thumb-file-id", "thumb-file-unique-id", 24, 24).with_file_size(1024))
            .with_file_size(10240),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "length": 124,
            "duration": 1234,
            "thumbnail": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-file-unique-id",
                "width": 24,
                "height": 24,
                "file_size": 1024
            },
            "file_size": 10240
        }),
    );
    assert_json_eq(
        VideoNote::new(1234, "file-id", "file-unique-id", 124),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "length": 124,
            "duration": 1234,
        }),
    );
}

#[test]
fn send_video_note() {
    assert_payload_eq(
        Payload::form(
            "sendVideoNote",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video_note", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendVideoNote::new(1, InputFile::file_id("file-id")),
    );
    let reply_parameters = ReplyParameters::new(1);
    assert_payload_eq(
        Payload::form(
            "sendVideoNote",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video_note", InputFile::file_id("file-id").into()),
                ("duration", 50.into()),
                ("length", 100.into()),
                ("thumbnail", InputFile::url("https://example.com/image.jpg").into()),
                ("business_connection_id", "id".into()),
                ("disable_notification", true.into()),
                ("message_thread_id", 1.into()),
                ("protect_content", true.into()),
                (
                    "reply_markup",
                    serde_json::to_string(&ForceReply::new(true)).unwrap().into(),
                ),
                ("reply_parameters", reply_parameters.serialize().unwrap().into()),
            ]),
        ),
        SendVideoNote::new(1, InputFile::file_id("file-id"))
            .with_business_connection_id("id")
            .with_disable_notification(true)
            .with_duration(50)
            .with_length(100)
            .with_message_thread_id(1)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_parameters(reply_parameters)
            .unwrap()
            .with_thumbnail(InputFile::url("https://example.com/image.jpg"))
            .unwrap(),
    );
}

#[test]
fn send_video_note_with_thumbnail() {
    let err = SendVideoNote::new(1, InputFile::file_id("file-id"))
        .with_thumbnail(InputFile::file_id("thumb-file-id"))
        .unwrap_err();
    assert!(matches!(err, SendVideoNoteError::InvalidThumbnail));
}

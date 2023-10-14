use crate::{
    form::{Form, FormValue},
    tests::{assert_json_eq, assert_request_eq, ExpectedRequest},
    types::{ForceReply, InputFile, PhotoSize, SendVideoNote, VideoNote},
};

#[test]
fn video_note() {
    assert_json_eq(
        VideoNote {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            length: 124,
            duration: 1234,
            thumb: Some(PhotoSize {
                file_id: String::from("thumb-file-id"),
                file_unique_id: String::from("thumb-file-unique-id"),
                width: 24,
                height: 24,
                file_size: Some(1024),
            }),
            file_size: Some(10240),
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "length": 124,
            "duration": 1234,
            "thumb": {
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
        VideoNote {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            length: 50,
            duration: 60,
            thumb: None,
            file_size: None,
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "length": 50,
            "duration": 60,
        }),
    );
}

#[test]
fn send_video_note() {
    assert_request_eq(
        ExpectedRequest::post_form(
            "sendVideoNote",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video_note", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendVideoNote::new(1, InputFile::file_id("file-id")),
    );
    assert_request_eq(
        ExpectedRequest::post_form(
            "sendVideoNote",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("video_note", InputFile::file_id("file-id").into()),
                ("duration", 50.into()),
                ("length", 100.into()),
                ("thumb", InputFile::file_id("thumb-file-id").into()),
                ("disable_notification", true.into()),
                ("protect_content", true.into()),
                ("reply_to_message_id", 1.into()),
                ("allow_sending_without_reply", true.into()),
                (
                    "reply_markup",
                    serde_json::to_string(&ForceReply::new(true)).unwrap().into(),
                ),
            ]),
        ),
        SendVideoNote::new(1, InputFile::file_id("file-id"))
            .duration(50)
            .length(100)
            .thumb(InputFile::file_id("thumb-file-id"))
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .unwrap(),
    );
}

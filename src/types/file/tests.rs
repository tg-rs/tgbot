use std::io::Cursor;

use crate::{
    api::{Payload, assert_payload_eq},
    types::{File, GetFile, InputFile, InputFileReader, tests::assert_json_eq},
};

#[test]
fn file() {
    assert_json_eq(
        File::new("file-id", "file-unique-id")
            .with_file_path("file-path")
            .with_file_size(1024),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "file_size": 1024,
            "file_path": "file-path"
        }),
    );
    assert_json_eq(
        File::new("file-id", "file-unique-id"),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
        }),
    );
}

#[test]
fn get_file() {
    assert_payload_eq(
        Payload::json(
            "getFile",
            serde_json::json!({
                "file_id": "file-id"
            }),
        ),
        GetFile::new("file-id"),
    );
}

#[tokio::test]
async fn input_file() {
    let id = InputFile::file_id("file-id");
    assert_eq!(format!("{:?}", id), r#"Id("file-id")"#);

    let url = InputFile::url("http://example.com/archive.zip");
    assert_eq!(format!("{:?}", url), r#"Url("http://example.com/archive.zip")"#);

    // NOTE: you must be sure that file exists in current working directory (usually it exists)
    // otherwise test will fail
    let path = InputFile::path("LICENSE").await.unwrap();
    assert_eq!(
        format!("{:?}", path),
        r#"Reader(InputFileReader { file_name: Some("LICENSE"), mime_type: Some("application/octet-stream") })"#,
    );

    let reader = InputFileReader::from(Cursor::new(b"data"))
        .with_file_name("name")
        .with_mime_type(mime::TEXT_PLAIN);
    assert_eq!(reader.file_name().unwrap(), "name");
    assert_eq!(reader.mime_type().unwrap(), &mime::TEXT_PLAIN);
    let reader = InputFile::from(reader);
    assert_eq!(
        format!("{:?}", reader),
        r#"Reader(InputFileReader { file_name: Some("name"), mime_type: Some("text/plain") })"#,
    );

    let reader = InputFile::from(Cursor::new(b"data"));
    assert_eq!(
        format!("{:?}", reader),
        "Reader(InputFileReader { file_name: None, mime_type: None })",
    );
}

use std::io::Cursor;

use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, File, GetFile, InputFile, InputFileReader},
};

#[test]
fn file() {
    assert_json_eq(
        File {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            file_size: Some(1024),
            file_path: Some(String::from("file-path")),
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "file_size": 1024,
            "file_path": "file-path"
        }),
    );
    assert_json_eq(
        File {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            file_size: None,
            file_path: None,
        },
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
    assert_eq!(format!("{:?}", id), r#"InputFile { kind: Id("file-id") }"#);

    let url = InputFile::url("http://example.com/archive.zip");
    assert_eq!(
        format!("{:?}", url),
        r#"InputFile { kind: Url("http://example.com/archive.zip") }"#
    );

    // NOTE: you must be sure that file exists in current working directory (usually it exists)
    // otherwise test will fail
    let path = InputFile::path("LICENSE").await.unwrap();
    assert_eq!(
        format!("{:?}", path),
        r#"InputFile { kind: Reader(InputFileReader { file_name: Some("LICENSE"), mime_type: Some("application/octet-stream") }) }"#,
    );

    let reader = InputFileReader::from(Cursor::new(b"data"))
        .with_file_name("name")
        .with_mime_type(mime::TEXT_PLAIN);
    assert_eq!(reader.file_name().unwrap(), "name");
    assert_eq!(reader.mime_type().unwrap(), &mime::TEXT_PLAIN);
    let reader = InputFile::from(reader);
    assert_eq!(
        format!("{:?}", reader),
        r#"InputFile { kind: Reader(InputFileReader { file_name: Some("name"), mime_type: Some("text/plain") }) }"#,
    );

    let reader = InputFile::from(Cursor::new(b"data"));
    assert_eq!(
        format!("{:?}", reader),
        "InputFile { kind: Reader(InputFileReader { file_name: None, mime_type: None }) }",
    );
}

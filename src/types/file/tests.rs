use std::io::Cursor;

use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, File, GetFile, InputFile, InputFileInfo, InputFileReader},
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
        r#"InputFile { kind: Reader(InputFileReader(reader: ..., info: Some(InputFileInfo { name: "LICENSE", mime_type: Some("application/octet-stream") }))) }"#,
    );

    let reader = InputFile::from(InputFileReader::from(Cursor::new(b"data")).info(("name", mime::TEXT_PLAIN)));
    assert_eq!(
        format!("{:?}", reader),
        r#"InputFile { kind: Reader(InputFileReader(reader: ..., info: Some(InputFileInfo { name: "name", mime_type: Some("text/plain") }))) }"#,
    );

    let reader = InputFile::from(Cursor::new(b"data"));
    assert_eq!(
        format!("{:?}", reader),
        "InputFile { kind: Reader(InputFileReader(reader: ..., info: None)) }",
    );
}

#[test]
fn input_file_info() {
    let info = InputFileInfo::from("name");
    assert_eq!(info.name(), "name");
    assert!(info.mime_type().is_none());

    let info = InputFileInfo::from(("name", mime::TEXT_PLAIN));
    assert_eq!(info.name(), "name");
    assert_eq!(info.mime_type().unwrap(), &mime::TEXT_PLAIN);

    let info = InputFileInfo::from(String::from("name"));
    assert_eq!(info.name(), "name");
    assert!(info.mime_type().is_none());

    let info = InputFileInfo::from((String::from("name"), mime::TEXT_PLAIN));
    assert_eq!(info.name(), "name");
    assert_eq!(info.mime_type().unwrap(), &mime::TEXT_PLAIN);
}

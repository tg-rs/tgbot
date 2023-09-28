use std::io::Cursor;

use crate::{
    method::Method,
    request::{RequestBody, RequestMethod},
    types::{File, GetFile, InputFile, InputFileInfo, InputFileReader},
};

#[test]
fn file_deserialize_full() {
    let data: File = serde_json::from_value(serde_json::json!({
        "file_id": "id",
        "file_unique_id": "unique-id",
        "file_size": 123,
        "file_path": "path"
    }))
    .unwrap();
    assert_eq!(data.file_id, "id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert_eq!(data.file_size.unwrap(), 123);
    assert_eq!(data.file_path.unwrap(), "path");
}

#[test]
fn file_deserialize_partial() {
    let data: File = serde_json::from_value(serde_json::json!({
        "file_id": "id",
        "file_unique_id": "unique-id"
    }))
    .unwrap();
    assert_eq!(data.file_id, "id");
    assert_eq!(data.file_unique_id, "unique-id");
    assert!(data.file_size.is_none());
    assert!(data.file_path.is_none());
}

#[test]
fn get_file() {
    let request = GetFile::new("file-id").into_request();
    assert_eq!(request.get_method(), RequestMethod::Post);
    assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/getFile");
    if let RequestBody::Json(data) = request.into_body() {
        assert_eq!(data.unwrap(), r#"{"file_id":"file-id"}"#);
    } else {
        panic!("Unexpected request body");
    }
}

#[tokio::test]
async fn input_file() {
    let id = InputFile::file_id("file-id");
    assert_eq!(
        format!("{:?}", id),
        r#"InputFile { kind: InputFileKind::Id("file-id") }"#
    );
    let url = InputFile::url("http://example.com/archive.zip");
    assert_eq!(
        format!("{:?}", url),
        r#"InputFile { kind: InputFileKind::Url("http://example.com/archive.zip") }"#
    );
    // NOTE: you must be sure that file exists in current working directory (usually it exists)
    // otherwise test will fail
    let path = InputFile::path("LICENSE").await.unwrap();
    let repr = format!("{:?}", path);
    assert!(
        repr.starts_with("InputFile { kind: InputFileKind::Reader("),
        "repr: {}",
        repr
    );

    let reader = InputFileReader::from(Cursor::new(b"data")).info(("name", mime::TEXT_PLAIN));
    let reader = InputFile::from(reader);
    let repr = format!("{:?}", reader);
    assert!(
        repr.starts_with("InputFile { kind: InputFileKind::Reader("),
        "repr: {}",
        repr
    );

    let reader = InputFile::from(Cursor::new(b"data"));
    let repr = format!("{:?}", reader);
    assert!(
        repr.starts_with("InputFile { kind: InputFileKind::Reader("),
        "repr: {}",
        repr
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

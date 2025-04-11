use crate::{
    api::{Payload, assert_payload_eq},
    types::{File, GetFile, tests::assert_json_eq},
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

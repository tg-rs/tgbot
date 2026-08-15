#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        ("get-file-base", GetFile::new("file-id"), |x| {
            assert_eq!(x.file_id, "file-id");
            assert_eq!(x.file_unique_id, "file-unique-id");
            assert!(x.file_size.is_none());
            assert!(x.file_path.is_none());
        }),
        ("get-file-all", GetFile::new("file-id"), |x| {
            assert_eq!(x.file_id, "file-id");
            assert_eq!(x.file_unique_id, "file-unique-id");
            assert_eq!(x.file_size.unwrap(), 10000);
            assert_eq!(x.file_path.unwrap(), "file-path");
        }),
    ])
    .await;
}

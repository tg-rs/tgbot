use crate::types::{LivePhoto, PhotoSize};

#[test]
fn live_photo() {
    let expected_struct = LivePhoto::new(1, "file-id", "file-unique-id", 200, 300);
    insta::assert_json_snapshot!(expected_struct.clone());

    let expected_struct = expected_struct
        .with_mime_type("video/gif")
        .with_file_size(1000)
        .with_photo([PhotoSize::new("p-id", "p-uid", 200, 300)]);
    insta::assert_json_snapshot!(expected_struct);
}

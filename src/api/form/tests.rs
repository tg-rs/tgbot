use std::io::Cursor;

use super::*;

#[test]
fn form_value() {
    let val = FormValue::from(1);
    assert_eq!(val.get_text().unwrap(), "1");
    assert!(val.get_file().is_none());

    let val = FormValue::from(InputFile::file_id("file-id"));
    assert!(val.get_text().is_none());
    assert!(val.get_file().is_some());
}

#[test]
fn form() {
    Form::from([
        ("id", 1.into()),
        ("file-id", InputFile::file_id("file-id").into()),
        ("file-url", InputFile::url("url").into()),
        ("file-reader", InputFile::from(Cursor::new(b"test")).into()),
    ])
    .into_multipart()
    .unwrap();
}

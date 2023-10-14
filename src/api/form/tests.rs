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
    let mut form = Form::new();
    form.insert_field("id", 1);
    form.insert_field("file-id", InputFile::file_id("file-id"));
    form.insert_field("file-url", InputFile::url("url"));
    form.insert_field("file-reader", InputFile::from(Cursor::new(b"test")));
    form.into_multipart().unwrap();
}

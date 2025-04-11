use std::io::Cursor;

use crate::{
    api::Form,
    types::{InputFile, InputProfilePhoto, InputProfilePhotoAnimated, InputProfilePhotoStatic},
};

#[test]
fn input_profile_photo() {
    let animated = InputProfilePhotoAnimated::new(InputFile::url("url"));
    let photo = InputProfilePhoto::from(animated);
    assert!(matches!(photo, InputProfilePhoto::Animated(_)));
    assert_eq!(
        Form::from([("photo", r#"{"type":"animated","animation":"url"}"#.into())]),
        Form::try_from(photo).unwrap()
    );

    let animated = InputProfilePhotoAnimated::new(Cursor::new("test"));
    let photo = InputProfilePhoto::from(animated);
    assert!(matches!(photo, InputProfilePhoto::Animated(_)));
    assert_eq!(
        Form::from([
            (
                "photo",
                r#"{"type":"animated","animation":"attach://tgbot_ipp_file"}"#.into()
            ),
            ("tgbot_ipp_file", InputFile::from(Cursor::new("test")).into())
        ]),
        Form::try_from(photo).unwrap()
    );

    let animated = InputProfilePhotoAnimated::new(InputFile::url("url")).with_main_frame_timestamp(1.0);
    let photo = InputProfilePhoto::from(animated);
    assert!(matches!(photo, InputProfilePhoto::Animated(_)));
    assert_eq!(
        Form::from([(
            "photo",
            r#"{"type":"animated","animation":"url","main_frame_timestamp":1.0}"#.into()
        )]),
        Form::try_from(photo).unwrap()
    );

    let photo = InputProfilePhoto::from(InputProfilePhotoStatic::new(InputFile::url("url")));
    assert!(matches!(photo, InputProfilePhoto::Static(_)));
    assert_eq!(
        Form::from([("photo", r#"{"type":"static","photo":"url"}"#.into())]),
        Form::try_from(photo).unwrap()
    );

    let photo = InputProfilePhoto::from(InputProfilePhotoStatic::new(Cursor::new("test")));
    assert!(matches!(photo, InputProfilePhoto::Static(_)));
    assert_eq!(
        Form::from([
            ("photo", r#"{"type":"static","photo":"attach://tgbot_ipp_file"}"#.into()),
            ("tgbot_ipp_file", InputFile::from(Cursor::new("test")).into())
        ]),
        Form::try_from(photo).unwrap()
    );
}

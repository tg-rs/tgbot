use std::io::Cursor;

use crate::{
    api::Form,
    types::{InputFile, InputStoryContent, InputStoryContentPhoto, InputStoryContentVideo},
};

#[test]
fn input_story_content() {
    let content = InputStoryContent::from(InputStoryContentPhoto::new(InputFile::url("url")));
    assert!(matches!(content, InputStoryContent::Photo(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([("story", r#"{"type":"photo","photo":"url"}"#.into())]),
        form
    );

    let content = InputStoryContent::from(InputStoryContentPhoto::new(Cursor::new("test")));
    assert!(matches!(content, InputStoryContent::Photo(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([
            ("story", r#"{"type":"photo","photo":"attach://tgbot_isc_file"}"#.into()),
            ("tgbot_isc_file", InputFile::from(Cursor::new("test")).into())
        ]),
        form
    );

    let content = InputStoryContent::from(InputStoryContentVideo::new(InputFile::url("url")));
    assert!(matches!(content, InputStoryContent::Video(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([("story", r#"{"type":"video","video":"url"}"#.into())]),
        form
    );

    let content = InputStoryContent::from(InputStoryContentVideo::new(Cursor::new("test")));
    assert!(matches!(content, InputStoryContent::Video(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([
            ("story", r#"{"type":"video","video":"attach://tgbot_isc_file"}"#.into()),
            ("tgbot_isc_file", InputFile::from(Cursor::new("test")).into())
        ]),
        form
    );

    let content = InputStoryContent::from(
        InputStoryContentVideo::new(InputFile::url("url"))
            .with_cover_frame_timestamp(1.0)
            .with_duration(1.0)
            .with_is_animation(true),
    );
    assert!(matches!(content, InputStoryContent::Video(_)));
    let form: Form = content.try_into().unwrap();
    assert_eq!(
        Form::from([(
            "story",
            r#"{"type":"video","video":"url","cover_frame_timestamp":1.0,"duration":1.0,"is_animation":true}"#.into()
        )]),
        form
    );
}

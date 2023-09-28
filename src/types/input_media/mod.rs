use crate::{
    request::Form,
    types::{InputFile, InputFileKind},
};
use serde::Serialize;
use serde_json::Error as JsonError;
use std::{error::Error, fmt};

mod animation;
mod audio;
mod document;
mod photo;
mod video;

pub use self::{animation::*, audio::*, document::*, photo::*, video::*};

/// Content of a media message to be sent
#[derive(Debug)]
pub struct InputMedia {
    form: Form,
}

impl InputMedia {
    /// Creates a new input media
    pub fn new<F, K>(file: F, kind: K) -> Result<InputMedia, InputMediaError>
    where
        F: Into<InputFile>,
        K: Into<InputMediaKind>,
    {
        Self::create(file, None::<InputFile>, kind)
    }

    /// Creates a new input media with thumbnail
    ///
    /// Note that photo can not have a thumbnail
    pub fn with_thumb<F, T, K>(file: F, thumb: T, kind: K) -> Result<InputMedia, InputMediaError>
    where
        F: Into<InputFile>,
        T: Into<InputFile>,
        K: Into<InputMediaKind>,
    {
        Self::create(file, Some(thumb), kind)
    }

    fn create<K, F, T>(media: F, thumb: Option<T>, kind: K) -> Result<Self, InputMediaError>
    where
        K: Into<InputMediaKind>,
        F: Into<InputFile>,
        T: Into<InputFile>,
    {
        let mut form = Form::new();

        let add_file = |form: &mut Form, key: &str, file: InputFile| -> String {
            match file.kind {
                InputFileKind::Id(text) | InputFileKind::Url(text) => text,
                _ => {
                    form.insert_field(key, file);
                    format!("attach://{}", key)
                }
            }
        };

        let media = add_file(&mut form, "tgbot_im_file", media.into());
        let thumb = thumb.map(|thumb| add_file(&mut form, "tgbot_im_thumb", thumb.into()));
        let data = match kind.into() {
            InputMediaKind::Animation(info) => InputMediaData::Animation { media, thumb, info },
            InputMediaKind::Audio(info) => InputMediaData::Audio { media, thumb, info },
            InputMediaKind::Document(info) => InputMediaData::Document { media, thumb, info },
            InputMediaKind::Photo(info) => InputMediaData::Photo { media, info },
            InputMediaKind::Video(info) => InputMediaData::Video { media, thumb, info },
        };
        let info = serde_json::to_string(&data).map_err(InputMediaError::SerializeInfo)?;
        form.insert_field("media", info);
        Ok(Self { form })
    }
}

impl From<InputMedia> for Form {
    fn from(value: InputMedia) -> Self {
        value.form
    }
}

/// Metadata of the input media
#[derive(Debug, derive_more::From)]
pub enum InputMediaKind {
    /// An animation file
    Animation(InputMediaAnimation),
    /// An audio file
    Audio(InputMediaAudio),
    /// A general file
    Document(InputMediaDocument),
    /// A photo
    Photo(InputMediaPhoto),
    /// A video file
    Video(InputMediaVideo),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum InputMediaData {
    Animation {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaAnimation,
    },
    Audio {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaAudio,
    },
    Document {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaDocument,
    },
    Photo {
        media: String,
        #[serde(flatten)]
        info: InputMediaPhoto,
    },
    Video {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaVideo,
    },
}

/// An error occurred with InputMedia
#[derive(Debug)]
pub enum InputMediaError {
    /// Can not serialize media info
    SerializeInfo(JsonError),
}

impl Error for InputMediaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InputMediaError::SerializeInfo(err) => Some(err),
        }
    }
}

impl fmt::Display for InputMediaError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputMediaError::SerializeInfo(err) => write!(out, "failed to serialize input media info: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::InputFileReader;
    use std::io::Cursor;

    #[test]
    fn input_media() {
        let data = InputMedia::new(
            InputFile::file_id("animation-file-id"),
            InputMediaAnimation::default().caption("test"),
        )
        .unwrap()
        .into_form();
        assert!(data.get("media").is_some());

        let data = InputMedia::with_thumb(
            InputFileReader::from(Cursor::new("animation-file-data")),
            InputFileReader::from(Cursor::new("animation-thumb-data")),
            InputMediaAnimation::default(),
        )
        .unwrap()
        .into_form();
        assert!(data.get("tgbot_im_file").is_some());
        assert!(data.get("tgbot_im_thumb").is_some());
        assert!(data.get("media").is_some());
    }

    #[test]
    fn input_media_kind() {
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                InputMediaAnimation::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "animation",
                "media": "file-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                String::from("thumb-id"),
                InputMediaAnimation::default().caption("test"),
            )))
            .unwrap(),
            serde_json::json!({
                "type": "animation",
                "media": "file-id",
                "thumb": "thumb-id",
                "caption": "test"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                InputMediaAudio::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "audio",
                "media": "file-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                String::from("thumb-id"),
                InputMediaAudio::default().caption("test"),
            )))
            .unwrap(),
            serde_json::json!({
                "type": "audio",
                "media": "file-id",
                "thumb": "thumb-id",
                "caption": "test"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                InputMediaDocument::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "document",
                "media": "file-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                String::from("thumb-id"),
                InputMediaDocument::default().caption("test"),
            )))
            .unwrap(),
            serde_json::json!({
                "type": "document",
                "media": "file-id",
                "thumb": "thumb-id",
                "caption": "test"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                InputMediaVideo::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "video",
                "media": "file-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                String::from("thumb-id"),
                InputMediaVideo::default().caption("test"),
            )))
            .unwrap(),
            serde_json::json!({
                "type": "video",
                "media": "file-id",
                "thumb": "thumb-id",
                "caption": "test"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                InputMediaPhoto::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "photo",
                "media": "file-id",
                "caption": "test"
            })
        );
    }
}

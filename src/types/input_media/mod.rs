use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::{
    api::Form,
    types::{InputFile, InputFileKind},
};

pub use self::{animation::*, audio::*, document::*, photo::*, video::*};

#[cfg(test)]
mod tests;

mod animation;
mod audio;
mod document;
mod photo;
mod video;

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
    pub fn with_thumbnail<F, T, K>(file: F, thumbnail: T, kind: K) -> Result<InputMedia, InputMediaError>
    where
        F: Into<InputFile>,
        T: Into<InputFile>,
        K: Into<InputMediaKind>,
    {
        Self::create(file, Some(thumbnail), kind)
    }

    fn create<K, F, T>(media: F, thumbnail: Option<T>, kind: K) -> Result<Self, InputMediaError>
    where
        K: Into<InputMediaKind>,
        F: Into<InputFile>,
        T: Into<InputFile>,
    {
        let mut form = Form::default();

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
        let thumbnail = thumbnail.map(|thumb| add_file(&mut form, "tgbot_im_thumb", thumb.into()));
        let data = match kind.into() {
            InputMediaKind::Animation(info) => InputMediaData::Animation { media, thumbnail, info },
            InputMediaKind::Audio(info) => InputMediaData::Audio { media, thumbnail, info },
            InputMediaKind::Document(info) => InputMediaData::Document { media, thumbnail, info },
            InputMediaKind::Photo(info) => InputMediaData::Photo { media, info },
            InputMediaKind::Video(info) => InputMediaData::Video { media, thumbnail, info },
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

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum InputMediaData {
    Animation {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputMediaAnimation,
    },
    Audio {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputMediaAudio,
    },
    Document {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
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
        thumbnail: Option<String>,
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

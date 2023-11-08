use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::{api::Form, types::InputFile};

pub use self::{animation::*, audio::*, document::*, photo::*, video::*};

#[cfg(test)]
mod tests;

mod animation;
mod audio;
mod document;
mod photo;
mod video;

/// Represents a content of a media message to be sent
#[derive(Debug)]
pub struct InputMedia {
    form: Form,
}

impl InputMedia {
    /// Creates a new input media
    ///
    /// # Arguments
    ///
    /// * file - File to send
    /// * media_type - Metadata of the media
    pub fn new<A, B>(file: A, media_type: B) -> Result<InputMedia, InputMediaError>
    where
        A: Into<InputFile>,
        B: Into<InputMediaType>,
    {
        Self::create(file, media_type, None::<InputFile>)
    }

    /// Creates a new input media with thumbnail
    ///
    /// # Arguments
    ///
    /// * file - File to send
    /// * media_type - Metadata of the media
    /// * thumbnail - Thumbnail file
    ///
    /// Note that photo can not have a thumbnail
    pub fn with_thumbnail<A, B, C>(file: A, media_type: B, thumbnail: C) -> Result<InputMedia, InputMediaError>
    where
        A: Into<InputFile>,
        B: Into<InputMediaType>,
        C: Into<InputFile>,
    {
        Self::create(file, media_type, Some(thumbnail))
    }

    fn create<A, B, C>(media: A, media_type: B, thumbnail: Option<C>) -> Result<Self, InputMediaError>
    where
        A: Into<InputFile>,
        B: Into<InputMediaType>,
        C: Into<InputFile>,
    {
        let mut form = Form::default();

        let add_file = |form: &mut Form, key: &str, file: InputFile| -> String {
            match file {
                InputFile::Id(text) | InputFile::Url(text) => text,
                _ => {
                    form.insert_field(key, file);
                    format!("attach://{}", key)
                }
            }
        };

        let media = add_file(&mut form, "tgbot_im_file", media.into());
        let thumbnail = thumbnail.map(|thumb| add_file(&mut form, "tgbot_im_thumb", thumb.into()));
        let data = match media_type.into() {
            InputMediaType::Animation(info) => InputMediaData::Animation { media, thumbnail, info },
            InputMediaType::Audio(info) => InputMediaData::Audio { media, thumbnail, info },
            InputMediaType::Document(info) => InputMediaData::Document { media, thumbnail, info },
            InputMediaType::Photo(info) => InputMediaData::Photo { media, info },
            InputMediaType::Video(info) => InputMediaData::Video { media, thumbnail, info },
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

/// Represents a metadata of the input media
#[derive(Debug, derive_more::From)]
pub enum InputMediaType {
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

/// An error occurred with [`InputMedia`]
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

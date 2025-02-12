use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

pub use self::{animation::*, audio::*, document::*, photo::*, video::*};
use crate::{api::Form, types::InputFile};

#[cfg(test)]
mod tests;

mod animation;
mod audio;
mod document;
mod photo;
mod video;

/// Represents a content of a media message to be sent.
#[derive(Debug)]
pub struct InputMedia {
    form: Form,
}

impl InputMedia {
    /// Creates a new `InputMedia`.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The type of the media.
    pub fn new(mut media_type: InputMediaType) -> Result<Self, InputMediaError> {
        let info = serde_json::to_string(&media_type.data).map_err(InputMediaError::SerializeInfo)?;
        media_type.form.insert_field("media", info);
        Ok(Self { form: media_type.form })
    }
}

impl From<InputMedia> for Form {
    fn from(value: InputMedia) -> Self {
        value.form
    }
}

/// Represents a metadata of the input media.
#[derive(Debug)]
pub struct InputMediaType {
    form: Form,
    data: InputMediaData,
}

impl InputMediaType {
    /// Creates a new `InputMediaType` for animation.
    ///
    /// # Arguments
    ///
    pub fn for_animation<T>(media: T, info: InputMediaAnimation) -> Self
    where
        T: Into<InputFile>,
    {
        let (media, form) = create_form(media);
        Self {
            form,
            data: InputMediaData::Animation {
                media,
                thumbnail: None,
                info,
            },
        }
    }

    /// Creates a new `InputMediaType` for audio.
    ///
    /// # Arguments
    ///
    pub fn for_audio<T>(media: T, info: InputMediaAudio) -> Self
    where
        T: Into<InputFile>,
    {
        let (media, form) = create_form(media);
        Self {
            form,
            data: InputMediaData::Audio {
                media,
                thumbnail: None,
                info,
            },
        }
    }

    /// Creates a new `InputMediaType` for document.
    ///
    /// # Arguments
    ///
    pub fn for_document<T>(media: T, info: InputMediaDocument) -> Self
    where
        T: Into<InputFile>,
    {
        let (media, form) = create_form(media);
        Self {
            form,
            data: InputMediaData::Document {
                media,
                thumbnail: None,
                info,
            },
        }
    }

    /// Creates a new `InputMediaType` for photo.
    ///
    /// # Arguments
    ///
    pub fn for_photo<T>(media: T, info: InputMediaPhoto) -> Self
    where
        T: Into<InputFile>,
    {
        let (media, form) = create_form(media);
        Self {
            form,
            data: InputMediaData::Photo { media, info },
        }
    }

    /// Creates a new `InputMediaType` for video.
    ///
    /// # Arguments
    ///
    pub fn for_video<T>(media: T, info: InputMediaVideo) -> Self
    where
        T: Into<InputFile>,
    {
        let (media, form) = create_form(media);
        Self {
            form,
            data: InputMediaData::Video {
                media,
                cover: None,
                thumbnail: None,
                info,
            },
        }
    }

    /// Sets a new cover for the media type.
    ///
    /// # Arguments
    ///
    /// * `value` - The new thumbnail.
    ///
    /// # Errors
    ///
    /// It is considered an error when the media type is not a video.
    pub fn with_cover<T>(mut self, value: T) -> Result<Self, InputMediaError>
    where
        T: Into<InputFile>,
    {
        match &mut self.data {
            InputMediaData::Animation { .. }
            | InputMediaData::Audio { .. }
            | InputMediaData::Document { .. }
            | InputMediaData::Photo { .. } => return Err(InputMediaError::CoverNotAcceptable),
            InputMediaData::Video { cover, .. } => {
                let new_cover = match value.into() {
                    InputFile::Id(text) | InputFile::Url(text) => text,
                    value => {
                        let key = "tgbot_im_cover";
                        self.form.insert_field(key, value);
                        format!("attach://{}", key)
                    }
                };
                *cover = Some(new_cover);
            }
        };
        Ok(self)
    }

    /// Sets a new thumbnail for the media type.
    ///
    /// # Arguments
    ///
    /// * `value` - The new thumbnail.
    ///
    /// # Errors
    ///
    /// It is considered an error when the media type is a photo.
    pub fn with_thumbnail<T>(mut self, value: T) -> Result<Self, InputMediaError>
    where
        T: Into<InputFile>,
    {
        let new_thumbnail = match value.into() {
            InputFile::Id(text) | InputFile::Url(text) => text,
            value => {
                let key = "tgbot_im_thumb";
                self.form.insert_field(key, value);
                format!("attach://{}", key)
            }
        };
        match &mut self.data {
            InputMediaData::Animation { thumbnail, .. } => {
                *thumbnail = Some(new_thumbnail);
            }
            InputMediaData::Audio { thumbnail, .. } => {
                *thumbnail = Some(new_thumbnail);
            }
            InputMediaData::Document { thumbnail, .. } => {
                *thumbnail = Some(new_thumbnail);
            }
            InputMediaData::Photo { .. } => return Err(InputMediaError::ThumbnailNotAcceptable),
            InputMediaData::Video { thumbnail, .. } => {
                *thumbnail = Some(new_thumbnail);
            }
        };
        Ok(self)
    }
}

fn create_form<T>(media: T) -> (String, Form)
where
    T: Into<InputFile>,
{
    let mut form = Form::default();
    let media = match media.into() {
        InputFile::Id(text) | InputFile::Url(text) => text,
        media => {
            let key = "tgbot_im_file";
            form.insert_field(key, media);
            format!("attach://{}", key)
        }
    };
    (media, form)
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
        cover: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputMediaVideo,
    },
}

/// An error occurred with [`InputMedia`].
#[derive(Debug)]
pub enum InputMediaError {
    /// Can not set a cover for the media type.
    CoverNotAcceptable,
    /// Can not serialize media info.
    SerializeInfo(JsonError),
    /// Can not set a thumbnail for the media type.
    ThumbnailNotAcceptable,
}

impl Error for InputMediaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CoverNotAcceptable => None,
            Self::SerializeInfo(err) => Some(err),
            Self::ThumbnailNotAcceptable => None,
        }
    }
}

impl fmt::Display for InputMediaError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CoverNotAcceptable => write!(out, "can not set a cover"),
            Self::SerializeInfo(err) => write!(out, "failed to serialize input media info: {}", err),
            Self::ThumbnailNotAcceptable => write!(out, "can not set a thumbnail"),
        }
    }
}

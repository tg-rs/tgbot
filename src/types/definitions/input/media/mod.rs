use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

pub use self::{
    animation::*,
    audio::*,
    document::*,
    live_photo::*,
    location::*,
    photo::*,
    sticker::*,
    venue::*,
    video::*,
};
use crate::{api::Form, types::InputFile};

mod animation;
mod audio;
mod document;
mod live_photo;
mod location;
mod photo;
mod sticker;
mod venue;
mod video;

/// Represents a metadata of the input media.
#[derive(Debug)]
pub struct InputMedia {
    form: Form,
    data: InputMediaData,
}

impl InputMedia {
    /// Creates a new `InputMedia` for animation.
    ///
    /// # Arguments
    ///
    /// * `media` - The media to upload.
    /// * `info` - Information about the media.
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

    /// Creates a new `InputMedia` for audio.
    ///
    /// # Arguments
    ///
    /// * `media` - The media to upload.
    /// * `info` - Information about the media.
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

    /// Creates a new `InputMedia` for document.
    ///
    /// # Arguments
    ///
    /// * `media` - The media to upload.
    /// * `info` - Information about the media.
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

    /// Creates a new `InputMedia` for an HTTP link.
    ///
    /// # Arguments
    ///
    /// * `value` - HTTP URL of the link
    pub fn for_link<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            form: Form::default(),
            data: InputMediaData::Link { url: value.into() },
        }
    }

    /// Creates a new `InputMedia` for live photo.
    ///
    /// # Arguments
    ///
    /// * `media` - Video of the live photo to be send.
    /// * `photo` - The static phot to send.
    /// * `info` - Information about the media.
    pub fn for_live_photo<A, B>(media: A, photo: B, info: InputMediaLivePhoto) -> Self
    where
        A: Into<InputFile>,
        B: Into<InputFile>,
    {
        let (media, mut form) = create_form(media);
        let photo = match photo.into() {
            InputFile::Id(text) | InputFile::Url(text) => text,
            value => {
                let key = "tgbot_im_photo";
                form.insert_field(key, value);
                format!("attach://{key}")
            }
        };
        Self {
            form,
            data: InputMediaData::LivePhoto { media, photo, info },
        }
    }

    /// Creates a new `InputMedia` for location.
    ///
    /// # Arguments
    ///
    /// * `info` - Information about the media.
    pub fn for_location(info: InputMediaLocation) -> Self {
        Self {
            form: Form::default(),
            data: InputMediaData::Location { info },
        }
    }

    /// Creates a new `InputMedia` for photo.
    ///
    /// # Arguments
    ///
    /// * `media` - The media to upload.
    /// * `info` - Information about the media.
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

    /// Creates a new `InputMedia` for sticker.
    ///
    /// # Arguments
    ///
    /// * `media` - The media to upload.
    /// * `info` - Information about the media.
    pub fn for_sticker<T>(media: T, info: InputMediaSticker) -> Self
    where
        T: Into<InputFile>,
    {
        let (media, form) = create_form(media);
        Self {
            form,
            data: InputMediaData::Sticker { media, info },
        }
    }

    /// Creates a new `InputMedia` for venue.
    ///
    /// # Arguments
    ///
    /// * `media` - The media to upload.
    /// * `info` - Information about the media.
    pub fn for_venue(info: InputMediaVenue) -> Self {
        Self {
            form: Form::default(),
            data: InputMediaData::Venue { info },
        }
    }

    /// Creates a new `InputMedia` for video.
    ///
    /// # Arguments
    ///
    /// * `media` - The media to upload.
    /// * `info` - Information about the media.
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

    /// Sets a new cover for the media.
    ///
    /// # Arguments
    ///
    /// * `value` - The new thumbnail.
    ///
    /// # Errors
    ///
    /// It is considered an error when the media is not a video.
    pub fn with_cover<T>(mut self, value: T) -> Result<Self, InputMediaError>
    where
        T: Into<InputFile>,
    {
        match &mut self.data {
            InputMediaData::Animation { .. }
            | InputMediaData::Audio { .. }
            | InputMediaData::Document { .. }
            | InputMediaData::Link { .. }
            | InputMediaData::LivePhoto { .. }
            | InputMediaData::Location { .. }
            | InputMediaData::Photo { .. }
            | InputMediaData::Sticker { .. }
            | InputMediaData::Venue { .. } => return Err(InputMediaError::CoverNotAcceptable),
            InputMediaData::Video { cover, .. } => {
                let new_cover = match value.into() {
                    InputFile::Id(text) | InputFile::Url(text) => text,
                    value => {
                        let key = "tgbot_im_cover";
                        self.form.insert_field(key, value);
                        format!("attach://{key}")
                    }
                };
                *cover = Some(new_cover);
            }
        };
        Ok(self)
    }

    /// Sets a new thumbnail for the media.
    ///
    /// # Arguments
    ///
    /// * `value` - The new thumbnail.
    ///
    /// # Errors
    ///
    /// It is considered an error when the media is a
    /// link, live photo, location, photo, sticker or venue.
    pub fn with_thumbnail<T>(mut self, value: T) -> Result<Self, InputMediaError>
    where
        T: Into<InputFile>,
    {
        let new_thumbnail = match value.into() {
            InputFile::Id(text) | InputFile::Url(text) => text,
            value => {
                let key = "tgbot_im_thumb";
                self.form.insert_field(key, value);
                format!("attach://{key}")
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
            InputMediaData::Link { .. }
            | InputMediaData::LivePhoto { .. }
            | InputMediaData::Location { .. }
            | InputMediaData::Photo { .. }
            | InputMediaData::Sticker { .. }
            | InputMediaData::Venue { .. } => {
                return Err(InputMediaError::ThumbnailNotAcceptable);
            }
            InputMediaData::Video { thumbnail, .. } => {
                *thumbnail = Some(new_thumbnail);
            }
        };
        Ok(self)
    }

    pub(crate) fn into_parts(self) -> (Form, InputMediaData) {
        (self.form, self.data)
    }

    pub(crate) fn try_into_form(mut self, field_name: &'static str) -> Result<Form, InputMediaError> {
        let info = serde_json::to_string(&self.data).map_err(InputMediaError::SerializeInfo)?;
        self.form.insert_field(field_name, info);
        Ok(self.form)
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
            format!("attach://{key}")
        }
    };
    (media, form)
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub(crate) enum InputMediaData {
    Animation {
        media: String,
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputMediaAnimation,
    },
    Audio {
        media: String,
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputMediaAudio,
    },
    Document {
        media: String,
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputMediaDocument,
    },
    Link {
        url: String,
    },
    LivePhoto {
        media: String,
        photo: String,
        #[serde(flatten)]
        info: InputMediaLivePhoto,
    },
    Location {
        #[serde(flatten)]
        info: InputMediaLocation,
    },
    Photo {
        media: String,
        #[serde(flatten)]
        info: InputMediaPhoto,
    },
    Sticker {
        media: String,
        #[serde(flatten)]
        info: InputMediaSticker,
    },
    Venue {
        #[serde(flatten)]
        info: InputMediaVenue,
    },
    Video {
        media: String,
        cover: Option<String>,
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputMediaVideo,
    },
}

/// An error occurred with [`InputMedia`].
#[derive(Debug)]
pub enum InputMediaError {
    /// Can not set a cover for the input media.
    CoverNotAcceptable,
    /// Can not serialize media info.
    SerializeInfo(JsonError),
    /// Can not set a thumbnail for the input media.
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
            Self::SerializeInfo(err) => write!(out, "failed to serialize input media info: {err}"),
            Self::ThumbnailNotAcceptable => write!(out, "can not set a thumbnail"),
        }
    }
}

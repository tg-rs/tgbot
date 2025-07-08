use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::{
    api::Form,
    types::{Float, InputFile},
};

/// Describes a profile photo to set.
#[derive(Debug, derive_more::From)]
pub enum InputProfilePhoto {
    /// An animated profile photo.
    Animated(InputProfilePhotoAnimated),
    /// A static profile photo.
    Static(InputProfilePhotoStatic),
}

impl TryFrom<InputProfilePhoto> for Form {
    type Error = InputProfilePhotoError;

    fn try_from(value: InputProfilePhoto) -> Result<Self, Self::Error> {
        match value {
            InputProfilePhoto::Animated(value) => value.try_into(),
            InputProfilePhoto::Static(value) => value.try_into(),
        }
    }
}

/// A static profile photo in the JPEG format.
#[derive(Debug)]
pub struct InputProfilePhotoStatic {
    photo: InputFile,
}

impl InputProfilePhotoStatic {
    /// Creates a new `InputProfilePhotoStatic`.
    ///
    /// # Arguments
    ///
    /// * `photo` - The static profile photo;
    ///   profile photos can't be reused and can only be uploaded as a new file.
    pub fn new<T>(photo: T) -> Self
    where
        T: Into<InputFile>,
    {
        Self { photo: photo.into() }
    }
}

/// An animated profile photo in the MPEG4 format.
#[derive(Debug)]
pub struct InputProfilePhotoAnimated {
    animation: InputFile,
    main_frame_timestamp: Option<Float>,
}

impl InputProfilePhotoAnimated {
    /// Creates a new `InputProfilePhotoAnimated`.
    ///
    /// # Arguments
    ///
    /// * `animation` - The animated profile photo;
    ///   profile photos can't be reused and can only be uploaded as a new file.
    pub fn new<T>(animation: T) -> Self
    where
        T: Into<InputFile>,
    {
        Self {
            animation: animation.into(),
            main_frame_timestamp: None,
        }
    }

    /// Sets a new main frame timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    pub fn with_main_frame_timestamp(mut self, value: Float) -> Self {
        self.main_frame_timestamp = Some(value);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawInputProfilePhoto {
    Animated {
        animation: String,
        main_frame_timestamp: Option<Float>,
    },
    Static {
        photo: String,
    },
}

impl TryFrom<InputProfilePhotoAnimated> for Form {
    type Error = InputProfilePhotoError;

    fn try_from(value: InputProfilePhotoAnimated) -> Result<Self, Self::Error> {
        let mut result = Self::default();
        let file = match value.animation {
            InputFile::Url(value) | InputFile::Id(value) => value,
            input_file => {
                result.insert_field("tgbot_ipp_file", input_file);
                String::from("attach://tgbot_ipp_file")
            }
        };
        result.insert_field(
            "photo",
            serde_json::to_string(&RawInputProfilePhoto::Animated {
                animation: file,
                main_frame_timestamp: value.main_frame_timestamp,
            })
            .map_err(InputProfilePhotoError::Serialize)?,
        );
        Ok(result)
    }
}

impl TryFrom<InputProfilePhotoStatic> for Form {
    type Error = InputProfilePhotoError;

    fn try_from(value: InputProfilePhotoStatic) -> Result<Self, Self::Error> {
        let mut result = Self::default();
        let file = match value.photo {
            InputFile::Url(value) | InputFile::Id(value) => value,
            input_file => {
                result.insert_field("tgbot_ipp_file", input_file);
                String::from("attach://tgbot_ipp_file")
            }
        };
        result.insert_field(
            "photo",
            serde_json::to_string(&RawInputProfilePhoto::Static { photo: file })
                .map_err(InputProfilePhotoError::Serialize)?,
        );
        Ok(result)
    }
}

/// Represents an input profile photo error.
#[derive(Debug)]
pub enum InputProfilePhotoError {
    /// Can not serialize data.
    Serialize(JsonError),
}

impl fmt::Display for InputProfilePhotoError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Serialize(err) => write!(out, "can not serialize input profile photo: {err}"),
        }
    }
}

impl Error for InputProfilePhotoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Serialize(err) => err,
        })
    }
}

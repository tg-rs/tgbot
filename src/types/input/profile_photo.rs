use serde::Serialize;

use crate::{
    api::{Form, WriteForm},
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

impl WriteForm for InputProfilePhoto {
    type Output = InputProfilePhotoData;

    fn write(self, form: &mut Form) -> Self::Output {
        match self {
            Self::Animated(value) => value.write(form),
            Self::Static(value) => value.write(form),
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
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum InputProfilePhotoData {
    Animated {
        animation: String,
        main_frame_timestamp: Option<Float>,
    },
    Static {
        photo: String,
    },
}

impl WriteForm for InputProfilePhotoAnimated {
    type Output = InputProfilePhotoData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self {
            animation,
            main_frame_timestamp,
        } = self;
        let id = animation.write(form);
        InputProfilePhotoData::Animated {
            animation: id,
            main_frame_timestamp,
        }
    }
}

impl WriteForm for InputProfilePhotoStatic {
    type Output = InputProfilePhotoData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self { photo } = self;
        let id = photo.write(form);
        InputProfilePhotoData::Static { photo: id }
    }
}

use std::{error::Error, fmt};

use serde::Serialize;
use serde_json::Error as JsonError;

use crate::{
    api::Form,
    types::{Float, InputFile},
};

#[cfg(test)]
mod tests;

/// Describes the content of a story to post.
#[derive(derive_more::From)]
pub enum InputStoryContent {
    /// A photo to post as a story.
    Photo(InputStoryContentPhoto),
    /// A video to post as a story.
    Video(InputStoryContentVideo),
}

impl TryFrom<InputStoryContent> for Form {
    type Error = InputStoryContentError;

    fn try_from(value: InputStoryContent) -> Result<Self, Self::Error> {
        match value {
            InputStoryContent::Photo(value) => value.try_into(),
            InputStoryContent::Video(value) => value.try_into(),
        }
    }
}

/// Describes a photo to post as a story.
pub struct InputStoryContentPhoto {
    photo: InputFile,
}

impl InputStoryContentPhoto {
    /// Creates a new `InputStoryContentPhoto`.
    ///
    /// # Arguments
    ///
    /// * `photo` - The photo to post as a story;
    ///   the photo must be of the size 1080x1920 and must not exceed 10 MB;
    ///   the photo can't be reused and can only be uploaded as a new file
    pub fn new<T>(photo: T) -> Self
    where
        T: Into<InputFile>,
    {
        Self { photo: photo.into() }
    }
}

impl TryFrom<InputStoryContentPhoto> for Form {
    type Error = InputStoryContentError;

    fn try_from(value: InputStoryContentPhoto) -> Result<Self, Self::Error> {
        let mut result = Form::default();
        let photo = match value.photo {
            InputFile::Id(value) | InputFile::Url(value) => value,
            input_file => {
                result.insert_field("tgbot_isc_file", input_file);
                String::from("attach://tgbot_isc_file")
            }
        };
        let info = RawInputStoryContent::Photo { photo };
        result.insert_field(
            "story",
            serde_json::to_string(&info).map_err(InputStoryContentError::Serialize)?,
        );
        Ok(result)
    }
}

/// Describes a video to post as a story.
pub struct InputStoryContentVideo {
    video: InputFile,
    cover_frame_timestamp: Option<Float>,
    duration: Option<Float>,
    is_animation: Option<bool>,
}

impl InputStoryContentVideo {
    /// Creates a new `InputStoryContentVideo`.
    ///
    /// # Arguments
    ///
    /// * `video` - The video to post as a story;
    ///   the video must be of the size 720x1280, streamable, encoded with H.265 codec,
    ///   with key frames added each second in the MPEG4 format, and must not exceed 30 MB;
    ///   the video can't be reused and can only be uploaded as a new file.
    pub fn new<T>(video: T) -> Self
    where
        T: Into<InputFile>,
    {
        Self {
            video: video.into(),
            cover_frame_timestamp: None,
            duration: None,
            is_animation: None,
        }
    }

    /// Sets a new cover frame timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - Timestamp in seconds of the frame
    ///   that will be used as the static cover for the story;
    ///   defaults to 0.0.
    pub fn with_cover_frame_timestamp(mut self, value: Float) -> Self {
        self.cover_frame_timestamp = Some(value);
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Precise duration of the video in seconds; 0-60.
    pub fn with_duration(mut self, value: Float) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets a new value for the `is_animation` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the video has no sound.
    pub fn with_is_animation(mut self, value: bool) -> Self {
        self.is_animation = Some(value);
        self
    }
}

impl TryFrom<InputStoryContentVideo> for Form {
    type Error = InputStoryContentError;

    fn try_from(value: InputStoryContentVideo) -> Result<Self, Self::Error> {
        let mut result = Form::default();
        let video = match value.video {
            InputFile::Id(value) | InputFile::Url(value) => value,
            input_file => {
                result.insert_field("tgbot_isc_file", input_file);
                String::from("attach://tgbot_isc_file")
            }
        };
        let info = RawInputStoryContent::Video {
            video,
            cover_frame_timestamp: value.cover_frame_timestamp,
            duration: value.duration,
            is_animation: value.is_animation,
        };
        result.insert_field(
            "story",
            serde_json::to_string(&info).map_err(InputStoryContentError::Serialize)?,
        );
        Ok(result)
    }
}

/// Represents an input story content error.
#[derive(Debug)]
pub enum InputStoryContentError {
    /// Can not serialize the content to JSON.
    Serialize(JsonError),
}

impl fmt::Display for InputStoryContentError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Serialize(err) => write!(out, "can not serialize: {}", err),
        }
    }
}

impl Error for InputStoryContentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Serialize(err) => err,
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawInputStoryContent {
    Photo {
        photo: String,
    },
    Video {
        video: String,
        cover_frame_timestamp: Option<Float>,
        duration: Option<Float>,
        is_animation: Option<bool>,
    },
}

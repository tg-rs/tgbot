use serde::Serialize;

use crate::{
    api::{Form, WriteForm},
    types::{Float, InputFile},
};

/// Describes the content of a story to post.
#[derive(derive_more::From)]
pub enum InputStoryContent {
    /// A photo to post as a story.
    Photo(InputStoryContentPhoto),
    /// A video to post as a story.
    Video(InputStoryContentVideo),
}

impl WriteForm for InputStoryContent {
    type Output = InputStoryContentData;

    fn write(self, form: &mut Form) -> Self::Output {
        match self {
            Self::Photo(value) => value.write(form),
            Self::Video(value) => value.write(form),
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

impl WriteForm for InputStoryContentPhoto {
    type Output = InputStoryContentData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self { photo } = self;
        let id = photo.write(form);
        InputStoryContentData::Photo { photo: id }
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

impl WriteForm for InputStoryContentVideo {
    type Output = InputStoryContentData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self {
            video,
            cover_frame_timestamp,
            duration,
            is_animation,
        } = self;
        let id = video.write(form);
        InputStoryContentData::Video {
            video: id,
            cover_frame_timestamp,
            duration,
            is_animation,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum InputStoryContentData {
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

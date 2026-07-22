use std::{error::Error, fmt};

use serde::Serialize;

use crate::{
    api::{Form, WriteForm},
    types::{InputFile, Integer},
};

const MIN_INPUT_GROUP_ITEMS: usize = 1;
const MAX_INPUT_GROUP_ITEMS: usize = 10;

/// Describes the paid media group to be sent.
#[derive(Debug)]
pub struct InputPaidMediaGroup {
    items: Vec<InputPaidMedia>,
}

impl InputPaidMediaGroup {
    /// Creates a new `InputPaidMediaGroup`.
    ///
    /// # Arguments
    ///
    /// * `items` - Items of the group.
    pub fn new<A, B>(items: A) -> Result<Self, InputPaidMediaGroupError>
    where
        A: IntoIterator<Item = B>,
        B: Into<InputPaidMedia>,
    {
        let items: Vec<InputPaidMedia> = items.into_iter().map(Into::into).collect();

        let total_items = items.len();
        if total_items < MIN_INPUT_GROUP_ITEMS {
            return Err(InputPaidMediaGroupError::NotEnoughItems(MIN_INPUT_GROUP_ITEMS));
        }
        if total_items > MAX_INPUT_GROUP_ITEMS {
            return Err(InputPaidMediaGroupError::TooManyItems(MAX_INPUT_GROUP_ITEMS));
        }
        Ok(Self { items })
    }
}

impl WriteForm for InputPaidMediaGroup {
    type Output = Vec<InputPaidMediaData>;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self { items } = self;
        items.into_iter().map(|x| x.write(form)).collect()
    }
}

/// Describes an [`crate::types::InputPaidMediaGroup`] error.
#[derive(Debug)]
pub enum InputPaidMediaGroupError {
    /// Group contains not enough items.
    NotEnoughItems(usize),
    /// Group contains too many items.
    TooManyItems(usize),
}

impl Error for InputPaidMediaGroupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotEnoughItems(_) => None,
            Self::TooManyItems(_) => None,
        }
    }
}

impl fmt::Display for InputPaidMediaGroupError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotEnoughItems(number) => write!(out, "group must contain at least {number} items"),
            Self::TooManyItems(number) => write!(out, "group must contain no more than {number} items"),
        }
    }
}

/// Describes the paid media to be sent.
#[derive(Debug)]
pub struct InputPaidMedia {
    parameters: InputPaidMediaParameters,
    media: InputFile,
    cover: Option<InputFile>,
    photo: Option<InputFile>,
    thumbnail: Option<InputFile>,
}

impl WriteForm for InputPaidMedia {
    type Output = InputPaidMediaData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self {
            parameters,
            media,
            cover,
            photo,
            thumbnail,
        } = self;
        let media = media.write(form);
        let cover = cover.map(|x| x.write(form));
        let photo = photo.map(|x| x.write(form));
        let thumbnail = thumbnail.map(|x| x.write(form));
        match parameters {
            InputPaidMediaParameters::LivePhoto => InputPaidMediaData::LivePhoto { media, photo },
            InputPaidMediaParameters::Photo => InputPaidMediaData::Photo { media },
            InputPaidMediaParameters::Video(parameters) => InputPaidMediaData::Video {
                media,
                cover,
                thumbnail,
                parameters,
            },
        }
    }
}

impl InputPaidMedia {
    fn new(parameters: InputPaidMediaParameters, media: InputFile) -> Self {
        Self {
            parameters,
            media,
            cover: None,
            photo: None,
            thumbnail: None,
        }
    }

    fn with_cover(mut self, value: Option<InputFile>) -> Self {
        self.cover = value;
        self
    }

    fn with_photo(mut self, value: InputFile) -> Self {
        self.photo = Some(value);
        self
    }

    fn with_thumbnail(mut self, value: Option<InputFile>) -> Self {
        self.thumbnail = value;
        self
    }
}

impl From<InputPaidMediaLivePhoto> for InputPaidMedia {
    fn from(value: InputPaidMediaLivePhoto) -> Self {
        Self::new(InputPaidMediaParameters::LivePhoto, value.media).with_photo(value.photo)
    }
}

impl From<InputPaidMediaPhoto> for InputPaidMedia {
    fn from(value: InputPaidMediaPhoto) -> Self {
        Self::new(InputPaidMediaParameters::Photo, value.media)
    }
}

impl From<InputPaidMediaVideo> for InputPaidMedia {
    fn from(value: InputPaidMediaVideo) -> Self {
        Self::new(InputPaidMediaParameters::Video(value.parameters), value.media)
            .with_cover(value.cover)
            .with_thumbnail(value.thumbnail)
    }
}

/// The paid media to send is a live photo.
#[derive(Debug)]
pub struct InputPaidMediaLivePhoto {
    media: InputFile,
    photo: InputFile,
}

impl<A, B> From<(A, B)> for InputPaidMediaLivePhoto
where
    A: Into<InputFile>,
    B: Into<InputFile>,
{
    fn from((media, photo): (A, B)) -> Self {
        Self {
            media: media.into(),
            photo: photo.into(),
        }
    }
}

/// The paid media to send is a photo.
#[derive(Debug)]
pub struct InputPaidMediaPhoto {
    media: InputFile,
}

impl<T> From<T> for InputPaidMediaPhoto
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self { media: value.into() }
    }
}

/// The paid media to send is a video.
#[derive(Debug)]
pub struct InputPaidMediaVideo {
    media: InputFile,
    cover: Option<InputFile>,
    thumbnail: Option<InputFile>,
    parameters: InputPaidMediaVideoParameters,
}

impl<T> From<T> for InputPaidMediaVideo
where
    T: Into<InputFile>,
{
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            cover: None,
            thumbnail: None,
            parameters: Default::default(),
        }
    }
}

impl InputPaidMediaVideo {
    /// Sets a new cover.
    ///
    /// # Arguments
    ///
    /// * `value` - Cover for the video in the message.
    pub fn with_cover<T>(mut self, file: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.cover = Some(file.into());
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// `value` - Video duration in seconds.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.parameters.duration = Some(value);
        self
    }

    /// Sets a new height.
    ///
    /// # Arguments
    ///
    /// `value` - Video height.
    pub fn with_height(mut self, value: Integer) -> Self {
        self.parameters.height = Some(value);
        self
    }

    /// Sets a new start timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - Start timestamp for the video in the message.
    pub fn with_start_timestamp(mut self, value: Integer) -> Self {
        self.parameters.start_timestamp = Some(value);
        self
    }

    /// Sets a new value for the `supports_streaming` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether the uploaded video is suitable for streaming.
    pub fn with_supports_streaming(mut self, value: bool) -> Self {
        self.parameters.supports_streaming = Some(value);
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail of the file sent;
    ///   can be ignored if thumbnail generation for the file is supported server-side.
    pub fn with_thumbnail<T>(mut self, file: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.thumbnail = Some(file.into());
        self
    }

    /// Sets a new width.
    ///
    /// # Arguments
    ///
    /// `value` - Video width.
    pub fn with_width(mut self, value: Integer) -> Self {
        self.parameters.width = Some(value);
        self
    }
}

#[derive(Debug)]
enum InputPaidMediaParameters {
    LivePhoto,
    Photo,
    Video(InputPaidMediaVideoParameters),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub(crate) enum InputPaidMediaData {
    LivePhoto {
        media: String,
        photo: Option<String>,
    },
    Photo {
        media: String,
    },
    Video {
        cover: Option<String>,
        media: String,
        thumbnail: Option<String>,
        #[serde(flatten)]
        parameters: InputPaidMediaVideoParameters,
    },
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub(crate) struct InputPaidMediaVideoParameters {
    duration: Option<Integer>,
    height: Option<Integer>,
    start_timestamp: Option<Integer>,
    supports_streaming: Option<bool>,
    width: Option<Integer>,
}

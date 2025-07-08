use std::{error::Error, fmt};

use serde::Serialize;
use serde_json::Error as JsonError;

use crate::{
    api::Form,
    types::{InputFile, Integer},
};

const MIN_INPUT_GROUP_ITEMS: usize = 1;
const MAX_INPUT_GROUP_ITEMS: usize = 10;

/// Describes the paid media group to be sent.
#[derive(Debug)]
pub struct InputPaidMediaGroup {
    form: Form,
}

impl InputPaidMediaGroup {
    /// Creates a new `InputPaidMediaGroup`.
    ///
    /// # Arguments
    ///
    /// * `items` - Items of the group.
    pub fn new<T>(items: T) -> Result<Self, InputPaidMediaGroupError>
    where
        T: IntoIterator<Item = InputPaidMediaGroupItem>,
    {
        let items: Vec<(usize, InputPaidMediaGroupItem)> = items.into_iter().enumerate().collect();

        let total_items = items.len();
        if total_items < MIN_INPUT_GROUP_ITEMS {
            return Err(InputPaidMediaGroupError::NotEnoughItems(MIN_INPUT_GROUP_ITEMS));
        }
        if total_items > MAX_INPUT_GROUP_ITEMS {
            return Err(InputPaidMediaGroupError::TooManyItems(MAX_INPUT_GROUP_ITEMS));
        }

        let mut form = Form::default();
        let mut add_file = |key: String, file: InputFile| -> String {
            match &file {
                InputFile::Id(text) | InputFile::Url(text) => text.clone(),
                _ => {
                    form.insert_field(&key, file);
                    format!("attach://{key}")
                }
            }
        };
        let mut info = Vec::new();
        for (idx, item) in items {
            let media = add_file(format!("tgbot_ipm_file_{idx}"), item.file);
            let thumbnail = item
                .thumbnail
                .map(|thumbnail| add_file(format!("tgbot_ipm_thumb_{idx}"), thumbnail));
            let data = match item.item_type {
                InputPaidMediaGroupItemType::Photo => InputPaidMediaGroupItemData::Photo { media },
                InputPaidMediaGroupItemType::Video(info) => InputPaidMediaGroupItemData::Video {
                    media,
                    cover: item
                        .cover
                        .map(|cover| add_file(format!("tgbot_ipm_cover_{idx}"), cover)),
                    thumbnail,
                    info,
                },
            };
            info.push(data);
        }

        form.insert_field(
            "media",
            serde_json::to_string(&info).map_err(InputPaidMediaGroupError::Serialize)?,
        );

        Ok(Self { form })
    }
}

impl From<InputPaidMediaGroup> for Form {
    fn from(value: InputPaidMediaGroup) -> Self {
        value.form
    }
}

/// Describes an [`crate::types::InputPaidMediaGroup`] error.
#[derive(Debug)]
pub enum InputPaidMediaGroupError {
    /// Group contains not enough items.
    NotEnoughItems(usize),
    /// Group contains too many items.
    TooManyItems(usize),
    /// Can not serialize items.
    Serialize(JsonError),
}

impl Error for InputPaidMediaGroupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InputPaidMediaGroupError::Serialize(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for InputPaidMediaGroupError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::InputPaidMediaGroupError::*;
        match self {
            NotEnoughItems(number) => write!(out, "group must contain at least {number} items"),
            TooManyItems(number) => write!(out, "group must contain no more than {number} items"),
            Serialize(err) => write!(out, "can not serialize group items: {err}"),
        }
    }
}

/// Represents an input paid media.
#[derive(Debug)]
pub struct InputPaidMediaGroupItem {
    file: InputFile,
    item_type: InputPaidMediaGroupItemType,
    cover: Option<InputFile>,
    thumbnail: Option<InputFile>,
}

impl InputPaidMediaGroupItem {
    /// Creates a `InputPaidMediaGroupItem` for a photo.
    ///
    /// # Arguments
    ///
    /// * `file` - File to attach.
    pub fn for_photo<T>(file: T) -> Self
    where
        T: Into<InputFile>,
    {
        Self::new(file, InputPaidMediaGroupItemType::Photo)
    }

    /// Creates a `InputPaidMediaGroupItem` for a video.
    ///
    /// # Arguments
    ///
    /// * `file` - File to attach.
    /// * `metadata` - Metadata.
    pub fn for_video<T>(file: T, metadata: InputPaidMediaVideo) -> Self
    where
        T: Into<InputFile>,
    {
        Self::new(file, InputPaidMediaGroupItemType::Video(metadata))
    }

    /// Sets a new cover.
    ///
    /// # Arguments
    ///
    /// * `value` - Cover.
    ///
    /// Note that the cover is ignored if the media type is not a video.
    pub fn with_cover<T>(mut self, file: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.cover = Some(file.into());
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail.
    ///
    /// Note that photo can not have thumbnail and it will be ignored.
    pub fn with_thumbnail<T>(mut self, file: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.thumbnail = Some(file.into());
        self
    }

    fn new<T>(file: T, item_type: InputPaidMediaGroupItemType) -> Self
    where
        T: Into<InputFile>,
    {
        Self {
            item_type,
            file: file.into(),
            cover: None,
            thumbnail: None,
        }
    }
}

#[derive(Debug)]
enum InputPaidMediaGroupItemType {
    Photo,
    Video(InputPaidMediaVideo),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum InputPaidMediaGroupItemData {
    Photo {
        media: String,
    },
    Video {
        media: String,
        cover: Option<String>,
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputPaidMediaVideo,
    },
}

/// The paid media to send is a video.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct InputPaidMediaVideo {
    duration: Option<Integer>,
    height: Option<Integer>,
    start_timestamp: Option<Integer>,
    supports_streaming: Option<bool>,
    width: Option<Integer>,
}

impl InputPaidMediaVideo {
    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// `value` - Video duration in seconds.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets a new height.
    ///
    /// # Arguments
    ///
    /// `value` - Video height.
    pub fn with_height(mut self, value: Integer) -> Self {
        self.height = Some(value);
        self
    }

    /// Sets a new start timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - Start timestamp for the video in the message.
    pub fn with_start_timestamp(mut self, value: Integer) -> Self {
        self.start_timestamp = Some(value);
        self
    }

    /// Sets a new value for the `supports_streaming` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether the uploaded video is suitable for streaming.
    pub fn with_supports_streaming(mut self, value: bool) -> Self {
        self.supports_streaming = Some(value);
        self
    }

    /// Sets a new width.
    ///
    /// # Arguments
    ///
    /// `value` - Video width.
    pub fn with_width(mut self, value: Integer) -> Self {
        self.width = Some(value);
        self
    }
}

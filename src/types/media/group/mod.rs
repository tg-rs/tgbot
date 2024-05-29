use std::{error::Error, fmt};

use serde::Serialize;
use serde_json::Error as JsonError;

use crate::{
    api::{Form, Method, Payload},
    types::{
        ChatId,
        InputFile,
        InputMediaAudio,
        InputMediaDocument,
        InputMediaPhoto,
        InputMediaVideo,
        Integer,
        Message,
        ReplyParameters,
        ReplyParametersError,
    },
};

#[cfg(test)]
mod tests;

const MIN_GROUP_ATTACHMENTS: usize = 2;
const MAX_GROUP_ATTACHMENTS: usize = 10;

/// Represents a group of input media to be sent.
#[derive(Debug)]
pub struct MediaGroup {
    form: Form,
}

impl MediaGroup {
    /// Creates a new `MediaGroup`.
    ///
    /// # Arguments
    ///
    /// * `items` - Items of the group.
    pub fn new<T>(items: T) -> Result<Self, MediaGroupError>
    where
        T: IntoIterator<Item = MediaGroupItem>,
    {
        let items: Vec<(usize, MediaGroupItem)> = items.into_iter().enumerate().collect();

        let total_items = items.len();
        if total_items < MIN_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::NotEnoughAttachments(MIN_GROUP_ATTACHMENTS));
        }
        if total_items > MAX_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::TooManyAttachments(MAX_GROUP_ATTACHMENTS));
        }

        let mut form = Form::default();

        let mut add_file = |key: String, file: InputFile| -> String {
            match &file {
                InputFile::Id(text) | InputFile::Url(text) => text.clone(),
                _ => {
                    form.insert_field(&key, file);
                    format!("attach://{}", key)
                }
            }
        };

        let mut info = Vec::new();
        for (idx, item) in items {
            let media = add_file(format!("tgbot_im_file_{}", idx), item.file);
            let thumbnail = item
                .thumbnail
                .map(|thumbnail| add_file(format!("tgbot_im_thumb_{}", idx), thumbnail));
            let data = match item.item_type {
                MediaGroupItemType::Audio(info) => MediaGroupItemData::Audio { media, thumbnail, info },
                MediaGroupItemType::Document(info) => MediaGroupItemData::Document { media, thumbnail, info },
                MediaGroupItemType::Photo(info) => MediaGroupItemData::Photo { media, info },
                MediaGroupItemType::Video(info) => MediaGroupItemData::Video { media, thumbnail, info },
            };
            info.push(data);
        }

        form.insert_field(
            "media",
            serde_json::to_string(&info).map_err(MediaGroupError::Serialize)?,
        );

        Ok(Self { form })
    }
}

impl From<MediaGroup> for Form {
    fn from(group: MediaGroup) -> Self {
        group.form
    }
}

/// Represents a media group item.
#[derive(Debug)]
pub struct MediaGroupItem {
    file: InputFile,
    item_type: MediaGroupItemType,
    thumbnail: Option<InputFile>,
}

impl MediaGroupItem {
    /// Creates a `MediaGroupItem` for an audio.
    ///
    /// # Arguments
    ///
    /// * `file` - File to attach.
    /// * `metadata` - Metadata.
    pub fn for_audio<T>(file: T, metadata: InputMediaAudio) -> Self
    where
        T: Into<InputFile>,
    {
        Self::new(file, MediaGroupItemType::Audio(metadata))
    }

    /// Creates a `MediaGroupItem` for a document.
    ///
    /// # Arguments
    ///
    /// * `file` - File to attach.
    /// * `metadata` - Metadata.
    pub fn for_document<T>(file: T, metadata: InputMediaDocument) -> Self
    where
        T: Into<InputFile>,
    {
        Self::new(file, MediaGroupItemType::Document(metadata))
    }

    /// Creates a `MediaGroupItem` for a photo.
    ///
    /// # Arguments
    ///
    /// * `file` - File to attach.
    /// * `metadata` - Metadata.
    pub fn for_photo<T>(file: T, metadata: InputMediaPhoto) -> Self
    where
        T: Into<InputFile>,
    {
        Self::new(file, MediaGroupItemType::Photo(metadata))
    }

    /// Creates a `MediaGroupItem` for a video.
    ///
    /// # Arguments
    ///
    /// * `file` - File to attach.
    /// * `metadata` - Metadata.
    pub fn for_video<T>(file: T, metadata: InputMediaVideo) -> Self
    where
        T: Into<InputFile>,
    {
        Self::new(file, MediaGroupItemType::Video(metadata))
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

    fn new<T>(file: T, item_type: MediaGroupItemType) -> Self
    where
        T: Into<InputFile>,
    {
        Self {
            item_type,
            file: file.into(),
            thumbnail: None,
        }
    }
}

#[derive(Debug)]
enum MediaGroupItemType {
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum MediaGroupItemData {
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

/// Represents a media group error.
#[derive(Debug)]
pub enum MediaGroupError {
    /// Media group contains not enough files.
    NotEnoughAttachments(usize),
    /// Media group contains too many files.
    TooManyAttachments(usize),
    /// Can not serialize items.
    Serialize(JsonError),
}

impl Error for MediaGroupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MediaGroupError::Serialize(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for MediaGroupError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MediaGroupError::NotEnoughAttachments(number) => {
                write!(out, "media group must contain at least {} attachments", number)
            }
            MediaGroupError::TooManyAttachments(number) => {
                write!(out, "media group must contain no more than {} attachments", number)
            }
            MediaGroupError::Serialize(err) => write!(out, "can not serialize media group items: {}", err),
        }
    }
}

/// Sends a group of photos or videos as an album.
#[derive(Debug)]
pub struct SendMediaGroup {
    form: Form,
}

impl SendMediaGroup {
    /// Creates a new `SendMediaGroup`.
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `media` - Photos and videos to be sent; 2–10 items.
    pub fn new<T>(chat_id: T, media: MediaGroup) -> Self
    where
        T: Into<ChatId>,
    {
        let mut form: Form = media.into();
        form.insert_field("chat_id", chat_id.into());
        Self { form }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("business_connection_id", value.into());
        self
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Sets a new message effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect to be added to the message; for private chats only.
    pub fn with_message_effect_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("message_effect_id", value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value);
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Result<Self, ReplyParametersError> {
        self.form.insert_field("reply_parameters", value.serialize()?);
        Ok(self)
    }
}

impl Method for SendMediaGroup {
    type Response = Vec<Message>;

    fn into_payload(self) -> Payload {
        Payload::form("sendMediaGroup", self.form)
    }
}

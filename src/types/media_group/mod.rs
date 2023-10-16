use std::{error::Error, fmt};

use serde::Serialize;
use serde_json::Error as JsonError;

use crate::{
    api::{Form, Method, Payload},
    types::{
        ChatId,
        InputFile,
        InputFileKind,
        InputMediaAudio,
        InputMediaDocument,
        InputMediaPhoto,
        InputMediaVideo,
        Integer,
        Message,
    },
};

#[cfg(test)]
mod tests;

const MIN_GROUP_ATTACHMENTS: usize = 2;
const MAX_GROUP_ATTACHMENTS: usize = 10;

/// Group of input media to be sent
#[derive(Debug)]
pub struct MediaGroup {
    form: Form,
}

impl MediaGroup {
    /// Creates a new group
    ///
    /// # Arguments
    ///
    /// * items - Items of the group
    pub fn new<I>(items: I) -> Result<Self, MediaGroupError>
    where
        I: IntoIterator<Item = MediaGroupItem>,
    {
        let items: Vec<(usize, MediaGroupItem)> = items.into_iter().enumerate().collect();

        let total_items = items.len();
        if total_items < MIN_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::NotEnoughAttachments(MIN_GROUP_ATTACHMENTS));
        }
        if total_items > MAX_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::TooManyAttachments(MAX_GROUP_ATTACHMENTS));
        }

        let mut form = Form::new();

        let mut add_file = |key: String, file: InputFile| -> String {
            match &file.kind {
                InputFileKind::Id(text) | InputFileKind::Url(text) => text.clone(),
                _ => {
                    form.insert_field(&key, file);
                    format!("attach://{}", key)
                }
            }
        };

        let mut info = Vec::new();
        for (idx, item) in items {
            let media = add_file(format!("tgbot_im_file_{}", idx), item.file);
            let thumb = item
                .thumb
                .map(|thumb| add_file(format!("tgbot_im_thumb_{}", idx), thumb));
            let data = match item.kind {
                MediaGroupItemKind::Audio(info) => MediaGroupItemData::Audio { media, thumb, info },
                MediaGroupItemKind::Document(info) => MediaGroupItemData::Document { media, thumb, info },
                MediaGroupItemKind::Photo(info) => MediaGroupItemData::Photo { media, info },
                MediaGroupItemKind::Video(info) => MediaGroupItemData::Video { media, thumb, info },
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

/// A media group item
#[derive(Debug)]
pub struct MediaGroupItem {
    kind: MediaGroupItemKind,
    file: InputFile,
    thumb: Option<InputFile>,
}

impl MediaGroupItem {
    /// Creates an audio item
    ///
    /// # Arguments
    ///
    /// * file - File to attach
    /// * info - Item metadata
    pub fn audio<F>(file: F, info: InputMediaAudio) -> Self
    where
        F: Into<InputFile>,
    {
        Self::new(file, MediaGroupItemKind::Audio(info))
    }

    /// Creates a document item
    ///
    /// # Arguments
    ///
    /// * file - File to attach
    /// * info - Item metadata
    pub fn document<F>(file: F, info: InputMediaDocument) -> Self
    where
        F: Into<InputFile>,
    {
        Self::new(file, MediaGroupItemKind::Document(info))
    }

    /// Creates a photo item
    ///
    /// # Arguments
    ///
    /// * file - File to attach
    /// * info - Item metadata
    pub fn photo<F>(file: F, info: InputMediaPhoto) -> Self
    where
        F: Into<InputFile>,
    {
        Self::new(file, MediaGroupItemKind::Photo(info))
    }

    /// Creates a video item
    ///
    /// # Arguments
    ///
    /// * file - File to attach
    /// * info - Item metadata
    pub fn video<F>(file: F, info: InputMediaVideo) -> Self
    where
        F: Into<InputFile>,
    {
        Self::new(file, MediaGroupItemKind::Video(info))
    }

    /// Adds a thumbnail to the item
    ///
    /// # Arguments
    ///
    /// * file - Thumbnail file
    ///
    /// Note that photo can not have thumbnail and it will be ignored
    pub fn with_thumb<F>(mut self, file: F) -> Self
    where
        F: Into<InputFile>,
    {
        self.thumb = Some(file.into());
        self
    }

    fn new<F>(file: F, kind: MediaGroupItemKind) -> Self
    where
        F: Into<InputFile>,
    {
        Self {
            kind,
            file: file.into(),
            thumb: None,
        }
    }
}

#[derive(Debug)]
enum MediaGroupItemKind {
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
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaAudio,
    },
    Document {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
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
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaVideo,
    },
}

/// A media group error
#[derive(Debug)]
pub enum MediaGroupError {
    /// Media group contains not enough files
    NotEnoughAttachments(usize),
    /// Media group contains too many files
    TooManyAttachments(usize),
    /// Can not serialize items
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

/// Send a group of photos or videos as an album
#[derive(Debug)]
pub struct SendMediaGroup {
    form: Form,
}

impl SendMediaGroup {
    /// Creates a new SendMediaGroup with empty optional parameters
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * media - Photos and videos to be sent, must include 2–10 items
    pub fn new<C: Into<ChatId>>(chat_id: C, media: MediaGroup) -> Self {
        let mut form: Form = media.into();
        form.insert_field("chat_id", chat_id.into());
        SendMediaGroup { form }
    }

    /// Sends the messages silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Protects the contents of the sent messages from forwarding and saving
    pub fn protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value);
        self
    }

    /// If the messages are a reply, ID of the original message
    pub fn reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.insert_field("reply_to_message_id", value);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, value: bool) -> Self {
        self.form.insert_field("allow_sending_without_reply", value.to_string());
        self
    }

    /// Unique identifier for the target message thread (topic) of the forum;
    /// for forum supergroups only
    pub fn message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }
}

impl Method for SendMediaGroup {
    type Response = Vec<Message>;

    fn into_payload(self) -> Payload {
        Payload::form("sendMediaGroup", self.form)
    }
}

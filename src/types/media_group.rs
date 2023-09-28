use crate::{
    request::Form,
    types::{InputFile, InputFileKind, InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo},
};
use serde::Serialize;
use serde_json::Error as JsonError;
use std::{error::Error as StdError, fmt};

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

impl StdError for MediaGroupError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::InputFileReader;
    use std::{io::Cursor, iter::repeat_with};

    #[test]
    fn media_group() {
        let group: Form = MediaGroup::new(vec![
            MediaGroupItem::audio(InputFileReader::from(Cursor::new("test")), InputMediaAudio::default()),
            MediaGroupItem::document(
                InputFileReader::from(Cursor::new("test")),
                InputMediaDocument::default(),
            ),
            MediaGroupItem::photo(
                InputFileReader::from(Cursor::new("test")),
                InputMediaPhoto::default().caption("caption"),
            ),
            MediaGroupItem::video(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default()),
            MediaGroupItem::audio(InputFile::file_id("file-id"), InputMediaAudio::default())
                .with_thumb(InputFile::url("thumb-url")),
            MediaGroupItem::document(InputFile::file_id("file-id"), InputMediaDocument::default())
                .with_thumb(InputFile::url("thumb-url")),
            MediaGroupItem::video(InputFile::file_id("file-id"), InputMediaVideo::default())
                .with_thumb(InputFile::url("thumb-url")),
        ])
        .try_into()
        .unwrap();
        let media: &str = group.fields.get("media").unwrap().get_text().unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(media).unwrap(),
            serde_json::json!([
                {
                    "media": "attach://tgbot_im_file_0",
                    "type": "audio"
                },
                {
                    "media": "attach://tgbot_im_file_1",
                    "type": "document"
                },
                {
                    "media": "attach://tgbot_im_file_2",
                    "type": "photo",
                    "caption": "caption"
                },
                {
                    "media": "attach://tgbot_im_file_3",
                    "type": "video"
                },
                {
                    "media": "file-id",
                    "thumb": "thumb-url",
                    "type": "audio"
                },
                {
                    "media": "file-id",
                    "thumb": "thumb-url",
                    "type": "document"
                },
                {
                    "media": "file-id",
                    "thumb": "thumb-url",
                    "type": "video"
                }
            ])
        );
        assert!(group.fields.get("tgbot_im_file_0").is_some());
        assert!(group.fields.get("tgbot_im_file_1").is_some());
        assert!(group.fields.get("tgbot_im_file_2").is_some());
        assert!(group.fields.get("tgbot_im_file_3").is_some());

        let err = TryInto::<Form>::try_into(MediaGroup::default()).unwrap_err();
        assert_eq!(err.to_string(), "media group must contain at least 2 attachments");

        let group = MediaGroup::new(
            repeat_with(|| {
                MediaGroupItem::photo(InputFileReader::from(Cursor::new("test")), InputMediaPhoto::default())
            })
            .take(11)
            .collect::<Vec<_>>(),
        );
        let err = TryInto::<Form>::try_into(group).unwrap_err();
        assert_eq!(err.to_string(), "media group must contain no more than 10 attachments");
    }
}

use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::{
    api::{Form, Method, Payload},
    types::{
        ChatId,
        InputFile,
        Integer,
        Message,
        ParseMode,
        PhotoSize,
        ReplyMarkup,
        ReplyMarkupError,
        ReplyParameters,
        ReplyParametersError,
        TextEntities,
        TextEntity,
        TextEntityError,
        User,
        Video,
    },
};

#[cfg(test)]
mod tests;

/// Contains information about a paid media purchase.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PaidMediaPurchased {
    /// User who purchased the media.
    pub from: User,
    /// Bot-specified paid media payload.
    #[serde(rename = "paid_media_payload")]
    pub payload: String,
}

impl PaidMediaPurchased {
    /// Creates a new `PaidMediaPurchased`.
    ///
    /// # Arguments
    ///
    /// * `from` - User who purchased the media.
    /// * `payload` - Bot-specified paid media payload.
    pub fn new<T>(from: User, payload: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            from,
            payload: payload.into(),
        }
    }
}

/// Describes the paid media added to a message.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media.
    pub star_count: Integer,
    /// Information about the paid media.
    pub paid_media: Vec<PaidMedia>,
}

impl PaidMediaInfo {
    /// Creates a new `PaidMediaInfo`.
    ///
    /// # Arguments
    ///
    /// * `star_count` - The number of Telegram Stars that must be paid to buy access to the media.
    /// * `paid_media` - Information about the paid media.
    pub fn new<A, B>(star_count: Integer, paid_media: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<PaidMedia>,
    {
        Self {
            star_count,
            paid_media: paid_media.into_iter().map(Into::into).collect(),
        }
    }
}

/// Describes paid media.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(from = "RawPaidMedia", into = "RawPaidMedia")]
pub enum PaidMedia {
    /// The paid media is a photo.
    Photo(Vec<PhotoSize>),
    /// The paid media isn't available before the payment.
    Preview(PaidMediaPreview),
    /// The paid media is a video.
    Video(Video),
}

/// The paid media isn't available before the payment.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PaidMediaPreview {
    /// Duration of the media in seconds as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Media height as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Media width as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
}

impl PaidMediaPreview {
    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration of the media in seconds as defined by the sender.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets a new height.
    ///
    /// # Arguments
    ///
    /// * `value` - Height of the media in seconds as defined by the sender.
    pub fn with_height(mut self, value: Integer) -> Self {
        self.height = Some(value);
        self
    }

    /// Sets a new width.
    ///
    /// # Arguments
    ///
    /// * `value` - Width of the media in seconds as defined by the sender.
    pub fn with_width(mut self, value: Integer) -> Self {
        self.width = Some(value);
        self
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum RawPaidMedia {
    Photo {
        photo: Vec<PhotoSize>,
    },
    Preview {
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<Integer>,
    },
    Video {
        video: Video,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
struct RawPaidMediaPhoto {
    photo: Vec<PhotoSize>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
struct RawPaidMediaVideo {
    video: Video,
}

impl From<RawPaidMedia> for PaidMedia {
    fn from(value: RawPaidMedia) -> Self {
        match value {
            RawPaidMedia::Photo { photo } => Self::Photo(photo),
            RawPaidMedia::Preview {
                duration,
                height,
                width,
            } => Self::Preview(PaidMediaPreview {
                duration,
                height,
                width,
            }),
            RawPaidMedia::Video { video } => Self::Video(video),
        }
    }
}

impl From<PaidMedia> for RawPaidMedia {
    fn from(value: PaidMedia) -> Self {
        match value {
            PaidMedia::Photo(photo) => Self::Photo { photo },
            PaidMedia::Preview(PaidMediaPreview {
                duration,
                height,
                width,
            }) => Self::Preview {
                duration,
                height,
                width,
            },
            PaidMedia::Video(video) => Self::Video { video },
        }
    }
}

/// Send paid media to channel chats.
#[derive(Debug)]
pub struct SendPaidMedia {
    form: Form,
}

impl SendPaidMedia {
    /// Creates a new `SendPaidMedia`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `media` - An array describing the media to be sent
    /// * `star_count` - The number of Telegram Stars that must be paid to buy access to the media.
    pub fn new<T>(chat_id: T, media: InputPaidMediaGroup, star_count: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        let mut form: Form = media.into();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("star_count", star_count);
        Self { form }
    }

    /// Sets a new value for an `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///             for a fee of 0.1 Telegram Stars per message.
    ///             The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.form.insert_field("allow_paid_broadcast", value);
        self
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection
    ///             on behalf of which the message will be sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("business_connection_id", value.into());
        self
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// `value` - Media caption, 0-1024 characters after entities parsing.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("caption", value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// `value` - A list of special entities that appear in the caption, which can be specified instead of parse_mode.
    pub fn with_caption_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value = value.into_iter().collect::<TextEntities>().serialize()?;
        self.form.insert_field("caption_entities", value);
        self.form.remove_field("parse_mode");
        Ok(self)
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether to send the message silently.
    ///
    /// Users will receive a notification with no sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// `value` - Mode for parsing entities in the media caption.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.form.insert_field("parse_mode", value);
        self.form.remove_field("caption_entities");
        self
    }

    /// Sets a new payload.
    ///
    /// # Arguments
    ///
    /// * `value` - Bot-defined paid media payload;
    ///             0-128 bytes;
    ///             This will not be displayed to the user, use it for your internal processes.
    pub fn with_payload<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("payload", value.into());
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether to protect the contents of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value);
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Result<Self, ReplyParametersError> {
        let value = value.serialize()?;
        self.form.insert_field("reply_parameters", value);
        Ok(self)
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// `value` - Additional interface options.
    pub fn with_reply_markup<T>(mut self, value: T) -> Result<Self, ReplyMarkupError>
    where
        T: Into<ReplyMarkup>,
    {
        let value = value.into().serialize()?;
        self.form.insert_field("reply_markup", value);
        Ok(self)
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.form.insert_field("show_caption_above_media", value);
        self
    }
}

impl Method for SendPaidMedia {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendPaidMedia", self.form)
    }
}

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
                    format!("attach://{}", key)
                }
            }
        };
        let mut info = Vec::new();
        for (idx, item) in items {
            let media = add_file(format!("tgbot_ipm_file_{}", idx), item.file);
            let thumbnail = item
                .thumbnail
                .map(|thumbnail| add_file(format!("tgbot_ipm_thumb_{}", idx), thumbnail));
            let data = match item.item_type {
                InputPaidMediaGroupItemType::Photo => InputPaidMediaGroupItemData::Photo { media },
                InputPaidMediaGroupItemType::Video(info) => {
                    InputPaidMediaGroupItemData::Video { media, thumbnail, info }
                }
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
            NotEnoughItems(number) => write!(out, "group must contain at least {} items", number),
            TooManyItems(number) => write!(out, "group must contain no more than {} items", number),
            Serialize(err) => write!(out, "can not serialize group items: {}", err),
        }
    }
}

/// Represents an input paid media.
#[derive(Debug)]
pub struct InputPaidMediaGroupItem {
    file: InputFile,
    item_type: InputPaidMediaGroupItemType,
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
            thumbnail: None,
        }
    }
}

#[derive(Debug)]
enum InputPaidMediaGroupItemType {
    Photo,
    Video(InputPaidMediaVideo),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum InputPaidMediaGroupItemData {
    Photo {
        media: String,
    },
    Video {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(flatten)]
        info: InputPaidMediaVideo,
    },
}

/// The paid media to send is a video.
#[derive(Debug, Default, Serialize)]
pub struct InputPaidMediaVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

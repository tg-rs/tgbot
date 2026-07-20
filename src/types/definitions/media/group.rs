use std::{error::Error, fmt};

use serde_json::Error as JsonError;

use crate::{
    api::{Form, Method, Payload, PayloadError},
    types::{
        ChatId,
        InputMedia,
        InputMediaAudio,
        InputMediaDocument,
        InputMediaLivePhoto,
        InputMediaPhoto,
        InputMediaVideo,
        Integer,
        Message,
        ReplyParameters,
        ReplyParametersError,
    },
};

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
    pub fn new<A, B>(items: A) -> Result<Self, MediaGroupError>
    where
        A: IntoIterator<Item = B>,
        B: Into<MediaGroupItem>,
    {
        let items: Vec<(usize, MediaGroupItem)> = items.into_iter().map(Into::into).enumerate().collect();

        let total_items = items.len();
        if total_items < MIN_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::NotEnoughAttachments(MIN_GROUP_ATTACHMENTS));
        }
        if total_items > MAX_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::TooManyAttachments(MAX_GROUP_ATTACHMENTS));
        }

        let mut form = Form::default();
        let mut info = Vec::new();
        for (idx, item) in items {
            let (item_form, item_info) = item.data.into_parts(&[idx]);
            form.extend(item_form);
            info.push(item_info);
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
    data: InputMedia,
}

impl From<InputMediaAudio> for MediaGroupItem {
    fn from(value: InputMediaAudio) -> Self {
        Self { data: value.into() }
    }
}

impl From<InputMediaDocument> for MediaGroupItem {
    fn from(value: InputMediaDocument) -> Self {
        Self { data: value.into() }
    }
}

impl From<InputMediaLivePhoto> for MediaGroupItem {
    fn from(value: InputMediaLivePhoto) -> Self {
        Self { data: value.into() }
    }
}

impl From<InputMediaPhoto> for MediaGroupItem {
    fn from(value: InputMediaPhoto) -> Self {
        Self { data: value.into() }
    }
}

impl From<InputMediaVideo> for MediaGroupItem {
    fn from(value: InputMediaVideo) -> Self {
        Self { data: value.into() }
    }
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
                write!(out, "media group must contain at least {number} attachments")
            }
            MediaGroupError::TooManyAttachments(number) => {
                write!(out, "media group must contain no more than {number} attachments")
            }
            MediaGroupError::Serialize(err) => write!(out, "can not serialize media group items: {err}"),
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

    /// Sets a new value for the `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///   for a fee of 0.1 Telegram Stars per message.
    ///   The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.form.insert_field("allow_paid_broadcast", value);
        self
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

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.form.insert_field("direct_messages_topic_id", value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
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
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
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

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::form("sendMediaGroup", self.form)
    }
}

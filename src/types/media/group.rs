use std::{error::Error, fmt};

use serde::Serialize;

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{
        ChatId,
        InputMedia,
        InputMediaAudio,
        InputMediaData,
        InputMediaDocument,
        InputMediaLivePhoto,
        InputMediaPhoto,
        InputMediaVideo,
        Integer,
        Message,
        ReplyParameters,
    },
};

const MIN_GROUP_ATTACHMENTS: usize = 2;
const MAX_GROUP_ATTACHMENTS: usize = 10;

/// Represents a group of input media to be sent.
#[derive(Debug)]
pub struct MediaGroup {
    items: Vec<MediaGroupItem>,
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
        let items: Vec<MediaGroupItem> = items.into_iter().map(Into::into).collect();

        let total_items = items.len();
        if total_items < MIN_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::NotEnoughAttachments(MIN_GROUP_ATTACHMENTS));
        }
        if total_items > MAX_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::TooManyAttachments(MAX_GROUP_ATTACHMENTS));
        }

        Ok(Self { items })
    }
}

impl WriteForm for MediaGroup {
    type Output = Vec<InputMediaData>;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self { items } = self;
        items
            .into_iter()
            .map(|MediaGroupItem { data }| data.write(form))
            .collect()
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
}

impl Error for MediaGroupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotEnoughAttachments(_) => None,
            Self::TooManyAttachments(_) => None,
        }
    }
}

impl fmt::Display for MediaGroupError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotEnoughAttachments(number) => {
                write!(out, "media group must contain at least {number} attachment(s)")
            }
            Self::TooManyAttachments(number) => {
                write!(out, "media group must contain no more than {number} attachment(s)")
            }
        }
    }
}

/// Sends a group of photos or videos as an album.
#[derive(Debug)]
pub struct SendMediaGroup {
    media: MediaGroup,
    parameters: SendMediaGroupParameters,
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
        Self {
            media,
            parameters: SendMediaGroupParameters {
                chat_id: Some(chat_id.into()),
                ..Default::default()
            },
        }
    }

    /// Sets a new value for the `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///   for a fee of 0.1 Telegram Stars per message.
    ///   The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.parameters.allow_paid_broadcast = Some(value);
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
        self.parameters.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.parameters.direct_messages_topic_id = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.parameters.disable_notification = Some(value);
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
        self.parameters.message_effect_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.parameters.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.parameters.protect_content = Some(value);
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.parameters.reply_parameters = Some(value);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
struct SendMediaGroupParameters {
    chat_id: Option<ChatId>,
    media: Option<Vec<InputMediaData>>,
    allow_paid_broadcast: Option<bool>,
    business_connection_id: Option<String>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    message_effect_id: Option<String>,
    message_thread_id: Option<Integer>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
}

impl Method for SendMediaGroup {
    type Response = Vec<Message>;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self { media, mut parameters } = self;
        let mut form = Form::default();
        parameters.media = Some(media.write(&mut form));
        parameters.serialize(&mut form)?;
        Payload::form("sendMediaGroup", form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::InputFile;

    #[test]
    fn media_group_err() {
        let err = MediaGroup::new([InputMediaDocument::from(InputFile::file_id("test-err"))]).unwrap_err();
        assert!(err.source().is_none());
        assert_eq!(err.to_string(), "media group must contain at least 2 attachment(s)");

        let err = MediaGroup::new([
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
            InputMediaDocument::from(InputFile::file_id("test-err")),
        ])
        .unwrap_err();
        assert!(err.source().is_none());
        assert_eq!(
            err.to_string(),
            "media group must contain no more than 10 attachment(s)"
        );
    }
}

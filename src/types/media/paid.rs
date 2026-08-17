use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{
        ChatId,
        InputPaidMediaData,
        InputPaidMediaGroup,
        InputTextCaption,
        Integer,
        LivePhoto,
        Message,
        PhotoSize,
        ReplyMarkup,
        ReplyParameters,
        SuggestedPostParameters,
        User,
        Video,
    },
};

/// Contains information about a paid media purchase.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PaidMediaPurchased {
    /// User who purchased the media.
    pub from: User,
    /// Bot-specified paid media payload.
    #[serde(rename = "paid_media_payload")]
    pub payload: String,
}

/// Describes the paid media added to a message.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media.
    pub star_count: Integer,
    /// Information about the paid media.
    pub paid_media: Vec<PaidMedia>,
}

/// Describes paid media.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(from = "RawPaidMedia", into = "RawPaidMedia")]
pub enum PaidMedia {
    /// The paid media is a live photo.
    LivePhoto(LivePhoto),
    /// The paid media is a photo.
    Photo(Vec<PhotoSize>),
    /// The paid media isn't available before the payment.
    Preview(PaidMediaPreview),
    /// The paid media is a video.
    #[from(Video)]
    Video(Box<Video>),
}

/// The paid media isn't available before the payment.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PaidMediaPreview {
    /// Duration of the media in seconds as defined by the sender.
    pub duration: Option<Integer>,
    /// Media height as defined by the sender.
    pub height: Option<Integer>,
    /// Media width as defined by the sender.
    pub width: Option<Integer>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum RawPaidMedia {
    LivePhoto {
        live_photo: LivePhoto,
    },
    Photo {
        photo: Vec<PhotoSize>,
    },
    Preview {
        duration: Option<Integer>,
        height: Option<Integer>,
        width: Option<Integer>,
    },
    Video {
        video: Box<Video>,
    },
}

impl From<RawPaidMedia> for PaidMedia {
    fn from(value: RawPaidMedia) -> Self {
        match value {
            RawPaidMedia::LivePhoto { live_photo } => Self::LivePhoto(live_photo),
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
            PaidMedia::LivePhoto(live_photo) => Self::LivePhoto { live_photo },
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
    media: InputPaidMediaGroup,
    parameters: SendPaidMediaParameters,
}

impl SendPaidMedia {
    /// Creates a new `SendPaidMedia`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `media` - An array describing the media to be sent
    /// * `star_count` - The number of Telegram Stars that must be paid to buy access to the media; 1-25000.
    pub fn new<T>(chat_id: T, media: InputPaidMediaGroup, star_count: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            media,
            parameters: SendPaidMediaParameters {
                chat_id: Some(chat_id.into()),
                star_count: Some(star_count),
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
    /// * `value` - Unique identifier of the business connection
    ///   on behalf of which the message will be sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// `value` - Media caption, 0-1024 characters after entities parsing.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<InputTextCaption>,
    {
        self.parameters.caption = Some(value.into());
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
    /// `value` - Whether to send the message silently.
    ///
    /// Users will receive a notification with no sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.parameters.disable_notification = Some(value);
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

    /// Sets a new payload.
    ///
    /// # Arguments
    ///
    /// * `value` - Bot-defined paid media payload;
    ///   0-128 bytes;
    ///   This will not be displayed to the user, use it for your internal processes.
    pub fn with_payload<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.payload = Some(value.into());
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether to protect the contents of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.parameters.protect_content = Some(value);
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.parameters.reply_parameters = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// `value` - Additional interface options.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.parameters.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.parameters.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new suggested post parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - An object containing the parameters of the suggested post to send.
    ///
    /// For direct messages chats only.
    ///
    /// If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub fn with_suggested_post_parameters(mut self, value: SuggestedPostParameters) -> Self {
        self.parameters.suggested_post_parameters = Some(value);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
struct SendPaidMediaParameters {
    chat_id: Option<ChatId>,
    star_count: Option<Integer>,
    allow_paid_broadcast: Option<bool>,
    business_connection_id: Option<String>,
    #[serde(flatten)]
    caption: Option<InputTextCaption>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    media: Vec<InputPaidMediaData>,
    message_thread_id: Option<Integer>,
    payload: Option<String>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
    reply_markup: Option<ReplyMarkup>,
    show_caption_above_media: Option<bool>,
    suggested_post_parameters: Option<SuggestedPostParameters>,
}

impl Method for SendPaidMedia {
    type Response = Message;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self { media, mut parameters } = self;
        let mut form = Form::default();
        parameters.media = media.write(&mut form);
        parameters.serialize(&mut form)?;
        Payload::form("sendPaidMedia", form)
    }
}

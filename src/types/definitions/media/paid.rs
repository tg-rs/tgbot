use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload},
    types::{
        ChatId,
        InputPaidMediaGroup,
        Integer,
        Message,
        ParseMode,
        PhotoSize,
        ReplyMarkup,
        ReplyMarkupError,
        ReplyParameters,
        ReplyParametersError,
        SuggestedPostParameters,
        SuggestedPostParametersError,
        TextEntities,
        TextEntity,
        TextEntityError,
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
#[allow(clippy::large_enum_variant)]
pub enum PaidMedia {
    /// The paid media is a photo.
    Photo(Vec<PhotoSize>),
    /// The paid media isn't available before the payment.
    Preview(PaidMediaPreview),
    /// The paid media is a video.
    Video(Video),
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

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[allow(clippy::large_enum_variant)]
enum RawPaidMedia {
    Photo {
        photo: Vec<PhotoSize>,
    },
    Preview {
        duration: Option<Integer>,
        height: Option<Integer>,
        width: Option<Integer>,
    },
    Video {
        video: Video,
    },
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
    /// * `star_count` - The number of Telegram Stars that must be paid to buy access to the media; 1-25000.
    pub fn new<T>(chat_id: T, media: InputPaidMediaGroup, star_count: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        let mut form: Form = media.into();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("star_count", star_count);
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
    /// * `value` - Unique identifier of the business connection
    ///   on behalf of which the message will be sent.
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
    /// `value` - Whether to send the message silently.
    ///
    /// Users will receive a notification with no sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
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
    ///   0-128 bytes;
    ///   This will not be displayed to the user, use it for your internal processes.
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

    /// Sets a new suggested post parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - An object containing the parameters of the suggested post to send.
    ///
    /// For direct messages chats only.
    ///
    /// If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub fn with_suggested_post_parameters(
        mut self,
        value: &SuggestedPostParameters,
    ) -> Result<Self, SuggestedPostParametersError> {
        let value = serde_json::to_string(value).map_err(SuggestedPostParametersError::Serialize)?;
        self.form.insert_field("suggested_post_parameters", value);
        Ok(self)
    }
}

impl Method for SendPaidMedia {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendPaidMedia", self.form)
    }
}

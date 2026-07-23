use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{
        ChatId,
        InputFile,
        InputTextCaption,
        Integer,
        Message,
        ReplyMarkup,
        ReplyParameters,
        SuggestedPostParameters,
    },
};

/// Represents a size of a photo or a file / sticker thumbnail.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PhotoSize {
    /// Identifier of the file.
    ///
    /// Can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier of the file.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Height of the photo.
    pub height: Integer,
    /// Width of the photo.
    pub width: Integer,
    /// File size in bytes.
    pub file_size: Option<Integer>,
}

impl PhotoSize {
    /// Creates a new `PhotoSize`.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Identifier of the file.
    /// * `file_unique_id` - Unique identifier of the file.
    /// * `height` - Height of the photo.
    /// * `width` - Width of the photo.
    pub fn new<A, B>(file_id: A, file_unique_id: B, height: Integer, width: Integer) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            height,
            width,
            file_size: None,
        }
    }

    /// Sets a new size of the file.
    ///
    /// # Arguments
    ///
    /// * `value` - The size of the file in bytes.
    pub fn with_file_size(mut self, value: Integer) -> Self {
        self.file_size = Some(value);
        self
    }
}

/// Sends a photo.
#[derive(Debug)]
pub struct SendPhoto {
    photo: InputFile,
    parameters: SendPhotoParameters,
}

impl SendPhoto {
    /// Creates a new `SendPhoto`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `photo` - Photo to send.
    pub fn new<A, B>(chat_id: A, photo: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<InputFile>,
    {
        Self {
            photo: photo.into(),
            parameters: SendPhotoParameters {
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

    /// Sets a new callback query ID.
    ///
    /// # Arguments
    ///
    /// * `value` - For outgoing ephemeral messages,
    ///   identifier of the callback query
    ///   which triggered the message if any.
    pub fn with_callback_query_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.callback_query_id = Some(value.into());
        self
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    ///
    /// May also be used when resending documents by `file_id`.
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
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.parameters.disable_notification = Some(value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to cover with a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.parameters.has_spoiler = Some(value);
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

    /// Sets a new receiver user ID.
    ///
    /// # Arguments
    ///
    /// * `value` - For outgoing ephemeral messages,
    ///   unique identifier of the user who will receive the message;
    ///   for group and supergroup chats only.
    ///
    /// It is not guaranteed that the user will receive the message,
    /// especially if they are offline.
    pub fn with_receiver_user_id(mut self, value: Integer) -> Self {
        self.parameters.receiver_user_id = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.parameters.reply_markup = Some(value.into());
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
struct SendPhotoParameters {
    allow_paid_broadcast: Option<bool>,
    business_connection_id: Option<String>,
    callback_query_id: Option<String>,
    #[serde(flatten)]
    caption: Option<InputTextCaption>,
    chat_id: Option<ChatId>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    has_spoiler: Option<bool>,
    message_effect_id: Option<String>,
    message_thread_id: Option<Integer>,
    photo: Option<String>,
    protect_content: Option<bool>,
    receiver_user_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
    reply_parameters: Option<ReplyParameters>,
    show_caption_above_media: Option<bool>,
    suggested_post_parameters: Option<SuggestedPostParameters>,
}

impl Method for SendPhoto {
    type Response = Message;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self { photo, mut parameters } = self;
        let mut form = Form::default();
        parameters.photo = Some(photo.write(&mut form));
        parameters.serialize(&mut form)?;
        Payload::form("sendPhoto", form)
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{
        ChatId,
        InputFile,
        InputTextCaption,
        Integer,
        Message,
        PhotoSize,
        ReplyMarkup,
        ReplyParameters,
        SuggestedPostParameters,
    },
};

/// Represents a live photo.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct LivePhoto {
    /// Duration of the video in seconds as defined by the sender.
    duration: Integer,
    /// Identifier for the video file which can be used to download or reuse the file.
    file_id: String,
    /// Unique identifier for the video file which is supposed to be the same over time and for different bots.
    ///
    /// Can't be used to download or reuse the file.
    file_unique_id: String,
    /// Video height as defined by the sender.
    height: Integer,
    /// Video width as defined by the sender.
    width: Integer,
    /// File size in bytes.
    file_size: Option<Integer>,
    /// MIME type of the file as defined by the sender.
    mime_type: Option<String>,
    /// Available sizes of the corresponding static photo.
    photo: Option<Vec<PhotoSize>>,
}

impl LivePhoto {
    /// Creates a new `LivePhoto`.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the video in seconds.
    /// * `file_id` - Identifier for the video file which can be used to download or reuse the file.
    /// * `file_unique_id` - Unique identifier for the video file which is supposed to be the same over time for different bots.
    /// * `height` - Video height.
    /// * `width` -> Video width.
    pub fn new<A, B>(duration: Integer, file_id: A, file_unique_id: B, height: Integer, width: Integer) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            duration,
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            height,
            width,
            file_size: None,
            mime_type: None,
            photo: None,
        }
    }

    /// Sets a new file size.
    ///
    /// # Arguments
    ///
    /// * `value` - File size in bytes.
    pub fn with_file_size(mut self, value: Integer) -> Self {
        self.file_size = Some(value);
        self
    }

    /// Sets a new MIME type.
    ///
    /// # Arguments
    ///
    /// * `value` - MIME type of the file as defined by the sender.
    pub fn with_mime_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.mime_type = Some(value.into());
        self
    }

    /// Sets a new photo.
    ///
    /// # Arguments
    ///
    /// * `value` - Available sizes of the corresponding static photo.
    pub fn with_photo<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PhotoSize>,
    {
        self.photo = Some(value.into_iter().collect());
        self
    }
}

/// Sends live photos.
#[derive(Debug)]
pub struct SendLivePhoto {
    live_photo: InputFile,
    photo: InputFile,
    parameters: SendLivePhotoParameters,
}

impl SendLivePhoto {
    /// Creates a new `SendLivePhoto`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `live_photo` -Live photo videi to send.
    /// * `photo` - Static photo to send.
    pub fn new<A, B, C>(chat_id: A, live_photo: B, photo: C) -> Self
    where
        A: Into<ChatId>,
        B: Into<InputFile>,
        C: Into<InputFile>,
    {
        Self {
            live_photo: live_photo.into(),
            photo: photo.into(),
            parameters: SendLivePhotoParameters {
                chat_id: Some(chat_id.into()),
                ..Default::default()
            },
        }
    }

    /// Sets a new value for the `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second;
    ///   ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message;
    ///   the relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.parameters.allow_paid_broadcast = Some(value);
        self
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the message will be sent.
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
    /// * `value` - Video caption (may also be used when resending videos by file_id);
    ///   0-1024 characters after entities parsing.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<InputTextCaption>,
    {
        self.parameters.caption = Some(value.into());
        self
    }

    /// Sets a new direct messages topic ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent;
    ///   required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.parameters.direct_messages_topic_id = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Sends the message silently;
    ///   users will receive a notification with no sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.parameters.disable_notification = Some(value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Pass True if the video needs to be covered with a spoiler animation
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.parameters.has_spoiler = Some(value);
        self
    }

    /// Sets a new message effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect to be added to the message;
    ///   for private chats only.
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
    /// * `value` - Unique identifier for the target message thread (topic) of a forum;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.parameters.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Protects the contents of the sent message from forwarding and saving.
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
    /// * `value` - Additional interface options.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.parameters.reply_markup = Some(value.into());
        self
    }

    /// Sets a new reply parameters.
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
    /// * `value` - The parameters of the suggested post to send;
    ///   for direct messages chats only;
    ///   if the message is sent as a reply to another suggested post,
    ///   then that suggested post is automatically declined.
    pub fn with_suggested_post_parameters(mut self, value: SuggestedPostParameters) -> Self {
        self.parameters.suggested_post_parameters = Some(value);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
struct SendLivePhotoParameters {
    allow_paid_broadcast: Option<bool>,
    business_connection_id: Option<String>,
    callback_query_id: Option<String>,
    #[serde(flatten)]
    caption: Option<InputTextCaption>,
    chat_id: Option<ChatId>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    has_spoiler: Option<bool>,
    live_photo: Option<String>,
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

impl Method for SendLivePhoto {
    type Response = Message;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self {
            live_photo,
            photo,
            mut parameters,
        } = self;
        let mut form = Form::default();
        parameters.live_photo = Some(live_photo.write(&mut form));
        parameters.photo = Some(photo.write(&mut form));
        parameters.serialize(&mut form)?;
        Payload::form("sendLivePhoto", form)
    }
}

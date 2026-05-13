use serde::{Deserialize, Serialize};

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
        SuggestedPostParameters,
        SuggestedPostParametersError,
        TextEntities,
        TextEntity,
        TextEntityError,
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
    form: Form,
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
        let chat_id = chat_id.into();
        let live_photo = live_photo.into();
        let photo = photo.into();
        Self {
            form: Form::from([
                ("chat_id", chat_id.into()),
                ("live_photo", live_photo.into()),
                ("photo", photo.into()),
            ]),
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
        self.form.insert_field("allow_paid_broadcast", value);
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
        self.form.insert_field("business_connection_id", value.into());
        self
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Video caption (may also be used when resending videos by file_id);
    ///    0-1024 characters after entities parsing.
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
    /// * `value` - A list of special entities that appear in the caption;
    ///   parse mode will be removed when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Result<Self, TextEntityError>
    where
        T: IntoIterator<Item = TextEntity>,
    {
        let value = TextEntities::from_iter(value);
        self.form.insert_field("caption_entities", value.serialize()?);
        self.form.remove_field("parse_mode");
        Ok(self)
    }

    /// Sets a new direct messages topic ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent;
    ///   required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.form.insert_field("direct_messages_topic_id", value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Sends the message silently;
    ///   users will receive a notification with no sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Pass True if the video needs to be covered with a spoiler animation
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.form.insert_field("has_spoiler", value);
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
        self.form.insert_field("message_effect_id", value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier for the target message thread (topic) of a forum;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the video caption.
    ///   Caption entities will be removed when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.form.insert_field("parse_mode", value);
        self.form.remove_field("caption_entities");
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Protects the contents of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Additional interface options.
    pub fn with_reply_markup<T>(mut self, value: T) -> Result<Self, ReplyMarkupError>
    where
        T: Into<ReplyMarkup>,
    {
        self.form.insert_field("reply_markup", value.into().serialize()?);
        Ok(self)
    }

    /// Sets a new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Result<Self, ReplyParametersError> {
        self.form.insert_field("reply_parameters", value.serialize()?);
        Ok(self)
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.form.insert_field("show_caption_above_media", value);
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
    pub fn with_suggested_post_parameters(
        mut self,
        value: &SuggestedPostParameters,
    ) -> Result<Self, SuggestedPostParametersError> {
        self.form.insert_field("suggested_post_parameters", value.serialize()?);
        Ok(self)
    }
}

impl Method for SendLivePhoto {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::form("sendLivePhoto", self.form)
    }
}

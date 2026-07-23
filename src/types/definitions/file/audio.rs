use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{
        ChatId,
        InputFile,
        InputFileReader,
        InputTextCaption,
        Integer,
        Message,
        PhotoSize,
        ReplyMarkup,
        ReplyParameters,
        SuggestedPostParameters,
    },
};

/// Represents an audio file to be treated as music by the Telegram clients.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Audio {
    /// Duration of the audio in seconds as defined by sender.
    pub duration: Integer,
    /// Identifier of the file.
    ///
    /// Can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier of the file.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Original filename as defined by sender.
    pub file_name: Option<String>,
    /// File size in bytes.
    pub file_size: Option<Integer>,
    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,
    /// Performer of the audio as defined by sender or by audio tags.
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags.
    pub title: Option<String>,
    /// Thumbnail of the album cover to which the music file belongs.
    pub thumbnail: Option<PhotoSize>,
}

impl Audio {
    /// Creates a new `Audio`.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds.
    /// * `file_id` - Identifier of the file.
    /// * `file_unique_id` - Unique identifier of the file.
    pub fn new<A, B>(duration: Integer, file_id: A, file_unique_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            duration,
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            file_name: None,
            file_size: None,
            mime_type: None,
            performer: None,
            title: None,
            thumbnail: None,
        }
    }

    /// Sets a new name of the file.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the file.
    pub fn with_file_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.file_name = Some(value.into());
        self
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

    /// Sets a new MIME type.
    ///
    /// # Arguments
    ///
    /// * `value` - MIME type.
    pub fn with_mime_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.mime_type = Some(value.into());
        self
    }

    /// Sets a new performer.
    ///
    /// # Arguments
    ///
    /// * `value` - Performer.
    pub fn with_performer<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.performer = Some(value.into());
        self
    }

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail.
    pub fn with_thumbnail(mut self, value: PhotoSize) -> Self {
        self.thumbnail = Some(value);
        self
    }
}

/// Sends am audio file.
///
/// Your audio must be in the `.MP3` or `.M4A` format.
/// Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
/// For sending voice messages, use the [`crate::types::SendVoice`] method instead.
#[derive(Debug)]
pub struct SendAudio {
    audio: InputFile,
    thumbnail: Option<InputFileReader>,
    parameters: SendAudioParameters,
}

impl SendAudio {
    /// Creates a new `SendAudio`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `audio` - Audio file to send.
    pub fn new<C, A>(chat_id: C, audio: A) -> Self
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        Self {
            audio: audio.into(),
            thumbnail: None,
            parameters: SendAudioParameters {
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

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration in seconds.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.parameters.duration = Some(value);
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

    /// Sets a new performer.
    ///
    /// # Arguments
    ///
    /// * `value` - Performer.
    pub fn with_performer<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.performer = Some(value.into());
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

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Track name.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.title = Some(value.into());
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail.
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 320.
    pub fn with_thumbnail_file<T>(mut self, value: T) -> Self
    where
        T: Into<InputFileReader>,
    {
        self.thumbnail = Some(value.into());
        self.parameters.thumbnail = None;
        self
    }

    /// Sets a new thumbnail.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail.
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 320.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.parameters.thumbnail = Some(value.into());
        self.thumbnail = None;
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
struct SendAudioParameters {
    allow_paid_broadcast: Option<bool>,
    audio: Option<String>,
    business_connection_id: Option<String>,
    callback_query_id: Option<String>,
    #[serde(flatten)]
    caption: Option<InputTextCaption>,
    chat_id: Option<ChatId>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    duration: Option<Integer>,
    message_effect_id: Option<String>,
    message_thread_id: Option<Integer>,
    performer: Option<String>,
    protect_content: Option<bool>,
    receiver_user_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
    reply_parameters: Option<ReplyParameters>,
    suggested_post_parameters: Option<SuggestedPostParameters>,
    thumbnail: Option<String>,
    title: Option<String>,
}

impl Method for SendAudio {
    type Response = Message;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self {
            audio,
            thumbnail,
            mut parameters,
        } = self;
        let mut form = Form::default();
        parameters.audio = Some(audio.write(&mut form));
        parameters.thumbnail = parameters.thumbnail.or_else(|| thumbnail.map(|x| x.write(&mut form)));
        parameters.serialize(&mut form)?;

        Payload::form("sendAudio", form)
    }
}

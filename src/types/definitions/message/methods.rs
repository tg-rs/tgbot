use serde::Serialize;

use crate::{
    api::{Form, Method, Payload},
    types::{
        ChatId,
        EditMessageResult,
        Float,
        InlineKeyboardError,
        InlineKeyboardMarkup,
        InputMedia,
        Integer,
        LinkPreviewOptions,
        Message,
        MessageId,
        ParseMode,
        ReplyMarkup,
        ReplyParameters,
        SuggestedPostParameters,
        TextEntities,
        TextEntity,
    },
};

/// Copies a message.
///
/// Service messages, paid media messages, giveaway messages, giveaway winners messages,
/// and invoice messages can't be copied.
/// A quiz poll can be copied only if the value of the field `correct_option_id` is known to the bot.
/// The method is analogous to the method [`ForwardMessage`],
/// but the copied message doesn't have a link to the original message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CopyMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: Integer,
    allow_paid_broadcast: Option<bool>,
    caption: Option<String>,
    caption_entities: Option<TextEntities>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    message_thread_id: Option<Integer>,
    parse_mode: Option<ParseMode>,
    protect_content: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
    reply_parameters: Option<ReplyParameters>,
    show_caption_above_media: Option<bool>,
    suggested_post_parameters: Option<SuggestedPostParameters>,
    video_start_timestamp: Option<Integer>,
}

impl CopyMessage {
    /// Creates a new `CopyMessage`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `from_chat_id` - Unique identifier of the chat where the original message was sent.
    /// * `message_id` - Message identifier in the chat specified in `from_chat_id`.
    pub fn new<A, B>(chat_id: A, from_chat_id: B, message_id: Integer) -> Self
    where
        A: Into<ChatId>,
        B: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            allow_paid_broadcast: None,
            caption: None,
            caption_entities: None,
            direct_messages_topic_id: None,
            disable_notification: None,
            message_thread_id: None,
            parse_mode: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
            show_caption_above_media: None,
            suggested_post_parameters: None,
            video_start_timestamp: None,
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
        self.allow_paid_broadcast = Some(value);
        self
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters after entities parsing.
    ///
    /// If not specified, the original caption is kept.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.direct_messages_topic_id = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
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
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.reply_parameters = Some(value);
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether the caption must be shown above the message media;
    ///   ignored if a new caption isn't specified.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.show_caption_above_media = Some(value);
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
        self.suggested_post_parameters = Some(value);
        self
    }

    /// Sets a new video start timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - New start timestamp for the copied video in the message.
    pub fn with_video_start_timestamp(mut self, value: Integer) -> Self {
        self.video_start_timestamp = Some(value);
        self
    }
}

impl Method for CopyMessage {
    type Response = MessageId;

    fn into_payload(self) -> Payload {
        Payload::json("copyMessage", self)
    }
}

/// Copies messages of any kind.
///
/// If some of the specified messages can't be found or copied, they are skipped.
/// Service messages, paid media messages, giveaway messages, giveaway winners messages,
/// and invoice messages can't be copied.
/// A quiz poll can be copied only if the value of the field `correct_option_id` is known to the bot.
/// The method is analogous to the method [`ForwardMessages`],
/// but the copied messages don't have a link to the original message.
/// Album grouping is kept for copied messages.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CopyMessages {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<Integer>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    message_thread_id: Option<Integer>,
    protect_content: Option<bool>,
    remove_caption: Option<bool>,
}

impl CopyMessages {
    /// Creates a new `CopyMessages`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `from_chat_id` - Unique identifier for the chat where the original messages were sent.
    /// * `message_ids` - Identifiers of 1-100 messages in the chat from_chat_id to copy;
    ///   the identifiers must be specified in a strictly increasing order.
    pub fn new<A, B, C>(chat_id: A, from_chat_id: B, message_ids: C) -> Self
    where
        A: Into<ChatId>,
        B: Into<ChatId>,
        C: IntoIterator<Item = Integer>,
    {
        Self {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_ids: message_ids.into_iter().collect(),
            direct_messages_topic_id: None,
            disable_notification: None,
            message_thread_id: None,
            protect_content: None,
            remove_caption: None,
        }
    }

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.direct_messages_topic_id = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets a new value for the `remove_caption` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to copy the messages without their captions.
    pub fn with_remove_caption(mut self, value: bool) -> Self {
        self.remove_caption = Some(value);
        self
    }
}

impl Method for CopyMessages {
    type Response = Vec<MessageId>;

    fn into_payload(self) -> Payload {
        Payload::json("copyMessages", self)
    }
}

/// Deletes a message.
///
/// Limitations:
///
/// - A message can only be deleted if it was sent less than 48 hours ago.
/// - Service messages about a supergroup, channel, or forum topic creation can't be deleted.
/// - A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.
/// - Bots can delete outgoing messages in private chats, groups, and supergroups.
/// - Bots can delete incoming messages in private chats.
/// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// - If the bot is an administrator of a group, it can delete any message there.
/// - If the bot has `can_delete_messages` permission in a supergroup or a channel,
///   it can delete any message there.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteMessage {
    chat_id: ChatId,
    message_id: Integer,
}

impl DeleteMessage {
    /// Creates a new `DeleteMessage`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the message to delete.
    pub fn new<T>(chat_id: T, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl Method for DeleteMessage {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteMessage", self)
    }
}

/// Deletes multiple messages simultaneously.
///
/// If some of the specified messages can't be found, they are skipped.
///
/// See [`DeleteMessage`] for limitations on which messages can be deleted.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteMessages {
    chat_id: ChatId,
    message_ids: Vec<Integer>,
}

impl DeleteMessages {
    /// Creates a new `DeleteMessages`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_ids` - Identifiers of 1-100 messages to delete.
    pub fn new<A, B>(chat_id: A, message_ids: B) -> Self
    where
        A: Into<ChatId>,
        B: IntoIterator<Item = Integer>,
    {
        Self {
            chat_id: chat_id.into(),
            message_ids: message_ids.into_iter().collect(),
        }
    }
}

impl Method for DeleteMessages {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteMessages", self)
    }
}

/// Changes a caption of a message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageCaption {
    business_connection_id: Option<String>,
    caption: Option<String>,
    caption_entities: Option<TextEntities>,
    chat_id: Option<ChatId>,
    inline_message_id: Option<String>,
    message_id: Option<Integer>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
    show_caption_above_media: Option<bool>,
}

impl EditMessageCaption {
    /// Creates a new `EditMessageCaption` for a chat message.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the sent message.
    pub fn for_chat_message<T>(chat_id: T, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            business_connection_id: None,
            caption: None,
            caption_entities: None,
            chat_id: Some(chat_id.into()),
            inline_message_id: None,
            message_id: Some(message_id),
            parse_mode: None,
            reply_markup: None,
            show_caption_above_media: None,
        }
    }

    /// Creates a new `EditMessageCaption` for an inline message.
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message.
    pub fn for_inline_message<T>(inline_message_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: None,
            caption: None,
            caption_entities: None,
            chat_id: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            parse_mode: None,
            reply_markup: None,
            show_caption_above_media: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the message to be edited was sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether the caption must be shown above the message media;
    ///   supported only for animation, photo and video messages.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.show_caption_above_media = Some(value);
        self
    }
}

impl Method for EditMessageCaption {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageCaption", self)
    }
}

/// Changes a live location message.
///
/// A location can be edited until its `live_period` expires or editing
/// is explicitly disabled by a call to [`StopMessageLiveLocation`].
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageLiveLocation {
    latitude: Float,
    longitude: Float,
    business_connection_id: Option<String>,
    chat_id: Option<ChatId>,
    heading: Option<Integer>,
    horizontal_accuracy: Option<Float>,
    inline_message_id: Option<String>,
    live_period: Option<Integer>,
    message_id: Option<Integer>,
    proximity_alert_radius: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    /// Creates a new `EditMessageLiveLocation` for a chat message.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the sent message.
    /// * `latitude` - Latitude of new location.
    /// * `longitude` Longitude of new location.
    pub fn for_chat_message<T>(chat_id: T, message_id: Integer, latitude: Float, longitude: Float) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            latitude,
            longitude,
            business_connection_id: None,
            chat_id: Some(chat_id.into()),
            inline_message_id: None,
            live_period: None,
            heading: None,
            horizontal_accuracy: None,
            message_id: Some(message_id),
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    /// Creates a new `EditMessageLiveLocation` for an inline message.
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message.
    /// * `latitude` - Latitude of new location.
    /// * `longitude` - Longitude of new location.
    pub fn for_inline_message<T>(inline_message_id: T, latitude: Float, longitude: Float) -> Self
    where
        T: Into<String>,
    {
        Self {
            latitude,
            longitude,
            business_connection_id: None,
            chat_id: None,
            heading: None,
            horizontal_accuracy: None,
            inline_message_id: Some(inline_message_id.into()),
            live_period: None,
            message_id: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the message to be edited was sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new horizontal accuracy.
    ///
    /// # Arguments
    ///
    /// * `value` - A radius of uncertainty for the location; in meters; 0-1500.
    pub fn with_horizontal_accuracy(mut self, value: Float) -> Self {
        self.horizontal_accuracy = Some(value);
        self
    }

    /// Sets a new heading.
    ///
    /// # Arguments
    ///
    /// * `value` - A direction in which the user is moving; in degrees; 1-360.
    pub fn with_heading(mut self, value: Integer) -> Self {
        self.heading = Some(value);
        self
    }

    /// Sets a new live period
    ///
    /// # Arguments
    ///
    /// * `value` - New period in seconds during which the location can be updated,
    ///   starting from the message send date.
    ///   If 0x7FFFFFFF is specified, then the location can be updated forever.
    ///   Otherwise, the new value must not exceed the current live_period by more than a day,
    ///   and the live location expiration date must remain within the next 90 days.
    ///   If not specified, then live_period remains unchanged
    pub fn with_live_period(mut self, value: Integer) -> Self {
        self.live_period = Some(value);
        self
    }

    /// Sets a new proximity alert radius.
    ///
    /// # Arguments
    ///
    /// * `value` - A maximum distance for proximity alerts
    ///   about approaching another chat member; in meters; 1-100000.
    pub fn with_proximity_alert_radius(mut self, value: Integer) -> Self {
        self.proximity_alert_radius = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for EditMessageLiveLocation {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageLiveLocation", self)
    }
}

/// Changes animation, audio, document, photo, or video message.
///
/// If a message is part of a message album, then it can be edited only
/// to an audio for audio albums, only to a document for document albums
/// and to a photo or a video otherwise.
/// When an inline message is edited, a new file can't be uploaded;
/// use a previously uploaded file via its file_id or specify a URL.
#[derive(Debug)]
pub struct EditMessageMedia {
    form: Form,
}

impl EditMessageMedia {
    /// Creates a new `EditMessageMedia` for a chat message.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the sent message.
    /// * `media` - New media content of the message.
    pub fn for_chat_message<T>(chat_id: T, message_id: Integer, media: InputMedia) -> Self
    where
        T: Into<ChatId>,
    {
        let mut form: Form = media.into();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("message_id", message_id);
        Self { form }
    }

    /// Creates a new `EditMessageMedia` for an inline message.
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message.
    /// * `media` - New media content of the message.
    pub fn for_inline_message<T>(inline_message_id: T, media: InputMedia) -> Self
    where
        T: Into<String>,
    {
        let mut form: Form = media.into();
        form.insert_field("inline_message_id", inline_message_id.into());
        EditMessageMedia { form }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the message to be edited was sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert_field("business_connection_id", value.into());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Result<Self, InlineKeyboardError>
    where
        T: Into<InlineKeyboardMarkup>,
    {
        let reply_markup = value.into().serialize()?;
        self.form.insert_field("reply_markup", reply_markup);
        Ok(self)
    }
}

impl Method for EditMessageMedia {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::form("editMessageMedia", self.form)
    }
}

/// Changes the reply markup of a message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageReplyMarkup {
    business_connection_id: Option<String>,
    chat_id: Option<ChatId>,
    inline_message_id: Option<String>,
    message_id: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageReplyMarkup {
    /// Creates a new `EditMessageReplyMarkup` for a chat message.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the sent message.
    pub fn for_chat_message<T>(chat_id: T, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            business_connection_id: None,
            chat_id: Some(chat_id.into()),
            inline_message_id: None,
            message_id: Some(message_id),
            reply_markup: None,
        }
    }

    /// Creates a new `EditMessageReplyMarkup` for an inline message.
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message.
    pub fn for_inline_message<T>(inline_message_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: None,
            chat_id: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            reply_markup: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the message to be edited was sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for EditMessageReplyMarkup {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageReplyMarkup", self)
    }
}

/// Changes a text or a game message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageText {
    text: String,
    business_connection_id: Option<String>,
    chat_id: Option<ChatId>,
    entities: Option<TextEntities>,
    link_preview_options: Option<LinkPreviewOptions>,
    inline_message_id: Option<String>,
    message_id: Option<Integer>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    /// Creates a new `EditMessageText` for a chat message.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the sent message.
    /// * `text` - New text of the message.
    pub fn for_chat_message<A, B>(chat_id: A, message_id: Integer, text: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            text: text.into(),
            business_connection_id: None,
            chat_id: Some(chat_id.into()),
            link_preview_options: None,
            entities: None,
            inline_message_id: None,
            message_id: Some(message_id),
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// Creates a new `EditMessageText` for an inline message.
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message.
    /// * `text` - New text of the message.
    pub fn for_inline_message<A, B>(inline_message_id: A, text: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            text: text.into(),
            business_connection_id: None,
            chat_id: None,
            link_preview_options: None,
            entities: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the message to be edited was sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new list of entities
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the text.
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new link preview options.
    ///
    /// # Arguments
    ///
    /// * `value` - Link preview generation options for the message.
    pub fn with_link_preview_options(mut self, value: LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(value);
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.entities = None;
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for EditMessageText {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageText", self)
    }
}

/// Forwards a message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: Integer,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    message_thread_id: Option<Integer>,
    suggested_post_parameters: Option<SuggestedPostParameters>,
    video_start_timestamp: Option<Integer>,
}

impl ForwardMessage {
    /// Creates a new `ForwardMessage`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `from_chat_id` - Unique identifier for the chat where the original message was sent.
    /// * `message_id` - Message identifier in the chat specified in `from_chat_id`.
    pub fn new<A, B>(chat_id: A, from_chat_id: B, message_id: Integer) -> Self
    where
        A: Into<ChatId>,
        B: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            direct_messages_topic_id: None,
            disable_notification: None,
            protect_content: None,
            message_thread_id: None,
            suggested_post_parameters: None,
            video_start_timestamp: None,
        }
    }

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.direct_messages_topic_id = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
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
        self.suggested_post_parameters = Some(value);
        self
    }

    /// Sets a new video start timestamp.
    ///
    /// # Arguments
    ///
    /// * `value` - New start timestamp for the forwarded video in the message.
    pub fn with_video_start_timestamp(mut self, value: Integer) -> Self {
        self.video_start_timestamp = Some(value);
        self
    }
}

impl Method for ForwardMessage {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("forwardMessage", self)
    }
}

/// Forwards multiple messages.
///
/// If some of the specified messages can't be found or forwarded, they are skipped.
/// Service messages and messages with protected content can't be forwarded.
/// Album grouping is kept for forwarded messages.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessages {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<Integer>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    message_thread_id: Option<Integer>,
}

impl ForwardMessages {
    /// Creates a new `ForwardMessages`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `from_chat_id` - Unique identifier for the chat where the original message was sent.
    /// * `message_ids` - Identifiers of 1-100 messages in the chat `from_chat_id` to forward;
    ///   the identifiers must be specified in a strictly increasing order.
    pub fn new<A, B, C>(chat_id: A, from_chat_id: B, message_ids: C) -> Self
    where
        A: Into<ChatId>,
        B: Into<ChatId>,
        C: IntoIterator<Item = Integer>,
    {
        Self {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_ids: message_ids.into_iter().collect(),
            direct_messages_topic_id: None,
            disable_notification: None,
            protect_content: None,
            message_thread_id: None,
        }
    }

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.direct_messages_topic_id = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }
}

impl Method for ForwardMessages {
    type Response = Vec<MessageId>;

    fn into_payload(self) -> Payload {
        Payload::json("forwardMessages", self)
    }
}

/// Sends a text message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendMessage {
    chat_id: ChatId,
    text: String,
    allow_paid_broadcast: Option<bool>,
    business_connection_id: Option<String>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    entities: Option<TextEntities>,
    link_preview_options: Option<LinkPreviewOptions>,
    message_effect_id: Option<String>,
    message_thread_id: Option<Integer>,
    parse_mode: Option<ParseMode>,
    protect_content: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
    reply_parameters: Option<ReplyParameters>,
    suggested_post_parameters: Option<SuggestedPostParameters>,
}

impl SendMessage {
    /// Creates a new `SendMessage`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `text` - Text of the message to be sent.
    pub fn new<A, B>(chat_id: A, text: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            text: text.into(),
            allow_paid_broadcast: None,
            business_connection_id: None,
            direct_messages_topic_id: None,
            disable_notification: None,
            entities: None,
            link_preview_options: None,
            message_effect_id: None,
            message_thread_id: None,
            parse_mode: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
            suggested_post_parameters: None,
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
        self.allow_paid_broadcast = Some(value);
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
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.direct_messages_topic_id = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new list of entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the text.
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new link preview options.
    ///
    /// # Arguments
    ///
    /// * `value` - Link preview generation options for the message.
    pub fn with_link_preview_options(mut self, value: LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(value);
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
        self.message_effect_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.entities = None;
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
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
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.reply_parameters = Some(value);
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
        self.suggested_post_parameters = Some(value);
        self
    }
}

impl Method for SendMessage {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendMessage", self)
    }
}

/// Streams a partial message to a user while the message is being generated;
/// supported only for bots with forum topic mode enabled.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendMessageDraft {
    chat_id: Integer,
    draft_id: Integer,
    text: String,
    entities: Option<TextEntities>,
    message_thread_id: Option<Integer>,
    parse_mode: Option<ParseMode>,
}

impl SendMessageDraft {
    /// Creates a new `SendMessageDraft`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target private chat.
    /// * `draft_id` - Unique identifier of the message draft; must be non-zero.
    ///   Changes of drafts with the same identifier are animated.
    /// * `text` - Text of the message to be sent, 1-4096 characters after entities parsing.
    pub fn new<T>(chat_id: Integer, draft_id: Integer, text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            chat_id,
            draft_id,
            text: text.into(),
            entities: None,
            message_thread_id: None,
            parse_mode: None,
        }
    }

    /// Sets a new list of entities
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the text.
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier for the target message thread.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.entities = None;
        self.parse_mode = Some(value);
        self
    }
}

impl Method for SendMessageDraft {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("sendMessageDraft", self)
    }
}

/// Stops updating a live location message before `live_period` expires.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct StopMessageLiveLocation {
    business_connection_id: Option<String>,
    chat_id: Option<ChatId>,
    inline_message_id: Option<String>,
    message_id: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocation {
    /// Creates a new `StopMessageLiveLocation` for a chat message.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `message_id` - Identifier of the sent message.
    pub fn for_chat_message<T>(chat_id: T, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            business_connection_id: None,
            chat_id: Some(chat_id.into()),
            inline_message_id: None,
            message_id: Some(message_id),
            reply_markup: None,
        }
    }

    /// Creates a new `StopMessageLiveLocation` for an inline message.
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message.
    pub fn for_inline_message<T>(inline_message_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: None,
            chat_id: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            reply_markup: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection on behalf of which the message to be edited was sent.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for StopMessageLiveLocation {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("stopMessageLiveLocation", self)
    }
}

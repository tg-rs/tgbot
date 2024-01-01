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
        TextEntities,
        TextEntity,
    },
};

#[cfg(test)]
mod tests;

/// Copies a message.
///
/// Service messages and invoice messages can't be copied.
/// A quiz poll can be copied only if the value of the field `correct_option_id` is known to the bot.
/// The method is analogous to the method [`ForwardMessage`],
/// but the copied message doesn't have a link to the original message.
#[derive(Clone, Debug, Serialize)]
pub struct CopyMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
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
            caption: None,
            caption_entities: None,
            disable_notification: None,
            message_thread_id: None,
            parse_mode: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
        }
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

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
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
/// Service messages, giveaway messages, giveaway winners messages,
/// and invoice messages can't be copied.
/// A quiz poll can be copied only if the value of the field `correct_option_id` is known to the bot.
/// The method is analogous to the method [`ForwardMessages`],
/// but the copied messages don't have a link to the original message.
/// Album grouping is kept for copied messages.
#[derive(Clone, Debug, Serialize)]
pub struct CopyMessages {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    ///                   the identifiers must be specified in a strictly increasing order.
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
            disable_notification: None,
            message_thread_id: None,
            protect_content: None,
            remove_caption: None,
        }
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets a new value for a `remove_caption` flag.
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
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageCaption {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
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
            caption: None,
            caption_entities: None,
            chat_id: Some(chat_id.into()),
            inline_message_id: None,
            message_id: Some(message_id),
            parse_mode: None,
            reply_markup: None,
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
            caption: None,
            caption_entities: None,
            chat_id: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            parse_mode: None,
            reply_markup: None,
        }
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
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageLiveLocation {
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            chat_id: Some(chat_id.into()),
            inline_message_id: None,
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
            chat_id: None,
            heading: None,
            horizontal_accuracy: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
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

    /// Sets a new proximity alert radius.
    ///
    /// # Arguments
    ///
    /// * `value` - A maximum distance for proximity alerts
    ///             about approaching another chat member; in meters; 1-100000.
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
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageReplyMarkup {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            chat_id: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            reply_markup: None,
        }
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
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageText {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            chat_id: None,
            link_preview_options: None,
            entities: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            parse_mode: None,
            reply_markup: None,
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
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
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
            disable_notification: None,
            protect_content: None,
            message_thread_id: None,
        }
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
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
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessages {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    ///                   the identifiers must be specified in a strictly increasing order.
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
            disable_notification: None,
            protect_content: None,
            message_thread_id: None,
        }
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
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
#[derive(Clone, Debug, Serialize)]
pub struct SendMessage {
    chat_id: ChatId,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
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
            disable_notification: None,
            entities: None,
            link_preview_options: None,
            message_thread_id: None,
            parse_mode: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
        }
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
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

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
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

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
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
}

impl Method for SendMessage {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendMessage", self)
    }
}

/// Stops updating a live location message before `live_period` expires.
#[derive(Clone, Debug, Serialize)]
pub struct StopMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            chat_id: None,
            inline_message_id: Some(inline_message_id.into()),
            message_id: None,
            reply_markup: None,
        }
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

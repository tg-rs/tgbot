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
        Message,
        MessageId,
        ParseMode,
        ReplyMarkup,
        TextEntities,
        TextEntity,
    },
};

#[cfg(test)]
mod tests;

/// Copy messages of any kind
///
/// The method is analogous to the method forwardMessages,
/// but the copied message doesn't have a link to the original message
#[derive(Clone, Debug, Serialize)]
pub struct CopyMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl CopyMessage {
    /// Creates a new CopyMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * from_chat_id - Unique identifier for the chat where the original message was sent
    /// * message_id - Message identifier in the chat specified in from_chat_id
    pub fn new<T, F>(chat_id: T, from_chat_id: F, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
        F: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    /// New caption for media, 0-1024 characters after entities parsing
    ///
    /// If not specified, the original caption is kept
    pub fn caption<C: Into<String>>(mut self, caption: C) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the new caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities<T>(mut self, caption_entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(caption_entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Mode for parsing entities in the new caption
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.caption_entities = None;
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for CopyMessage {
    type Response = MessageId;

    fn into_payload(self) -> Payload {
        Payload::json("copyMessage", self)
    }
}

/// Delete a message, including service messages
///
/// Limitations:
///
/// * A message can only be deleted if it was sent less than 48 hours ago.
/// * Bots can delete outgoing messages in private chats, groups, and supergroups.
/// * Bots can delete incoming messages in private chats.
/// * Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// * If the bot is an administrator of a group, it can delete any message there.
/// * If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteMessage {
    chat_id: ChatId,
    message_id: Integer,
}

impl DeleteMessage {
    /// Creates a new DeleteMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the message to delete
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        DeleteMessage {
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

/// Edit caption of message sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageCaption {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageCaption {
    /// Creates a new EditMessageCaption
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        EditMessageCaption {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageCaption
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        EditMessageCaption {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// New caption of the message
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities<T>(mut self, caption_entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(caption_entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Parse mode
    ///
    /// Entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.caption_entities = None;
        self
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageCaption {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageCaption", self)
    }
}

/// Edit live location messages sent by the bot or via the bot (for inline bots)
///
/// A location can be edited until its live_period expires or editing
/// is explicitly disabled by a call to stopMessageLiveLocation
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    /// Creates a new EditMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * latitude - Latitude of new location
    /// * longitude Longitude of new location
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer, latitude: Float, longitude: Float) -> Self {
        EditMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * latitude - Latitude of new location
    /// * longitude Longitude of new location
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, latitude: Float, longitude: Float) -> Self {
        EditMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: Float) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    /// Direction in which the user is moving, in degrees
    ///
    /// Must be between 1 and 360 if specified
    pub fn heading(mut self, heading: Integer) -> Self {
        self.heading = Some(heading);
        self
    }

    /// Maximum distance for proximity alerts about approaching another chat member, in meters
    ///
    /// Must be between 1 and 100000 if specified
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: Integer) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageLiveLocation {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageLiveLocation", self)
    }
}

/// Edit audio, document, photo, or video messages
///
/// If a message is a part of a message album, then it can be edited only to a photo or a video
/// Otherwise, message type can be changed arbitrarily
/// When inline message is edited, new file can't be uploaded
/// Use previously uploaded file via its file_id or specify a URL
#[derive(Debug)]
pub struct EditMessageMedia {
    form: Form,
}

impl EditMessageMedia {
    /// Creates a new EditMessageMedia
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * media - New media content of the message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer, media: InputMedia) -> Self {
        let mut form: Form = media.into();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("message_id", message_id);
        EditMessageMedia { form }
    }

    /// Creates a new EditMessageMedia
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * media - New media content of the message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, media: InputMedia) -> Self {
        let mut form: Form = media.into();
        form.insert_field("inline_message_id", inline_message_id.into());
        EditMessageMedia { form }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Result<Self, InlineKeyboardError> {
        let reply_markup = reply_markup.into().serialize()?;
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

/// Edit only the reply markup of messages sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageReplyMarkup {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageReplyMarkup {
    /// Creates a new EditMessageReplyMarkup
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        EditMessageReplyMarkup {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageReplyMarkup
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        EditMessageReplyMarkup {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            reply_markup: None,
        }
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageReplyMarkup {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageReplyMarkup", self)
    }
}

/// Edit text and game messages sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    /// Creates a new EditMessageText
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * text - New text of the message
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, message_id: Integer, text: S) -> Self {
        EditMessageText {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageText
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * text - New text of the message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, text: S) -> Self {
        EditMessageText {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Parse mode
    ///
    /// Entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.entities = None;
        self
    }

    /// List of special entities that appear in message text
    ///
    /// Parse mode will be set to None when this method is called
    pub fn entities<T>(mut self, entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Disables link previews for links in this message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageText {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageText", self)
    }
}

/// Forward message of any kind
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
}

impl ForwardMessage {
    /// Creates a new ForwardMessage with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * from_chat_id - Unique identifier for the chat where the original message was sent
    /// * message_id - Message identifier in the chat specified in from_chat_id
    pub fn new<C: Into<ChatId>>(chat_id: C, from_chat_id: C, message_id: Integer) -> Self {
        ForwardMessage {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
            protect_content: None,
        }
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the forwarded message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
}

impl Method for ForwardMessage {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("forwardMessage", self)
    }
}

/// Send text messages
#[derive(Clone, Debug, Serialize)]
pub struct SendMessage {
    chat_id: ChatId,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    /// Creates a new SendMessage with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * text - Text of the message to be sent
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, text: S) -> Self {
        SendMessage {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    /// Sets parse mode
    ///
    /// Entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.entities = None;
        self
    }

    /// List of special entities that appear in message text
    ///
    /// Parse mode will be set to None when this method is called
    pub fn entities<T>(mut self, entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Disables link previews for links in this message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendMessage {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendMessage", self)
    }
}

/// Stop updating a live location message
/// sent by the bot or via the bot (for inline bots)
/// before live_period expires
#[derive(Clone, Debug, Serialize)]
pub struct StopMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocation {
    /// Creates a new StopMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        StopMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            reply_markup: None,
        }
    }

    /// Creates a new StopMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        StopMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            reply_markup: None,
        }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for StopMessageLiveLocation {
    type Response = EditMessageResult;

    fn into_payload(self) -> Payload {
        Payload::json("stopMessageLiveLocation", self)
    }
}

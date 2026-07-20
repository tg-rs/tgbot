use serde::Serialize;

use crate::{
    api::{Form, Method, Payload, PayloadError},
    types::{
        ChatId,
        InlineKeyboardMarkup,
        InputMedia,
        Integer,
        LinkPreviewOptions,
        ParseMode,
        SerializeError,
        TextEntities,
        TextEntity,
    },
};

/// Ephemeral message identity.
///
/// Consists of a chat ID, receiver user ID and ephemeral message ID in the chat.
#[derive(Clone, Debug, Serialize)]
pub struct EphemeralMessageIdentity {
    chat_id: ChatId,
    receiver_user_id: Integer,
    ephemeral_message_id: Integer,
}

impl<T> From<(T, Integer, Integer)> for EphemeralMessageIdentity
where
    T: Into<ChatId>,
{
    fn from((chat_id, receiver_user_id, ephemeral_message_id): (T, Integer, Integer)) -> Self {
        Self {
            chat_id: chat_id.into(),
            receiver_user_id,
            ephemeral_message_id,
        }
    }
}

/// Deletes an ephemeral message.
///
/// Note that it is not guaranteed that the user will receive the message deletion event, especially if they are offline.
#[derive(Clone, Debug)]
pub struct DeleteEphemeralMessage {
    identity: EphemeralMessageIdentity,
}

impl<T> From<T> for DeleteEphemeralMessage
where
    T: Into<EphemeralMessageIdentity>,
{
    fn from(value: T) -> Self {
        Self { identity: value.into() }
    }
}

impl Method for DeleteEphemeralMessage {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("deleteEphemeralMessage", self.identity)
    }
}

/// Edits the caption of an ephemeral message.
///
/// Note that it is not guaranteed that
/// the user will receive the message edit event,
/// especially if they are offline.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditEphemeralMessageCaption {
    #[serde(flatten)]
    identity: EphemeralMessageIdentity,
    caption: Option<String>,
    caption_entities: Option<TextEntities>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl<T> From<T> for EditEphemeralMessageCaption
where
    T: Into<EphemeralMessageIdentity>,
{
    fn from(value: T) -> Self {
        Self {
            identity: value.into(),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
        }
    }
}

impl EditEphemeralMessageCaption {
    /// Sets a new caption
    ///
    /// # Arguments
    ///
    /// * `value` - New caption of the message; 0-1024 characters after entities parsing.
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
    /// * `value` - A list of special entities.
    ///
    /// It can be specified instead of parse mode.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities.
    ///
    /// It can be specified instead of entities.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - An object for an inline keyboard
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for EditEphemeralMessageCaption {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("editEphemeralMessageCaption", self)
    }
}

/// Edits the media of an ephemeral message.
///
/// Note that it is not guaranteed that
/// the user will receive the message edit event,
/// especially if they are offline.
#[derive(Debug)]
pub struct EditEphemeralMessageMedia {
    form: Form,
}

impl EditEphemeralMessageMedia {
    /// Creates a new `EditEphemeralMessageMedia`.
    ///
    /// # Arguments
    ///
    /// * `identity` - Identity of the ephemeral message.
    /// * `media` - The new media content of the message.
    pub fn new<A, B>(identity: A, media: B) -> Result<Self, SerializeError>
    where
        A: Into<EphemeralMessageIdentity>,
        B: Into<InputMedia>,
    {
        let identity = identity.into();
        let (mut form, media_data) = media.into().into_parts(&[0]);
        form.insert_field("media", media_data.serialize()?);
        form.insert_field("chat_id", identity.chat_id);
        form.insert_field("receiver_user_id", identity.receiver_user_id);
        form.insert_field("ephemeral_message_id", identity.ephemeral_message_id);
        Ok(Self { form })
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - An object for an inline keyboard
    pub fn with_reply_markup<T>(mut self, value: T) -> Result<Self, SerializeError>
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.form.insert_field("reply_markup", value.into().serialize()?);
        Ok(self)
    }
}

impl Method for EditEphemeralMessageMedia {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::form("editEphemeralMessageMedia", self.form)
    }
}

/// Edits only the reply markup of an ephemeral message.
///
/// Note that it is not guaranteed that
/// the user will receive the message edit event,
/// especially if they are offline.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditEphemeralMessageReplyMarkup {
    #[serde(flatten)]
    identity: EphemeralMessageIdentity,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl<T> From<T> for EditEphemeralMessageReplyMarkup
where
    T: Into<EphemeralMessageIdentity>,
{
    fn from(value: T) -> Self {
        Self {
            identity: value.into(),
            reply_markup: None,
        }
    }
}

impl EditEphemeralMessageReplyMarkup {
    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - An object for an inline keyboard
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for EditEphemeralMessageReplyMarkup {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("editEphemeralMessageReplyMarkup", self)
    }
}

/// Edits an ephemeral text message.
///
/// Note that it is not guaranteed that the user will receive the message edit event,
/// especially if they are offline.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditEphemeralMessageText {
    #[serde(flatten)]
    identity: EphemeralMessageIdentity,
    text: String,
    entities: Option<TextEntities>,
    link_preview_options: Option<LinkPreviewOptions>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditEphemeralMessageText {
    /// Creates a new `EditEpehemeralMessageText`.
    ///
    /// # Arguments
    ///
    /// * `identity` - Identity of the ephemeral message.
    /// * `text` - New text of the message; 1-4096 characters after entity parsing.
    pub fn new<A, B>(identity: A, text: B) -> Self
    where
        A: Into<EphemeralMessageIdentity>,
        B: Into<String>,
    {
        Self {
            identity: identity.into(),
            text: text.into(),
            entities: None,
            link_preview_options: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// Sets a new list of entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the quote.
    ///
    /// It can be specified instead of parse mode.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the quote.
    ///
    /// It can be specified instead of entities.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.entities = None;
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

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - An object for an inline keyboard
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for EditEphemeralMessageText {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("editEphemeralMessageText", self)
    }
}

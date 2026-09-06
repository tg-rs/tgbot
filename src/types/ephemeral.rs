use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{
        ChatId,
        InlineKeyboardMarkup,
        InputMedia,
        InputMediaData,
        InputText,
        InputTextCaption,
        Integer,
        LinkPreviewOptions,
    },
};

/// Ephemeral message edit parameters.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EphemeralMessageParameters {
    receiver_user_id: Integer,
    callback_query_id: Option<String>,
    replace_callback_query_message: Option<bool>,
}

impl From<Integer> for EphemeralMessageParameters {
    fn from(value: Integer) -> Self {
        Self {
            receiver_user_id: value,
            callback_query_id: None,
            replace_callback_query_message: None,
        }
    }
}

impl EphemeralMessageParameters {
    /// Sets a new callback query ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the callback query which triggered the message.
    pub fn with_callback_query_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.callback_query_id = Some(value.into());
        self
    }

    /// Sets a new value for the `replace_callback_query_message` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the ephemeral message must be shown in place of the original message.
    ///
    /// Must be false for callback queries from ephemeral messages,
    /// which must be edited using regular editEphemeralMessage methods.
    pub fn with_replace_callback_query_message(mut self, value: bool) -> Self {
        self.replace_callback_query_message = Some(value);
        self
    }
}

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
    #[serde(flatten)]
    caption: Option<InputTextCaption>,
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
        T: Into<InputTextCaption>,
    {
        self.caption = Some(value.into());
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
    identity: EphemeralMessageIdentity,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditEphemeralMessageMedia {
    /// Creates a new `EditEphemeralMessageMedia`.
    ///
    /// # Arguments
    ///
    /// * `identity` - Identity of the ephemeral message.
    /// * `media` - The new media content of the message.
    pub fn new<A, B>(identity: A, media: B) -> Self
    where
        A: Into<EphemeralMessageIdentity>,
        B: Into<InputMedia>,
    {
        Self {
            identity: identity.into(),
            media: media.into(),
            reply_markup: None,
        }
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
struct EditEphemeralMessageMediaParameters {
    #[serde(flatten)]
    identity: EphemeralMessageIdentity,
    media: InputMediaData,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditEphemeralMessageMedia {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self {
            identity,
            media,
            reply_markup,
        } = self;
        let mut form = Form::default();
        let parameters = EditEphemeralMessageMediaParameters {
            identity,
            media: media.write(&mut form),
            reply_markup,
        };
        parameters.serialize(&mut form)?;
        Payload::form("editEphemeralMessageMedia", form)
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
    #[serde(flatten)]
    text: InputText,
    link_preview_options: Option<LinkPreviewOptions>,
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
        B: Into<InputText>,
    {
        Self {
            identity: identity.into(),
            text: text.into(),
            link_preview_options: None,
            reply_markup: None,
        }
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

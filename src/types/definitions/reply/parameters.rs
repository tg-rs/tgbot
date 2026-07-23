use serde::{Deserialize, Serialize};

use crate::types::{ChatId, InputText, Integer, ParseMode, TextEntities};

/// Describes reply parameters for the message that is being sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReplyParameters {
    message_id: Option<Integer>,
    allow_sending_without_reply: Option<bool>,
    chat_id: Option<ChatId>,
    checklist_task_id: Option<Integer>,
    ephemeral_message_id: Option<Integer>,
    poll_option_id: Option<String>,
    #[serde(flatten)]
    quote: Option<ReplyQuote>,
}

impl ReplyParameters {
    /// Creates a new `ReplyParameters`.
    ///
    /// # Arguments
    ///
    /// * `message_id` - Identifier of the message that will be replied to in the current chat,
    ///   or in the chat chat_id if it is specified.
    pub fn new(message_id: Integer) -> Self {
        Self {
            message_id: Some(message_id),
            allow_sending_without_reply: None,
            chat_id: None,
            checklist_task_id: None,
            ephemeral_message_id: None,
            poll_option_id: None,
            quote: None,
        }
    }

    /// Sets a new value for the `allow_sending_without_reply` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message should be sent even if the specified message to be replied to is not found.
    ///
    /// Can be used only for replies in the same chat and forum topic.
    pub fn with_allow_sending_without_reply(mut self, value: bool) -> Self {
        self.allow_sending_without_reply = Some(value);
        self
    }

    /// Sets a new chat ID.
    ///
    /// # Arguments
    ///
    /// * `value` - If the message to be replied to is from a different chat, unique identifier for the chat.
    pub fn with_chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = Some(value.into());
        self
    }

    /// Sets a new checklist task ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the specific checklist task to be replied to.
    pub fn with_checklist_task_id(mut self, value: Integer) -> Self {
        self.checklist_task_id = Some(value);
        self
    }

    /// Sets a new ephemeral message ID.
    ///
    /// # Arguments
    ///
    /// * `value` -  Identifier of the incoming ephemeral message that will be replied to in the current chat.
    ///
    /// A reply to an ephemeral message must itself be an ephemeral message.
    /// An ephemeral message may only be replied to within 15 seconds of being sent.
    /// Required if message ID isn't specified.
    pub fn with_ephemeral_message_id(mut self, value: Integer) -> Self {
        self.ephemeral_message_id = Some(value);
        self
    }

    /// Sets a new poll option ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Persistent identifier of the specific poll option to be replied to.
    pub fn with_poll_option_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.poll_option_id = Some(value.into());
        self
    }

    /// Sets a new quote.
    ///
    /// # Arguments
    ///
    /// * `value` - Quoted part of the message to be replied to.
    pub fn with_quote(mut self, value: ReplyQuote) -> Self {
        self.quote = Some(value);
        self
    }
}

/// Quoted part of the message to be replied to.
///
/// The text must contain 0-1024 characters after entities parsing.
/// The quote must be an exact substring of the message to be replied to,
/// including bold, italic, underline, strikethrough, spoiler,
/// custom_emoji and date_time entities.
/// The message will fail to send if the quote isn't found in the original message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReplyQuote {
    #[serde(rename = "quote_position")]
    position: Integer,
    #[serde(flatten)]
    text: ReplyQuoteText,
}

impl ReplyQuote {
    /// Creates a new `ReplyQuote`.
    ///
    /// # Arguments
    ///
    /// * `position` - Position of the quote in the original message in UTF-16 code units.
    /// * `text` - Quoted part of the message to be replied to.
    pub fn new<T>(position: Integer, text: T) -> Self
    where
        T: Into<InputText>,
    {
        Self {
            position,
            text: ReplyQuoteText::from(text),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
struct ReplyQuoteText {
    #[serde(rename = "quote")]
    data: String,
    #[serde(rename = "quote_entities")]
    entities: Option<TextEntities>,
    #[serde(rename = "quote_parse_mode")]
    parse_mode: Option<ParseMode>,
}

impl<T> From<T> for ReplyQuoteText
where
    T: Into<InputText>,
{
    fn from(value: T) -> Self {
        let InputText {
            data,
            entities,
            parse_mode,
        } = value.into();
        Self {
            data,
            entities,
            parse_mode,
        }
    }
}

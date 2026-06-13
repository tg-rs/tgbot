use serde::{Deserialize, Serialize};

use super::block::RichBlock;
use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, Message, ReplyMarkup, ReplyParameters, SuggestedPostParameters},
};

/// Rich formatted message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RichMessage {
    /// Content of the message.
    pub blocks: Vec<RichBlock>,
    /// Whether the message must be show right-to-left.
    pub is_rtl: Option<bool>,
}

impl<A, B> From<A> for RichMessage
where
    A: IntoIterator<Item = B>,
    B: Into<RichBlock>,
{
    fn from(value: A) -> Self {
        Self::from_iter(value)
    }
}

impl<I> FromIterator<I> for RichMessage
where
    I: Into<RichBlock>,
{
    fn from_iter<T>(value: T) -> Self
    where
        T: IntoIterator<Item = I>,
    {
        Self {
            blocks: value.into_iter().map(Into::into).collect(),
            is_rtl: None,
        }
    }
}

impl RichMessage {
    /// Sets a new value for the `is_rtl` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message must be shown right-to-left.
    pub fn with_is_rtl(mut self, value: bool) -> Self {
        self.is_rtl = Some(value);
        self
    }
}

/// Describes a rich message to be sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputRichMessage {
    html: Option<String>,
    markdown: Option<String>,
    is_rtl: Option<bool>,
    skip_entity_detection: Option<bool>,
}

impl InputRichMessage {
    /// Creates a new `InputRichMessage`.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the rich message to send described using Markdown formatting.
    pub fn markdown<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            html: None,
            markdown: Some(value.into()),
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Creates a new `InputRichMessage`.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the rich message to send described using HTML formatting.
    pub fn html<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            html: Some(value.into()),
            markdown: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Sets a new value for the `is_rtl` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the rich message must be shown right-to-left.
    pub fn with_is_rtl(mut self, value: bool) -> Self {
        self.is_rtl = Some(value);
        self
    }

    /// Sets a new value for the `skip_entity_detection` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whethert to skip automatic detection of entities in the text.
    pub fn with_skip_entity_detection(mut self, value: bool) -> Self {
        self.skip_entity_detection = Some(value);
        self
    }
}

/// Sends rich messages.
///
/// If the message contains a block with a media element,
/// then the bot must have the right to send the media to the chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendRichMessage {
    chat_id: ChatId,
    rich_message: InputRichMessage,
    allow_paid_broadcast: Option<bool>,
    business_connection_id: Option<String>,
    direct_messages_topic_id: Option<Integer>,
    disable_notification: Option<bool>,
    message_effect_id: Option<String>,
    message_thread_id: Option<Integer>,
    protect_content: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
    reply_parameters: Option<ReplyParameters>,
    suggested_post_parameters: Option<SuggestedPostParameters>,
}

impl SendRichMessage {
    /// Creates a new `SendRichMessage`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `rich_message` - The message to be sent.
    pub fn new<T>(chat_id: T, rich_message: InputRichMessage) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            rich_message,
            allow_paid_broadcast: None,
            business_connection_id: None,
            direct_messages_topic_id: None,
            disable_notification: None,
            message_effect_id: None,
            message_thread_id: None,
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
    /// * `value` - Whether to allow up to 1000 messages per second,
    ///   ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message.
    ///
    /// The relevant Stars will be withdrawn from the bot's balance.
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
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
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

impl Method for SendRichMessage {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendRichMessage", self)
    }
}

/// Streams a partial rich message to a user while the message is being generated.
///
/// Note that streamed draft is ephemeral and acts as a temporary
/// 30-second preview - once the output is finalized.
///
/// You must call [`crate::types::SendRichMessage`] with the complete message
/// to persist it in the user's chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendRichMessageDraft {
    chat_id: Integer,
    draft_id: Integer,
    rich_message: InputRichMessage,
    message_thread_id: Option<Integer>,
}

impl SendRichMessageDraft {
    /// Creates a new `SendRichMessageDraft`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `draft_id` - Unique identifier of the message draft; must be non-zero;
    ///   changes to drafts with the same identifier are animated.
    /// * `rich_message` - The partial message to be streamed.
    pub fn new(chat_id: Integer, draft_id: Integer, rich_message: InputRichMessage) -> Self {
        Self {
            chat_id,
            draft_id,
            rich_message,
            message_thread_id: None,
        }
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }
}

impl Method for SendRichMessageDraft {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("sendRichMessageDraft", self)
    }
}

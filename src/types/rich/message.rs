use serde::{Deserialize, Serialize};

use super::block::RichBlock;
use crate::{
    api::{Form, Method, Payload, PayloadError, WriteForm},
    types::{
        ChatId,
        InputMedia,
        InputMediaData,
        InputRichBlock,
        InputRichBlockData,
        Integer,
        Message,
        ReplyMarkup,
        ReplyParameters,
        SuggestedPostParameters,
    },
};

/// Rich formatted message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RichMessage {
    /// Content of the message.
    pub blocks: Vec<RichBlock>,
    /// Whether the message must be shown right-to-left.
    pub is_rtl: Option<bool>,
}

/// Describes a rich message to be sent.
#[derive(Debug)]
pub struct InputRichMessage {
    data: InputRichMessageData,
    blocks: Option<Vec<InputRichBlock>>,
    media: Option<Vec<(String, InputMedia)>>,
}

impl InputRichMessage {
    fn new(data: InputRichMessageData) -> Self {
        Self {
            data,
            blocks: None,
            media: None,
        }
    }

    /// Creates a new `InputRichMessage`.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the rich message to send described using Markdown formatting.
    pub fn markdown<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(InputRichMessageData::markdown(value))
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
        Self::new(InputRichMessageData::html(value))
    }

    /// Creates a new `InputRichMessage`.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the rich message to send described as a list of blocks.
    pub fn blocks<T>(value: T) -> Self
    where
        T: IntoIterator<Item = InputRichBlock>,
    {
        let mut result = Self::new(InputRichMessageData::default());
        result.blocks = Some(value.into_iter().collect());
        result
    }

    /// Sets a new value for the `is_rtl` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the rich message must be shown right-to-left.
    pub fn with_is_rtl(mut self, value: bool) -> Self {
        self.data.is_rtl = Some(value);
        self
    }

    /// Sets a new list of media.
    ///
    /// # Arguments
    ///
    /// * `value` - A sequence of `(id, media)` pairs.
    ///
    /// Media specified in the markdown or html fields using
    /// `tg://(audio|photo|video)?id=` links.
    pub fn with_media<A, B, C>(mut self, value: A) -> Self
    where
        A: IntoIterator<Item = (B, C)>,
        B: Into<String>,
        C: Into<InputMedia>,
    {
        self.media = Some(value.into_iter().map(|(id, media)| (id.into(), media.into())).collect());
        self
    }

    /// Sets a new value for the `skip_entity_detection` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whethert to skip automatic detection of entities in the text.
    pub fn with_skip_entity_detection(mut self, value: bool) -> Self {
        self.data.skip_entity_detection = Some(value);
        self
    }
}

impl WriteForm for InputRichMessage {
    type Output = InputRichMessageData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self {
            mut data,
            blocks,
            media,
        } = self;
        data.blocks = blocks.map(|items| items.into_iter().map(|x| x.write(form)).collect());
        data.media = media.map(|items| {
            items
                .into_iter()
                .map(|(id, media)| InputRichMessageMedia {
                    id,
                    media: media.write(form),
                })
                .collect()
        });
        data
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub(crate) struct InputRichMessageData {
    blocks: Option<Vec<InputRichBlockData>>,
    html: Option<String>,
    markdown: Option<String>,
    media: Option<Vec<InputRichMessageMedia>>,
    is_rtl: Option<bool>,
    skip_entity_detection: Option<bool>,
}

impl InputRichMessageData {
    fn html<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            blocks: None,
            html: Some(value.into()),
            markdown: None,
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    fn markdown<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            blocks: None,
            html: None,
            markdown: Some(value.into()),
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }
}

#[derive(Debug, Serialize)]
struct InputRichMessageMedia {
    id: String,
    media: InputMediaData,
}

/// Sends rich messages.
///
/// If the message contains a block with a media element,
/// then the bot must have the right to send the media to the chat.
#[derive(Debug)]
pub struct SendRichMessage {
    chat_id: ChatId,
    rich_message: InputRichMessage,
    parameters: SendRichMessageParameters,
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
            parameters: SendRichMessageParameters::default(),
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
struct SendRichMessageParameters {
    chat_id: Option<ChatId>,
    rich_message: Option<InputRichMessageData>,
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

impl Method for SendRichMessage {
    type Response = Message;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self {
            chat_id,
            rich_message,
            mut parameters,
        } = self;
        let mut form = Form::default();
        parameters.chat_id = Some(chat_id);
        parameters.rich_message = Some(rich_message.write(&mut form));
        parameters.serialize(&mut form)?;
        Payload::form("sendRichMessage", form)
    }
}

/// Streams a partial rich message to a user while the message is being generated.
///
/// Note that streamed draft is ephemeral and acts as a temporary
/// 30-second preview - once the output is finalized.
///
/// You must call [`crate::types::SendRichMessage`] with the complete message
/// to persist it in the user's chat.
#[derive(Debug)]
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
struct SendRichMessageDraftParameters {
    chat_id: Integer,
    draft_id: Integer,
    rich_message: InputRichMessageData,
    message_thread_id: Option<Integer>,
}

impl Method for SendRichMessageDraft {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        let Self {
            chat_id,
            draft_id,
            rich_message,
            message_thread_id,
        } = self;
        let mut form = Form::default();
        let parameters = SendRichMessageDraftParameters {
            chat_id,
            draft_id,
            rich_message: rich_message.write(&mut form),
            message_thread_id,
        };
        parameters.serialize(&mut form)?;
        Payload::form("sendRichMessageDraft", form)
    }
}

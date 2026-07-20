use serde::{Deserialize, Serialize};

use super::block::RichBlock;
use crate::{
    api::{Form, Method, Payload, PayloadError},
    types::{
        ChatId,
        InputMedia,
        InputMediaData,
        InputRichBlock,
        InputRichBlockData,
        Integer,
        Message,
        ReplyMarkup,
        ReplyMarkupError,
        ReplyParameters,
        ReplyParametersError,
        SerializeError,
        SuggestedPostParameters,
        SuggestedPostParametersError,
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

    pub(crate) fn into_parts(mut self, suffix: &[usize]) -> (Form, InputRichMessageData) {
        let suffix = suffix.to_vec();
        let mut form = self
            .blocks
            .map(|x| self.data.attach_blocks(x, suffix.clone()))
            .unwrap_or_default();
        if let Some(media) = self.media {
            form.extend(self.data.attach_media(media, suffix));
        }

        (form, self.data)
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

    fn attach_blocks(&mut self, value: Vec<InputRichBlock>, suffix: Vec<usize>) -> Form {
        let mut form = Form::default();
        let mut items = Vec::new();
        for (idx, i) in value.into_iter().enumerate() {
            let mut item_suffix = suffix.clone();
            item_suffix.push(idx);
            let (item_form, data) = i.into_parts(&item_suffix);
            form.extend(item_form);
            items.push(data);
        }
        self.blocks = Some(items);
        form
    }

    fn attach_media(&mut self, value: Vec<(String, InputMedia)>, suffix: Vec<usize>) -> Form {
        let mut form = Form::default();
        let mut items = Vec::new();
        for (idx, (id, input_media)) in value.into_iter().enumerate() {
            let mut item_suffix = suffix.clone();
            item_suffix.push(idx);
            let (media_form, media) = input_media.into_parts(&item_suffix);
            form.extend(media_form);
            items.push(InputRichMessageMedia { id, media });
        }
        self.media = Some(items);
        form
    }

    pub(crate) fn serialize(&self) -> Result<String, SerializeError> {
        serde_json::to_string(&self).map_err(SerializeError::input_rich_message_data)
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
    form: Form,
}

impl SendRichMessage {
    /// Creates a new `SendRichMessage`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `rich_message` - The message to be sent.
    pub fn new<T>(chat_id: T, rich_message: InputRichMessage) -> Result<Self, SerializeError>
    where
        T: Into<ChatId>,
    {
        let (mut form, data) = rich_message.into_parts(&[0]);
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("rich_message", data.serialize()?);
        Ok(Self { form })
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
        self.form.insert_field("allow_paid_broadcast", value);
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
        self.form.insert_field("business_connection_id", value.into());
        self
    }

    /// Sets a new direct messages topic ID
    ///
    /// * `value` - Identifier of the direct messages topic to which the message will be sent.
    ///
    /// Required if the message is sent to a direct messages chat.
    pub fn with_direct_messages_topic_id(mut self, value: Integer) -> Self {
        self.form.insert_field("direct_messages_topic_id", value);
        self
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///   a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
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
        self.form.insert_field("message_effect_id", value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///   of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.form.insert_field("protect_content", value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Result<Self, ReplyMarkupError>
    where
        T: Into<ReplyMarkup>,
    {
        self.form.insert_field("reply_markup", value.into().serialize()?);
        Ok(self)
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Result<Self, ReplyParametersError> {
        self.form.insert_field("reply_parameters", value.serialize()?);
        Ok(self)
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
    pub fn with_suggested_post_parameters(
        mut self,
        value: SuggestedPostParameters,
    ) -> Result<Self, SuggestedPostParametersError> {
        self.form.insert_field("suggested_post_parameters", value.serialize()?);
        Ok(self)
    }
}

impl Method for SendRichMessage {
    type Response = Message;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::form("sendRichMessage", self.form)
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
    form: Form,
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
    pub fn new(chat_id: Integer, draft_id: Integer, rich_message: InputRichMessage) -> Result<Self, SerializeError> {
        let (mut form, data) = rich_message.into_parts(&[0]);
        form.insert_field("chat_id", chat_id);
        form.insert_field("draft_id", draft_id);
        form.insert_field("rich_message", data.serialize()?);
        Ok(Self { form })
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///   for forum supergroups and private chats of bots with forum topic mode enabled only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.form.insert_field("message_thread_id", value);
        self
    }
}

impl Method for SendRichMessageDraft {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::form("sendRichMessageDraft", self.form)
    }
}

use serde::{Deserialize, Deserializer, Serialize};

pub use self::{command::*, data::*, methods::*, origin::*, quote::*, reply::*, sender::*};
use crate::types::{Chat, InlineKeyboardMarkup, Integer, LinkPreviewOptions, SuggestedPostInfo, Text, User};

mod command;
mod data;
mod methods;
mod origin;
mod quote;
mod reply;
mod sender;

/// Represents a result of `EditMessage*` requests.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot.
    Message(Message),
    /// Returned if edited message is NOT sent by the bot.
    Bool(bool),
}

/// Describes a message that was deleted or is otherwise inaccessible to the bot.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to.
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: Integer,
}

/// Describes a message that can be inaccessible to the bot.
#[derive(Clone, Debug, derive_more::From, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    /// Describes a message that was deleted or is otherwise inaccessible to the bot.
    InaccessibleMessage(InaccessibleMessage),
    /// Describes a regular message.
    Message(Box<Message>),
}

impl<'de> Deserialize<'de> for MaybeInaccessibleMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Message::deserialize(deserializer).map(|x| {
            if x.date == 0 {
                Self::InaccessibleMessage(InaccessibleMessage {
                    chat: x.chat,
                    message_id: x.id,
                })
            } else {
                Self::Message(Box::new(x))
            }
        })
    }
}

/// Represents a message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    /// Chat the message belongs to.
    pub chat: Chat,
    /// Date the message was sent in Unix time.
    pub date: Integer,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<Integer>,
    /// Indicates whether the message can't be forwarded.
    #[serde(default)]
    pub has_protected_content: bool,
    /// Unique message identifier inside the chat.
    #[serde(rename = "message_id")]
    pub id: Integer,
    /// Indicates whether the message is a channel post that
    /// was automatically forwarded
    /// to the connected discussion group.
    #[serde(default)]
    pub is_automatic_forward: bool,
    /// Sender of the message.
    #[serde(flatten)]
    pub sender: MessageSender,
    /// Author signature.
    pub author_signature: Option<String>,
    /// Unique identifier of the business connection from which the message was received.
    ///
    /// If non-empty, the message belongs to a chat of the corresponding business account
    /// that is independent from any potential bot chat which might share the same identifier.
    pub business_connection_id: Option<String>,
    /// Information about the direct messages chat topic that contains the message.
    pub direct_messages_topic: Option<DirectMessagesTopic>,
    /// Unique identifier of the message effect added to the message.
    pub effect_id: Option<String>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic.
    pub external_reply: Option<ExternalReplyInfo>,
    /// formation about the original message for forwarded messages.
    pub forward_origin: Option<MessageOrigin>,
    /// Indicates whether the message media is covered by a spoiler animation.
    pub has_media_spoiler: Option<bool>,
    /// Whether the message was sent by an implicit action.
    ///
    /// For example, as an away or a greeting business message, or as a scheduled message.
    pub is_from_offline: Option<bool>,
    /// Whether the message is a paid post.
    ///
    /// Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    pub is_paid_post: Option<bool>,
    /// Indicates whether the message is sent to a topic in a forum supergroup or a private chat with the bot.
    pub is_topic_message: Option<bool>,
    /// Options used for link preview generation for the message,
    /// if it is a text message and link preview options were changed.
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Unique identifier of a media message group this message belongs to.
    pub media_group_id: Option<String>,
    /// Unique identifier of a message thread to which the message belongs;
    /// for supergroups and private chats only.
    pub message_thread_id: Option<Integer>,
    /// The number of Telegram Stars that were paid by the sender of the message to send it.
    pub paid_star_count: Option<Integer>,
    /// For replies that quote part of the original message, the quoted part of the message.
    pub quote: Option<TextQuote>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// For replies, the original message or story.
    #[serde(flatten)]
    pub reply_to: Option<ReplyTo>,
    /// Identifier of the specific checklist task that is being replied to.
    pub reply_to_checklist_task_id: Option<Integer>,
    /// Number of boosts added by the user.
    ///
    /// Contains a value only if the sender of the message boosted the chat.
    pub sender_boost_count: Option<Integer>,
    /// The bot that actually sent the message on behalf of the business account.
    ///
    /// Available only for outgoing messages sent on behalf of the connected business account.
    pub sender_business_bot: Option<User>,
    /// Whether the caption must be shown above the message media.
    pub show_caption_above_media: Option<bool>,
    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat.
    ///
    /// If the message is an approved or declined suggested post, then it can't be edited.
    pub suggested_post_info: Option<SuggestedPostInfo>,
    /// Bot through which the message was sent.
    pub via_bot: Option<User>,

    /// Contains message data.
    #[serde(flatten)]
    pub data: MessageData,
}

impl Message {
    /// Creates a new `Message`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique message identifier inside the chat.
    /// * `date` - Date the message was sent in Unix time.
    /// * `chat` - Chat the message belongs to.
    /// * `data` - Data of the message.
    /// * `sender` - Sender of the message.
    pub fn new<A, B>(id: Integer, date: Integer, chat: A, data: MessageData, sender: B) -> Self
    where
        A: Into<Chat>,
        B: Into<MessageSender>,
    {
        Self {
            chat: chat.into(),
            data,
            date,
            edit_date: None,
            has_protected_content: false,
            id,
            is_automatic_forward: false,
            sender: sender.into(),
            author_signature: None,
            business_connection_id: None,
            direct_messages_topic: None,
            effect_id: None,
            external_reply: None,
            forward_origin: None,
            has_media_spoiler: None,
            is_from_offline: None,
            is_paid_post: None,
            is_topic_message: None,
            link_preview_options: None,
            media_group_id: None,
            message_thread_id: None,
            paid_star_count: None,
            quote: None,
            reply_markup: None,
            reply_to: None,
            reply_to_checklist_task_id: None,
            sender_boost_count: None,
            sender_business_bot: None,
            show_caption_above_media: None,
            suggested_post_info: None,
            via_bot: None,
        }
    }

    /// Returns `true` if the message has edited and `false` otherwise.
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns a text of the message (includes caption).
    pub fn get_text(&self) -> Option<&Text> {
        match self.data {
            MessageData::Text(ref text)
            | MessageData::Audio(MessageDataAudio {
                caption: Some(ref text),
                ..
            })
            | MessageData::Document(MessageDataDocument {
                caption: Some(ref text),
                ..
            })
            | MessageData::Photo(MessageDataPhoto {
                caption: Some(ref text),
                ..
            })
            | MessageData::Video(MessageDataVideo {
                caption: Some(ref text),
                ..
            })
            | MessageData::Voice(MessageDataVoice {
                caption: Some(ref text),
                ..
            }) => Some(text),
            _ => None,
        }
    }

    /// Sets a new chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat.
    pub fn with_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.chat = value.into();
        self
    }

    /// Sets a new data.
    ///
    /// # Arguments
    ///
    /// * `value` - Data of the message.
    pub fn with_data(mut self, value: MessageData) -> Self {
        self.data = value;
        self
    }

    /// Sets a new date.
    ///
    /// # Arguments
    ///
    /// * `value` - Date; Unix timestamp.
    pub fn with_date(mut self, value: Integer) -> Self {
        self.date = value;
        self
    }

    /// Sets a new direct messages topic.
    ///
    /// # Arguments
    ///
    /// * `value` - Information about the direct messages chat topic that contains the message.
    pub fn with_direct_messages_topic(mut self, value: DirectMessagesTopic) -> Self {
        self.direct_messages_topic = Some(value);
        self
    }

    /// Sets a new edit date.
    ///
    /// # Arguments
    ///
    /// * `value` - Edit date; Unix timestamp.
    pub fn with_edit_date(mut self, value: Integer) -> Self {
        self.edit_date = Some(value);
        self
    }

    /// Sets a new effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect.
    pub fn with_effect_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.effect_id = Some(value.into());
        self
    }

    /// Sets a new value for the `has_protected_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether messages from the chat can't be forwarded to other chats.
    pub fn with_has_protected_content(mut self, value: bool) -> Self {
        self.has_protected_content = value;
        self
    }

    /// Sets a new ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier inside a chat.
    pub fn with_id(mut self, value: Integer) -> Self {
        self.id = value;
        self
    }

    /// Sets a new value for the `is_automatic_forward` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the message was automatically forwarded.
    pub fn with_is_automatic_forward(mut self, value: bool) -> Self {
        self.is_automatic_forward = value;
        self
    }

    /// Sets a new sender.
    ///
    /// # Arguments
    ///
    /// * `value` - Sender.
    pub fn with_sender<T>(mut self, value: T) -> Self
    where
        T: Into<MessageSender>,
    {
        self.sender = value.into();
        self
    }

    /// Sets a new author signature.
    ///
    /// # Arguments
    ///
    /// * `value` - Author signature.
    pub fn with_author_signature<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.author_signature = Some(value.into());
        self
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection from which the message was received.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new external reply.
    ///
    /// # Arguments
    ///
    /// * `value` - Information about the message that is being replied to,
    ///   which may come from another chat or forum topic.
    pub fn with_external_reply(mut self, value: ExternalReplyInfo) -> Self {
        self.external_reply = Some(value);
        self
    }

    /// Sets a new forward origin.
    ///
    /// # Arguments
    ///
    /// * `value` - Information about the message origin.
    pub fn with_forward_origin(mut self, value: MessageOrigin) -> Self {
        self.forward_origin = Some(value);
        self
    }

    /// Sets a new value for the `has_media_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the message has a media spoiler.
    pub fn with_has_media_spoiler(mut self, value: bool) -> Self {
        self.has_media_spoiler = Some(value);
        self
    }

    /// Sets a new value for the `is_from_offline` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the message was sent by an implicit action.
    pub fn with_is_from_offline(mut self, value: bool) -> Self {
        self.is_from_offline = Some(value);
        self
    }

    /// Sets a new value for the `is_paid_post` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the message is a paid post.
    pub fn with_is_paid_post(mut self, value: bool) -> Self {
        self.is_paid_post = Some(value);
        self
    }

    /// Sets a new value for the `is_topic_message` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the message is a topic message.
    pub fn with_is_topic_message(mut self, value: bool) -> Self {
        self.is_topic_message = Some(value);
        self
    }

    /// Sets a new link preview options.
    ///
    /// # Arguments
    ///
    /// * `value` - New options.
    pub fn with_link_preview_options(mut self, value: LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(value);
        self
    }

    /// Sets a new media group ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Media group ID.
    pub fn with_media_group_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.media_group_id = Some(value.into());
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

    /// Sets a new paid star count
    ///
    /// # Arguments
    ///
    /// * `value` - The number of Telegram Stars that were paid by the sender of the message to send it.
    pub fn with_paid_star_count(mut self, value: Integer) -> Self {
        self.paid_star_count = Some(value);
        self
    }

    /// Sets a new quote
    ///
    /// # Arguments
    ///
    /// * `value` - The quoted part of the original message.
    pub fn with_quote(mut self, value: TextQuote) -> Self {
        self.quote = Some(value);
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

    /// Sets a new original message or story for the reply.
    ///
    /// # Arguments
    ///
    /// * `value` - For replies, the original message or story.
    pub fn with_reply_to<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyTo>,
    {
        self.reply_to = Some(value.into());
        self
    }

    /// Sets a new reply checklist task ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the specific checklist task that is being replied to.
    pub fn with_reply_to_checklist_task_id(mut self, value: Integer) -> Self {
        self.reply_to_checklist_task_id = Some(value);
        self
    }

    /// Sets a new sender boost count.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of boosts added by the user.
    pub fn with_sender_boost_count(mut self, value: Integer) -> Self {
        self.sender_boost_count = Some(value);
        self
    }

    /// Sets a new bot.
    ///
    /// # Arguments
    ///
    /// * `value` - The bot that actually sent the message on behalf of the business account.
    pub fn with_sender_business_bot(mut self, value: User) -> Self {
        self.sender_business_bot = Some(value);
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new suggested post info.
    ///
    /// # Arguments
    ///
    /// * `value` - Information about suggested post parameters.
    pub fn with_suggested_post_info(mut self, value: SuggestedPostInfo) -> Self {
        self.suggested_post_info = Some(value);
        self
    }

    /// Sets a new bot.
    ///
    /// # Arguments
    ///
    /// * `value` - Bot through which the message was sent.
    pub fn with_via_bot(mut self, value: User) -> Self {
        self.via_bot = Some(value);
        self
    }
}

/// Describes a topic of a direct messages chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic.
    pub topic_id: Integer,
    /// Information about the user that created the topic.
    pub user: Option<User>,
}

impl DirectMessagesTopic {
    /// Creates a new `DirectMessagesTopic`.
    ///
    /// # Arguments
    ///
    /// * `topic_id` - Unique identifier of the topic.
    pub fn new(topic_id: Integer) -> Self {
        Self { topic_id, user: None }
    }

    /// Sets a new user.
    ///
    /// # Arguments
    ///
    /// * `value` - Information about the user that created the topic.
    pub fn with_user(mut self, value: User) -> Self {
        self.user = Some(value);
        self
    }
}

/// Represents an unique message identifier.
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageId {
    /// The unique message identifier.
    pub message_id: Integer,
}

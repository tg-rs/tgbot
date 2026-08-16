use serde::{Deserialize, Deserializer, Serialize};

pub use self::{command::*, data::*, guest::*, methods::*, origin::*, quote::*, reply::*, sender::*};
use crate::types::{Chat, InlineKeyboardMarkup, Integer, LinkPreviewOptions, SuggestedPostInfo, Text, User};

mod command;
mod data;
mod guest;
mod methods;
mod origin;
mod quote;
mod reply;
mod sender;

/// Represents a result of `EditMessage*` requests.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot.
    Message(Box<Message>),
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
#[derive(Clone, Debug, derive_more::From)]
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

impl Serialize for MaybeInaccessibleMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::InaccessibleMessage(value) => {
                let value = RawInaccessibleMessage {
                    message_id: value.message_id,
                    chat: &value.chat,
                    date: 0,
                };
                value.serialize(serializer)
            }
            Self::Message(value) => value.serialize(serializer),
        }
    }
}

#[derive(Serialize)]
struct RawInaccessibleMessage<'a> {
    message_id: Integer,
    chat: &'a Chat,
    date: Integer,
}

/// Represents a message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    // For ephemeral messages, identifier of the ephemeral message inside this chat.
    ///
    /// The identifier may be reused for another ephemeral message
    /// after the message is deleted or expires.
    pub ephemeral_message_id: Option<Integer>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic.
    pub external_reply: Option<ExternalReplyInfo>,
    /// Information about the original message for forwarded messages.
    pub forward_origin: Option<MessageOrigin>,
    /// For a message sent by a guest bot, this is the information about the user and chat.
    #[serde(flatten)]
    pub guest_bot: Option<MessageGuestBot>,
    /// The unique identifier for the guest query.
    ///
    /// Use this identifier with the method [`crate::types::AnswerGuestQuery`]
    /// to send a response message.
    ///
    /// If non-empty, the message belongs to the chat where the guest bot was summoned,
    /// which may not coincide with other existing bot chats sharing the same identifier.
    pub guest_query_id: Option<String>,
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
    /// For ephemeral messages, the use who received the message.
    pub receiver_user: Option<User>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// For replies, the original message or story.
    #[serde(flatten)]
    pub reply_to: Option<ReplyTo>,
    /// Identifier of the specific checklist task that is being replied to.
    pub reply_to_checklist_task_id: Option<Integer>,
    /// Persistent identifier of the specific poll option that is being replied to.
    pub reply_to_poll_option_id: Option<String>,
    /// Number of boosts added by the user.
    ///
    /// Contains a value only if the sender of the message boosted the chat.
    pub sender_boost_count: Option<Integer>,
    /// The bot that actually sent the message on behalf of the business account.
    ///
    /// Available only for outgoing messages sent on behalf of the connected business account.
    pub sender_business_bot: Option<User>,
    /// Tag or custom title of the sender of the message; for supergroups only.
    pub sender_tag: Option<String>,
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
    /// Returns `true` if the message has edited and `false` otherwise.
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns a text of the message (includes caption).
    pub fn get_text(&self) -> Option<&Text> {
        match &self.data {
            MessageData::Text(text) => Some(text),
            MessageData::Audio(audio) => audio.caption.as_ref(),
            MessageData::Document(doc) => doc.caption.as_ref(),
            MessageData::Photo(photo) => photo.caption.as_ref(),
            MessageData::Video(video) => video.caption.as_ref(),
            MessageData::Voice(voice) => voice.caption.as_ref(),
            _ => None,
        }
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

/// Represents an unique message identifier.
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageId {
    /// The unique message identifier.
    pub message_id: Integer,
}

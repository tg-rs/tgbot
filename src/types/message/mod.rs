use serde::{Deserialize, Serialize};

use crate::types::{Chat, InlineKeyboardMarkup, Integer, Text, User};

pub use self::{command::*, data::*, forward::*, methods::*, sender::*};

#[cfg(test)]
mod tests;

mod command;
mod data;
mod forward;
mod methods;
mod sender;

/// This object represents a message
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: Integer,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<Integer>,
    /// Sender of the message
    #[serde(flatten)]
    pub sender: MessageSender,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Author signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    /// True, if the message can't be forwarded
    #[serde(default)]
    pub has_protected_content: bool,
    /// Forwarded data
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<Forward>,
    /// True, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(default)]
    pub is_automatic_forward: bool,
    /// True, if the message is sent to a forum topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// For replies, the original message
    /// Note that the Message object in this field will not contain further
    /// reply_to fields even if it itself is a reply
    #[serde(rename = "reply_to_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Box<Message>>,
    /// Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,
    /// The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Contains message data
    #[serde(flatten)]
    pub data: MessageData,
}

impl Message {
    /// Returns true if message has edited and false otherwise
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns text of the message (includes caption)
    pub fn get_text(&self) -> Option<&Text> {
        match self.data {
            MessageData::Text(ref text)
            | MessageData::Audio {
                caption: Some(ref text),
                ..
            }
            | MessageData::Document {
                caption: Some(ref text),
                ..
            }
            | MessageData::Photo {
                caption: Some(ref text),
                ..
            }
            | MessageData::Video {
                caption: Some(ref text),
                ..
            }
            | MessageData::Voice {
                caption: Some(ref text),
                ..
            } => Some(text),
            _ => None,
        }
    }
}

/// Result of editMessage* requests
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot
    Message(Message),
    /// Returned if edited message is NOT sent by the bot
    Bool(bool),
}

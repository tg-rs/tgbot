use serde::{Deserialize, Serialize};

use crate::api::{Method, Payload};

pub use self::{
    action::*,
    boost::*,
    channel::*,
    group::*,
    id::*,
    invite_link::*,
    join_request::*,
    location::*,
    member::*,
    message::*,
    permissions::*,
    photo::*,
    private::*,
    sender_chat::*,
    sticker_set::*,
    supergroup::*,
};

#[cfg(test)]
mod tests;

mod action;
mod boost;
mod channel;
mod group;
mod id;
mod invite_link;
mod join_request;
mod location;
mod member;
mod message;
mod permissions;
mod photo;
mod private;
mod sender_chat;
mod sticker_set;
mod supergroup;

/// Represents a chat.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Chat {
    /// Represents a channel chat.
    Channel(ChannelChat),
    /// Represents a group chat.
    Group(GroupChat),
    /// Represents a private chat.
    Private(PrivateChat),
    /// Represents a supergroup chat.
    Supergroup(SupergroupChat),
}

impl Chat {
    /// Returns an ID of the chat.
    pub fn get_id(&self) -> ChatPeerId {
        match self {
            Chat::Channel(chat) => chat.id,
            Chat::Group(chat) => chat.id,
            Chat::Private(chat) => chat.id,
            Chat::Supergroup(chat) => chat.id,
        }
    }

    /// Returns a username of the chat.
    pub fn get_username(&self) -> Option<&ChatUsername> {
        match &self {
            Chat::Channel(chat) => chat.username.as_ref(),
            Chat::Group(_) => None,
            Chat::Private(chat) => chat.username.as_ref(),
            Chat::Supergroup(chat) => chat.username.as_ref(),
        }
    }
}

/// Returns up to date information about the chat.
#[derive(Clone, Debug, Serialize)]
pub struct GetChat {
    chat_id: ChatId,
}

impl GetChat {
    /// Creates a new `GetChat`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        GetChat {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChat {
    type Response = Chat;

    fn into_payload(self) -> Payload {
        Payload::json("getChat", self)
    }
}

/// Leaves a group, supergroup or channel.
#[derive(Clone, Debug, Serialize)]
pub struct LeaveChat {
    chat_id: ChatId,
}

impl LeaveChat {
    /// Creates a new `LeaveChat`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        LeaveChat {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for LeaveChat {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("leaveChat", self)
    }
}

/// Changes a description of a chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
#[derive(Clone, Debug, Serialize)]
pub struct SetChatDescription {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl SetChatDescription {
    /// Creates a new `SetChatDescription`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        SetChatDescription {
            chat_id: chat_id.into(),
            description: None,
        }
    }

    /// Sets a new chat description.
    ///
    /// # Arguments
    ///
    /// * `value` - Description; 0-255 characters.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(value.into());
        self
    }
}

impl Method for SetChatDescription {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatDescription", self)
    }
}

/// Changes a title of a chat.
///
/// Titles can't be changed for private chats.
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
///
/// # Notes
///
/// In regular groups (non-supergroups), this method will only work
/// if the ‘All Members Are Admins’ setting is off in the target group.
#[derive(Clone, Debug, Serialize)]
pub struct SetChatTitle {
    chat_id: ChatId,
    title: String,
}

impl SetChatTitle {
    /// Creates a new `SetChatTitle`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `title` - New chat title; 1-255 characters.
    pub fn new<A, B>(chat_id: A, title: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        SetChatTitle {
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }
}

impl Method for SetChatTitle {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setChatTitle", self)
    }
}

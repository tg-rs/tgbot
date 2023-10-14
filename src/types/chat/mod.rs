use serde::{Deserialize, Serialize};

use crate::{method::Method, request::Request, types::Integer};

pub use self::{
    action::*,
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

/// Chat
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Chat {
    /// Channel
    Channel(ChannelChat),
    /// Group
    Group(GroupChat),
    /// Private chat
    Private(PrivateChat),
    /// Supergroup
    Supergroup(SupergroupChat),
}

impl Chat {
    /// Returns ID of the chat
    pub fn get_id(&self) -> Integer {
        match self {
            Chat::Channel(chat) => chat.id,
            Chat::Group(chat) => chat.id,
            Chat::Private(chat) => chat.id,
            Chat::Supergroup(chat) => chat.id,
        }
    }

    /// Returns username of the chat
    pub fn get_username(&self) -> Option<&str> {
        match &self {
            Chat::Channel(chat) => &chat.username,
            Chat::Group(_) => &None,
            Chat::Private(chat) => &chat.username,
            Chat::Supergroup(chat) => &chat.username,
        }
        .as_deref()
    }
}

/// Get up to date information about the chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChat {
    chat_id: ChatId,
}

impl GetChat {
    /// Creates a new GetChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChat {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChat {
    type Response = Chat;

    fn into_request(self) -> Request {
        Request::json("getChat", self)
    }
}

/// Leave a group, supergroup or channel
#[derive(Clone, Debug, Serialize)]
pub struct LeaveChat {
    chat_id: ChatId,
}

impl LeaveChat {
    /// Creates a new LeaveChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        LeaveChat {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for LeaveChat {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("leaveChat", self)
    }
}

/// Change the description of a group, a supergroup or a channel
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
#[derive(Clone, Debug, Serialize)]
pub struct SetChatDescription {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl SetChatDescription {
    /// Creates a new SetChatDescription
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        SetChatDescription {
            chat_id: chat_id.into(),
            description: None,
        }
    }

    /// New chat description, 0-255 characters
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl Method for SetChatDescription {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setChatDescription", self)
    }
}

/// Change the title of a chat
///
/// Titles can't be changed for private chats
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
///
/// Note: In regular groups (non-supergroups), this method will only work
/// if the ‘All Members Are Admins’ setting is off in the target group
#[derive(Clone, Debug, Serialize)]
pub struct SetChatTitle {
    chat_id: ChatId,
    title: String,
}

impl SetChatTitle {
    /// Creates a new SetChatTitle
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * title - New chat title, 1-255 characters
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, title: S) -> Self {
        SetChatTitle {
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }
}

impl Method for SetChatTitle {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setChatTitle", self)
    }
}

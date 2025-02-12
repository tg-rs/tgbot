use serde::{Deserialize, Serialize};

pub use self::{
    action::*,
    boost::*,
    full_info::*,
    id::*,
    invite_link::*,
    join_request::*,
    location::*,
    member::*,
    message::*,
    permissions::*,
    photo::*,
    sender_chat::*,
    sticker_set::*,
};
use crate::{
    api::{Method, Payload},
    types::BackgroundType,
};

#[cfg(test)]
mod tests;

mod action;
mod boost;
mod full_info;
mod id;
mod invite_link;
mod join_request;
mod location;
mod member;
mod message;
mod permissions;
mod photo;
mod sender_chat;
mod sticker_set;

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

/// Represents a channel chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChannelChat {
    /// Unique identifier of the channel.
    pub id: ChatPeerId,
    /// Title of the channel.
    pub title: String,
    /// Username of the channel.
    pub username: Option<ChatUsername>,
}

impl ChannelChat {
    /// Creates a new `ChannelChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the channel.
    /// * `title` - Title of the channel.
    pub fn new<A, B>(id: A, title: B) -> Self
    where
        A: Into<ChatPeerId>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            username: None,
        }
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username of the channel.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<ChatUsername>,
    {
        self.username = Some(value.into());
        self
    }
}

/// Represents a group chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GroupChat {
    /// Unique identifier of the group.
    pub id: ChatPeerId,
    /// Title of the group.
    pub title: String,
}

impl GroupChat {
    /// Creates a new `GroupChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the group.
    /// * `title` - Title of the group.
    pub fn new<A, B>(id: A, title: B) -> Self
    where
        A: Into<ChatPeerId>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
        }
    }
}

/// Represents a private chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrivateChat {
    /// Unique identifier of the chat.
    pub id: ChatPeerId,
    /// First name of the other party.
    pub first_name: String,
    /// Last name of the other party.
    pub last_name: Option<String>,
    /// Username of the target chat.
    pub username: Option<ChatUsername>,
}

impl PrivateChat {
    /// Creates a new `PrivateChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the target chat.
    /// * `first_name` - First name of the other party.
    pub fn new<A, B>(id: A, first_name: B) -> Self
    where
        A: Into<ChatPeerId>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            first_name: first_name.into(),
            last_name: None,
            username: None,
        }
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - Last name.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<ChatUsername>,
    {
        self.username = Some(value.into());
        self
    }
}

/// Represents a supergroup chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SupergroupChat {
    /// Unique identifier of the supergroup.
    pub id: ChatPeerId,
    /// Title of the supergroup.
    pub title: String,
    /// Whether the supergroup has topic enabled.
    pub is_forum: Option<bool>,
    /// Username of the supergroup.
    pub username: Option<ChatUsername>,
}

impl SupergroupChat {
    /// Creates a new `SupergroupChat`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the supergroup.
    /// * `title` - Title of the supergroup.
    pub fn new<A, B>(id: A, title: B) -> Self
    where
        A: Into<ChatPeerId>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            is_forum: None,
            username: None,
        }
    }

    /// Sets a new value for an `is_forum` flag.
    ///
    /// # Arguments
    ///
    /// `value` - Whether the supergroup has topics enabled.
    pub fn with_is_forum(mut self, value: bool) -> Self {
        self.is_forum = Some(value);
        self
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username of the supergroup.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<ChatUsername>,
    {
        self.username = Some(value.into());
        self
    }
}

/// Represents a chat background.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatBackground {
    /// Type of the background
    #[serde(rename = "type")]
    pub background_type: BackgroundType,
}

impl From<BackgroundType> for ChatBackground {
    fn from(value: BackgroundType) -> Self {
        Self { background_type: value }
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
    type Response = ChatFullInfo;

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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetChatDescription {
    chat_id: ChatId,
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

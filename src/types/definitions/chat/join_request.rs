use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Chat, ChatId, ChatInviteLink, Integer, User},
};

/// Represents a join request sent to a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent.
    pub chat: Chat,
    /// Date the request was sent in Unix time.
    pub date: Integer,
    /// User that sent the join request.
    pub from: User,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request.
    pub invite_link: Option<ChatInviteLink>,
    /// Identifier of a private chat with the user who sent the join request.
    ///
    /// A bot can use this identifier for 5 minutes to
    /// send messages until the join request is processed,
    /// assuming no other administrator contacted the user.
    pub user_chat_id: Option<Integer>,
}

impl ChatJoinRequest {
    /// Creates a new `ChatJoinRequest`.
    ///
    /// # Arguments
    ///
    /// * `chat` - Chat to which the request was sent.
    /// * `date` - Date the request was sent in Unix time.
    /// * `from` - User that sent the join request.
    pub fn new(chat: Chat, date: Integer, from: User) -> Self {
        Self {
            chat,
            date,
            from,
            bio: None,
            invite_link: None,
            user_chat_id: None,
        }
    }

    /// Sets a new bio.
    ///
    /// # Arguments
    ///
    /// * `value` - The bio of the user.
    pub fn with_bio<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.bio = Some(value.into());
        self
    }

    /// Sets a new invite link.
    ///
    /// # Arguments
    ///
    /// * `value` - The chat invite link that was used by the user to send the join request.
    pub fn with_invite_link(mut self, value: ChatInviteLink) -> Self {
        self.invite_link = Some(value);
        self
    }

    /// Sets a new user chat ID.
    ///
    /// # Arguments
    ///
    /// * `value` - The identifier of a private chat with the user who sent the join request.
    pub fn with_user_chat_id(mut self, value: Integer) -> Self {
        self.user_chat_id = Some(value);
        self
    }
}

/// Approves a chat join request.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_invite_users` administrator right.
#[derive(Clone, Debug, Serialize)]
pub struct ApproveChatJoinRequest {
    chat_id: ChatId,
    user_id: Integer,
}

impl ApproveChatJoinRequest {
    /// Creates a new `ApproveChatJoinRequest`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `user_id` - Unique identifier of the target user.
    pub fn new<T>(chat_id: T, user_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl Method for ApproveChatJoinRequest {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("approveChatJoinRequest", self)
    }
}

/// Declines a chat join request.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_invite_users` administrator right.
#[derive(Clone, Debug, Serialize)]
pub struct DeclineChatJoinRequest {
    chat_id: ChatId,
    user_id: Integer,
}

impl DeclineChatJoinRequest {
    /// Creates a new `DeclineChatJoinRequest`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `user_id` - Unique identifier of the target user.
    pub fn new<T>(chat_id: T, user_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl Method for DeclineChatJoinRequest {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("declineChatJoinRequest", self)
    }
}

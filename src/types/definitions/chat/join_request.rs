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
    /// Identifier of the join request query.
    ///
    /// If present, then the bot must call [`crate::types::SendChatJoinRequestWebApp`]
    /// or directly call [`crate::types::AnswerChatJoinRequestQuery`] within 10 seconds.
    pub query_id: Option<String>,
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
            query_id: None,
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

    /// Sets a new query ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the join request query.
    pub fn with_query_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.query_id = Some(value.into());
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum ChatJoinRequestQueryResult {
    Approve,
    Decline,
    Queue,
}

/// Processes a received chat join request query.
#[derive(Clone, Debug, Serialize)]
pub struct AnswerChatJoinRequestQuery {
    chat_join_request_query_id: String,
    result: ChatJoinRequestQueryResult,
}

impl AnswerChatJoinRequestQuery {
    fn new<T>(query_id: T, result: ChatJoinRequestQueryResult) -> Self
    where
        T: Into<String>,
    {
        Self {
            chat_join_request_query_id: query_id.into(),
            result,
        }
    }

    /// Creates a new `AnswerChatJoinRequestQuery`.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Unique identifier of the request query.
    ///
    /// Allows the user to join the chat.
    pub fn approve<T>(query_id: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(query_id, ChatJoinRequestQueryResult::Approve)
    }

    /// Creates a new `AnswerChatJoinRequestQuery`.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Unique identifier of the request query.
    ///
    /// Disallows the user to join the chat.
    pub fn decline<T>(query_id: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(query_id, ChatJoinRequestQueryResult::Decline)
    }

    /// Creates a new `AnswerChatJoinRequestQuery`.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Unique identifier of the request query.
    ///
    /// Leaves the decision to other administrators.
    pub fn queue<T>(query_id: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(query_id, ChatJoinRequestQueryResult::Queue)
    }
}

impl Method for AnswerChatJoinRequestQuery {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("answerChatJoinRequestQuery", self)
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

/// Processes a received chat join request query by
/// showing a Mini App to the user before deciding the outcome.
#[derive(Clone, Debug, Serialize)]
pub struct SendChatJoinRequestWebApp {
    chat_join_request_query_id: String,
    web_app_url: String,
}

impl SendChatJoinRequestWebApp {
    /// Creates a new `SendChatJoinRequestWebApp`.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Unique identifier of the join request query.
    /// * `url` - The URL of the Mini App to be opened.
    pub fn new<A, B>(query_id: A, url: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            chat_join_request_query_id: query_id.into(),
            web_app_url: url.into(),
        }
    }
}

impl Method for SendChatJoinRequestWebApp {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("sendChatJoinRequestWebApp", self)
    }
}

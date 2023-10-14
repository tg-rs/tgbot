use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Chat, ChatId, ChatInviteLink, Integer, User},
};

#[cfg(test)]
mod tests;

/// Represents a join request sent to a chat
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Date the request was sent in Unix time
    pub date: Integer,
    /// Bio of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

/// Approve a chat join request
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_invite_users administrator right.
#[derive(Clone, Debug, Serialize)]
pub struct ApproveChatJoinRequest {
    chat_id: ChatId,
    user_id: Integer,
}

impl ApproveChatJoinRequest {
    /// Creates a new ApproveChatJoinRequest
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
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

/// Decline a chat join request
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_invite_users administrator right.
#[derive(Clone, Debug, Serialize)]
pub struct DeclineChatJoinRequest {
    chat_id: ChatId,
    user_id: Integer,
}

impl DeclineChatJoinRequest {
    /// Creates a new DeclineChatJoinRequest
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
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

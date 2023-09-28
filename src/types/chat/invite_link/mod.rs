use serde::{Deserialize, Serialize};

use crate::{
    method::Method,
    request::Request,
    types::{ChatId, Integer, User},
};

#[cfg(test)]
mod tests;

/// Represents an invite link for a chat.
#[derive(Clone, Debug, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link
    ///
    /// If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// True, if the link is primary
    pub is_primary: bool,
    /// True, if the link is revoked
    pub is_revoked: bool,
    /// Invite link name
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<Integer>,
    /// Maximum number of users that can be members
    /// of the chat simultaneously after joining
    /// the chat via this invite link; 1-99999
    pub member_limit: Option<Integer>,
    /// Number of pending join requests created using this link
    pub pending_join_request_count: Option<Integer>,
}

/// Create an additional invite link for a chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
/// The link can be revoked using the method `RevokeChatInviteLink`.
/// Returns the new invite link as `ChatInviteLink` object.
#[derive(Clone, Debug, Serialize)]
pub struct CreateChatInviteLink {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

impl CreateChatInviteLink {
    /// Creates a new CreateChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }

    /// Sets invite link name; 0-32 characters
    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets point in time (Unix timestamp) when the link will expire
    pub fn expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    pub fn member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// True, if users joining the chat via the link need to be approved by chat administrators.
    /// If True, member_limit can't be specified
    pub fn creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = Some(value);
        self
    }
}

impl Method for CreateChatInviteLink {
    type Response = ChatInviteLink;

    fn into_request(self) -> Request {
        Request::json("createChatInviteLink", self)
    }
}

/// Edit a non-primary invite link created by the bot
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
/// Returns the edited invite link as a `ChatInviteLink` object.
#[derive(Clone, Debug, Serialize)]
pub struct EditChatInviteLink {
    chat_id: ChatId,
    invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

impl EditChatInviteLink {
    /// Creates a new EditChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * invite_link - The invite link to edit
    pub fn new<C, I>(chat_id: C, invite_link: I) -> Self
    where
        C: Into<ChatId>,
        I: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }

    /// Sets invite link name; 0-32 characters
    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets point in time (Unix timestamp) when the link will expire
    pub fn expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    pub fn member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// True, if users joining the chat via the link need to be approved by chat administrators.
    /// If True, member_limit can't be specified
    pub fn creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = Some(value);
        self
    }
}

impl Method for EditChatInviteLink {
    type Response = ChatInviteLink;

    fn into_request(self) -> Request {
        Request::json("editChatInviteLink", self)
    }
}

/// Generate a new invite link for a chat
///
/// Any previously generated link is revoked.
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
/// Returns the new invite link as String on success.
///
/// Note: Each administrator in a chat generates their own invite links.
/// Bots can't use invite links generated by other administrators.
/// If you want your bot to work with invite links, it will need to generate
/// its own link using `ExportChatInviteLink` or by calling the `GetChat` method.
/// If your bot needs to generate a new primary invite link replacing its previous one,
/// use `ExportChatInviteLink` again.
#[derive(Clone, Debug, Serialize)]
pub struct ExportChatInviteLink {
    chat_id: ChatId,
}

impl ExportChatInviteLink {
    /// Creates a new ExportChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        ExportChatInviteLink {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for ExportChatInviteLink {
    type Response = String;

    fn into_request(self) -> Request {
        Request::json("exportChatInviteLink", self)
    }
}

/// Revoke an invite link created by the bot
///
/// If the primary link is revoked, a new link is automatically generated.
/// The bot must be an administrator in the chat for this to work and
/// must have the appropriate admin rights.
/// Returns the revoked invite link as ChatInviteLink object.
#[derive(Clone, Debug, Serialize)]
pub struct RevokeChatInviteLink {
    chat_id: ChatId,
    invite_link: String,
}

impl RevokeChatInviteLink {
    /// Creates a new RevokeChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * invite_link - The invite link to revoke
    pub fn new<C, I>(chat_id: C, invite_link: I) -> Self
    where
        C: Into<ChatId>,
        I: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }
}

impl Method for RevokeChatInviteLink {
    type Response = ChatInviteLink;

    fn into_request(self) -> Request {
        Request::json("revokeChatInviteLink", self)
    }
}

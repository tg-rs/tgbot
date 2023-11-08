use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, User},
};

#[cfg(test)]
mod tests;

/// Represents an invite link for a chat
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatInviteLink {
    /// Whether users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// Creator of the link
    pub creator: User,
    /// The invite link
    ///
    /// If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Whether the link is primary
    pub is_primary: bool,
    /// Whether the link is revoked
    pub is_revoked: bool,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<Integer>,
    /// Maximum number of users that can be members
    /// of the chat simultaneously after joining
    /// the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<Integer>,
    /// The name of the invite link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Number of pending join requests created using this link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<Integer>,
}

impl ChatInviteLink {
    /// Creates a new ChatInviteLink
    ///
    /// # Arguments
    ///
    /// * invite_link - Invite link
    /// * creator - Creator of the link
    ///
    /// All optional fields are `None` and flags are `false` by default.
    pub fn new<T>(invite_link: T, creator: User) -> Self
    where
        T: Into<String>,
    {
        Self {
            invite_link: invite_link.into(),
            creator,
            creates_join_request: false,
            is_primary: false,
            is_revoked: false,
            name: None,
            expire_date: None,
            member_limit: None,
            pending_join_request_count: None,
        }
    }

    /// Sets a new value for the `creates_join_request` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether users joining the chat via the link need
    ///           to be approved by chat administrators
    pub fn with_creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = value;
        self
    }

    /// Sets a new expiration date
    ///
    /// # Arguments
    ///
    /// * value - Point in time (Unix timestamp) when the link will expire or has been expired
    pub fn with_expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets a new value for the `is_primary` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the link is primary
    pub fn with_is_primary(mut self, value: bool) -> Self {
        self.is_primary = value;
        self
    }

    /// Sets a new value for the `is_revoked` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the link is revoked
    pub fn with_is_revoked(mut self, value: bool) -> Self {
        self.is_revoked = value;
        self
    }

    /// Sets a new member limit
    ///
    /// # Arguments
    ///
    /// * value - Maximum number of users that can be members
    ///           of the chat simultaneously after joining
    ///           the chat via this invite link; 1-99999
    pub fn with_member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// Sets a new name of the invite link
    ///
    /// # Arguments
    ///
    /// * value - Name
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }

    /// Sets a new pending join requests count
    ///
    /// # Arguments
    ///
    /// * value - Number of pending join requests created using this link
    pub fn with_pending_join_request_count(mut self, value: Integer) -> Self {
        self.pending_join_request_count = Some(value);
        self
    }
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
    creates_join_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl CreateChatInviteLink {
    /// Creates a new CreateChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier of the target chat
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            creates_join_request: None,
            expire_date: None,
            member_limit: None,
            name: None,
        }
    }

    /// Sets a new value for the `creates_join_request` flag
    ///
    /// * value - Whether users joining the chat via the link need
    ///           to be approved by chat administrators;
    ///           if `true`, member_limit can't be specified
    pub fn with_creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = Some(value);
        self
    }

    /// Sets a new expiration date
    ///
    /// # Arguments
    ///
    /// * value - Point in time (Unix timestamp) when the link will expire
    pub fn with_expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets a new member limit
    ///
    /// # Arguments
    ///
    /// * value - Maximum number of users that can be members of the chat simultaneously
    ///           after joining the chat via this invite link; 1-99999
    pub fn with_member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// Sets a new invite link name
    ///
    /// # Arguments
    ///
    /// * value - Name of the invite link; 0-32 characters
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }
}

impl Method for CreateChatInviteLink {
    type Response = ChatInviteLink;

    fn into_payload(self) -> Payload {
        Payload::json("createChatInviteLink", self)
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
    creates_join_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl EditChatInviteLink {
    /// Creates a new EditChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier of the target chat
    /// * invite_link - The invite link to edit
    pub fn new<A, B>(chat_id: A, invite_link: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            creates_join_request: None,
            expire_date: None,
            member_limit: None,
            name: None,
        }
    }

    /// Sets a new value for the `creates_join_request` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether users joining the chat via the link need
    ///           to be approved by chat administrators;
    ///           if True, member_limit can't be specified
    pub fn with_creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = Some(value);
        self
    }

    /// Sets a new expiration date
    ///
    /// # Arguments
    ///
    /// * value - Point in time (Unix timestamp) when the link will expire
    pub fn with_expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets a new member limit
    ///
    /// # Arguments
    ///
    /// * value - Maximum number of users that can be members of the chat simultaneously
    ///           after joining the chat via this invite link; 1-99999
    pub fn with_member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// Sets a new name of the invite link
    ///
    /// # Arguments
    ///
    /// * value - Name of the invite link; 0-32 characters
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }
}

impl Method for EditChatInviteLink {
    type Response = ChatInviteLink;

    fn into_payload(self) -> Payload {
        Payload::json("editChatInviteLink", self)
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
    /// * chat_id - Unique identifier of the target chat
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        ExportChatInviteLink {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for ExportChatInviteLink {
    type Response = String;

    fn into_payload(self) -> Payload {
        Payload::json("exportChatInviteLink", self)
    }
}

/// Revoke an invite link created by the bot
///
/// If the primary link is revoked, a new link is automatically generated.
/// The bot must be an administrator in the chat for this to work and
/// must have the appropriate admin rights.
/// Returns the revoked invite link as `ChatInviteLink` object.
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
    /// * chat_id - Unique identifier of the target chat
    /// * invite_link - The invite link to revoke
    pub fn new<A, B>(chat_id: A, invite_link: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }
}

impl Method for RevokeChatInviteLink {
    type Response = ChatInviteLink;

    fn into_payload(self) -> Payload {
        Payload::json("revokeChatInviteLink", self)
    }
}

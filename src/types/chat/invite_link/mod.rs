use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, User},
};

#[cfg(test)]
mod tests;

/// Represents an invite link for a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatInviteLink {
    /// Indicates whether users joining the chat via the link
    /// need to be approved by chat administrators.
    pub creates_join_request: bool,
    /// The creator of the link.
    pub creator: User,
    /// The invite link.
    ///
    /// If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Indicates whether the link is primary.
    pub is_primary: bool,
    /// Indicates whether the link is revoked.
    pub is_revoked: bool,
    /// The point in time (Unix timestamp) when the link will expire or has been expired.
    pub expire_date: Option<Integer>,
    /// The maximum number of users that can be members
    /// of the chat simultaneously after joining
    /// the chat via this invite link; 1-99999.
    pub member_limit: Option<Integer>,
    /// The name of the invite link.
    pub name: Option<String>,
    /// The number of pending join requests created using this link.
    pub pending_join_request_count: Option<Integer>,
}

impl ChatInviteLink {
    /// Creates a new `ChatInviteLink`.
    ///
    /// # Arguments
    ///
    /// * `invite_link` - Invite link.
    /// * `creator` - Creator of the link.
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

    /// Sets a new value for the `creates_join_request` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether users joining the chat via the link need
    ///   to be approved by chat administrators.
    pub fn with_creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = value;
        self
    }

    /// Sets a new expiration date.
    ///
    /// # Arguments
    ///
    /// * `value` - The point in time (Unix timestamp) when the link will expire or has been expired.
    pub fn with_expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets a new value for the `is_primary` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the link is primary.
    pub fn with_is_primary(mut self, value: bool) -> Self {
        self.is_primary = value;
        self
    }

    /// Sets a new value for the `is_revoked` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the link is revoked.
    pub fn with_is_revoked(mut self, value: bool) -> Self {
        self.is_revoked = value;
        self
    }

    /// Sets a new member limit
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum number of users that can be members
    ///   of the chat simultaneously after joining
    ///   the chat via this invite link; 1-99999.
    pub fn with_member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// Sets a new name of the invite link.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the invite link.
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }

    /// Sets a new pending join requests count.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of pending join requests created using this link.
    pub fn with_pending_join_request_count(mut self, value: Integer) -> Self {
        self.pending_join_request_count = Some(value);
        self
    }
}

/// Creates an additional invite link for a chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
/// The link can be revoked using the method [`RevokeChatInviteLink`].
/// Returns the new invite link as [`ChatInviteLink`] object.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CreateChatInviteLink {
    chat_id: ChatId,
    creates_join_request: Option<bool>,
    expire_date: Option<Integer>,
    member_limit: Option<Integer>,
    name: Option<String>,
}

impl CreateChatInviteLink {
    /// Creates a new `CreateChatInviteLink`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
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

    /// Sets a new value for the `creates_join_request` flag.
    ///
    /// * `value` - Indicates whether users joining the chat via the link need
    ///   to be approved by chat administrators;
    ///   if `true`, member_limit can't be specified.
    pub fn with_creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = Some(value);
        self
    }

    /// Sets a new expiration date.
    ///
    /// # Arguments
    ///
    /// * `value` - The point in time (Unix timestamp) when the link will expire.
    pub fn with_expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets a new member limit.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum number of users that can be members of the chat simultaneously
    ///   after joining the chat via this invite link; 1-99999.
    pub fn with_member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// Sets a new invite link name.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the invite link; 0-32 characters.
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

/// Creates a subscription invite link for a channel chat.
///
/// The bot must have the `can_invite_users` administrator rights.
/// The link can be edited using the method [`crate::types::EditChatSubscriptionInviteLink`]
/// or revoked using the method [`crate::types::RevokeChatInviteLink`].
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CreateChatSubscriptionInviteLink {
    chat_id: ChatId,
    subscription_period: Integer,
    subscription_price: Integer,
    name: Option<String>,
}

impl CreateChatSubscriptionInviteLink {
    /// Creates a new `CreateChatSubscriptionInviteLink`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target channel chat.
    /// * `subscription_period` - The number of seconds the subscription will be active for before the next payment.
    ///   Currently, it must always be 2592000 (30 days).
    /// * `subscription_price` - The amount of Telegram Stars a user must pay initially
    ///   and after each subsequent subscription period to be a member of the chat; 1-2500.
    pub fn new<T>(chat_id: T, subscription_period: Integer, subscription_price: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            name: None,
            subscription_period,
            subscription_price,
        }
    }

    /// Sets a new name.
    ///
    /// # Arguments
    ///
    /// * `value` - Invite link name; 0-32 characters.
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }
}

impl Method for CreateChatSubscriptionInviteLink {
    type Response = ChatInviteLink;

    fn into_payload(self) -> Payload {
        Payload::json("createChatSubscriptionInviteLink", self)
    }
}

/// Changes a non-primary invite link created by a bot.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
/// Returns the edited invite link as a [`ChatInviteLink`] object.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditChatInviteLink {
    chat_id: ChatId,
    invite_link: String,
    creates_join_request: Option<bool>,
    expire_date: Option<Integer>,
    member_limit: Option<Integer>,
    name: Option<String>,
}

impl EditChatInviteLink {
    /// Creates a new `EditChatInviteLink`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `invite_link` - The invite link to edit.
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

    /// Sets a new value for the `creates_join_request` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether users joining the chat via the link need
    ///   to be approved by chat administrators;
    ///   if `true`, `member_limit` can't be specified.
    pub fn with_creates_join_request(mut self, value: bool) -> Self {
        self.creates_join_request = Some(value);
        self
    }

    /// Sets a new expiration date.
    ///
    /// # Arguments
    ///
    /// * `value` - The point in time (Unix timestamp) when the link will expire.
    pub fn with_expire_date(mut self, value: Integer) -> Self {
        self.expire_date = Some(value);
        self
    }

    /// Sets a new member limit.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum number of users that can be members of the chat simultaneously
    ///   after joining the chat via this invite link; 1-99999.
    pub fn with_member_limit(mut self, value: Integer) -> Self {
        self.member_limit = Some(value);
        self
    }

    /// Sets a new name of the invite link.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the invite link; 0-32 characters.
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

/// Allows to edit a subscription invite link created by the bot.
///
/// The bot must have the `can_invite_users` administrator rights.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditChatSubscriptionInviteLink {
    chat_id: ChatId,
    invite_link: String,
    name: Option<String>,
}

impl EditChatSubscriptionInviteLink {
    /// Creates a new `EditChatSubscriptionInviteLink`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `invite_link` - The invite link to edit
    pub fn new<A, B>(chat_id: A, invite_link: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            name: None,
        }
    }

    /// Sets a new name of the invite link.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the invite link; 0-32 characters.
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }
}

impl Method for EditChatSubscriptionInviteLink {
    type Response = ChatInviteLink;

    fn into_payload(self) -> Payload {
        Payload::json("editChatSubscriptionInviteLink", self)
    }
}

/// Generates a new invite link for a chat.
///
/// Any previously generated link is revoked.
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights.
/// Returns the new invite link as String on success.
///
/// # Notes
///
/// Each administrator in a chat generates their own invite links.
/// Bots can't use invite links generated by other administrators.
/// If you want your bot to work with invite links, it will need to generate
/// its own link using [`ExportChatInviteLink`] or by calling the [`crate::types::GetChat`] method.
/// If your bot needs to generate a new primary invite link replacing its previous one,
/// use [`ExportChatInviteLink`] again.
#[derive(Clone, Debug, Serialize)]
pub struct ExportChatInviteLink {
    chat_id: ChatId,
}

impl ExportChatInviteLink {
    /// Creates a new `ExportChatInviteLink`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
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

/// Revokes an invite link created by a bot.
///
/// If the primary link is revoked, a new link is automatically generated.
/// The bot must be an administrator in the chat for this to work and
/// must have the appropriate admin rights.
/// Returns the revoked invite link as [`ChatInviteLink`] object.
#[derive(Clone, Debug, Serialize)]
pub struct RevokeChatInviteLink {
    chat_id: ChatId,
    invite_link: String,
}

impl RevokeChatInviteLink {
    /// Creates a new `RevokeChatInviteLink`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `invite_link` - The invite link to revoke.
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

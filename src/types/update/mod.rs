use std::{collections::HashSet, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{
    method::Method,
    request::Request,
    types::{
        CallbackQuery,
        Chat,
        ChatJoinRequest,
        ChatMemberUpdated,
        ChosenInlineResult,
        InlineQuery,
        Integer,
        Message,
        Poll,
        PollAnswer,
        PreCheckoutQuery,
        ShippingQuery,
        User,
    },
};

#[cfg(test)]
mod tests;

/// Incoming update
#[derive(Clone, Debug, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier
    ///
    /// Update identifiers start from a certain positive number and increase sequentially
    /// This ID becomes especially handy if you’re using Webhooks, since it allows you to
    /// ignore repeated updates or to restore the correct update sequence, should they get out of order
    /// If there are no new updates for at least a week, then identifier
    /// of the next update will be chosen randomly instead of sequentially
    #[serde(rename = "update_id")]
    pub id: Integer,
    /// Kind of update
    #[serde(flatten)]
    pub kind: UpdateKind,
}

impl Update {
    /// Returns a chat from update
    pub fn get_chat(&self) -> Option<&Chat> {
        self.get_message().map(|msg| &msg.chat).or(match self.kind {
            UpdateKind::BotStatus(ref status) | UpdateKind::UserStatus(ref status) => Some(&status.chat),
            UpdateKind::ChatJoinRequest(ref request) => Some(&request.chat),
            _ => None,
        })
    }

    /// Returns a chat ID from update
    pub fn get_chat_id(&self) -> Option<Integer> {
        self.get_chat().map(|chat| chat.get_id())
    }

    /// Returns a chat username from update
    pub fn get_chat_username(&self) -> Option<&str> {
        self.get_chat().and_then(|chat| chat.get_username())
    }

    /// Returns a user from update
    pub fn get_user(&self) -> Option<&User> {
        Some(match self.kind {
            UpdateKind::Message(ref msg)
            | UpdateKind::EditedMessage(ref msg)
            | UpdateKind::ChannelPost(ref msg)
            | UpdateKind::EditedChannelPost(ref msg) => return msg.sender.get_user(),
            UpdateKind::InlineQuery(ref query) => &query.from,
            UpdateKind::ChosenInlineResult(ref result) => &result.from,
            UpdateKind::CallbackQuery(ref query) => &query.from,
            UpdateKind::ShippingQuery(ref query) => &query.from,
            UpdateKind::PreCheckoutQuery(ref query) => &query.from,
            UpdateKind::Poll(_) => return None,
            UpdateKind::PollAnswer(ref answer) => &answer.user,
            UpdateKind::BotStatus(ref status) | UpdateKind::UserStatus(ref status) => &status.from,
            UpdateKind::ChatJoinRequest(ref request) => &request.from,
            UpdateKind::Unknown(_) => return None,
        })
    }

    /// Returns a user ID from update
    pub fn get_user_id(&self) -> Option<Integer> {
        self.get_user().map(|user| user.id)
    }

    /// Returns a user username from update
    pub fn get_user_username(&self) -> Option<&str> {
        self.get_user()
            .and_then(|user| user.username.as_ref())
            .map(String::as_str)
    }

    /// Returns a message from update
    pub fn get_message(&self) -> Option<&Message> {
        match self.kind {
            UpdateKind::Message(ref msg)
            | UpdateKind::EditedMessage(ref msg)
            | UpdateKind::ChannelPost(ref msg)
            | UpdateKind::EditedChannelPost(ref msg) => Some(msg),
            UpdateKind::CallbackQuery(ref query) => query.message.as_ref(),
            _ => None,
        }
    }
}

/// Kind of update
#[derive(Clone, Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc
    ChannelPost(Message),
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost(Message),
    /// New incoming inline query
    InlineQuery(InlineQuery),
    /// The result of an inline query that was chosen by a user and sent to their chat partner
    ///
    /// Please see our documentation on the feedback collecting
    /// for details on how to enable these updates for your bot
    ChosenInlineResult(ChosenInlineResult),
    /// New incoming callback query
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query
    ///
    /// Only for invoices with flexible price
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query
    ///
    /// Contains full information about checkout
    PreCheckoutQuery(PreCheckoutQuery),
    /// New poll state
    ///
    /// Bots receive only updates about polls, which are sent or stopped by the bot
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll
    ///
    /// Bots receive new votes only in polls that were sent by the bot itself
    PollAnswer(PollAnswer),
    /// The bot's chat member status was updated in a chat
    ///
    /// For private chats, this update is received only
    /// when the bot is blocked or unblocked by the user
    #[serde(rename = "my_chat_member")]
    BotStatus(ChatMemberUpdated),
    /// A chat member's status was updated in a chat
    ///
    /// The bot must be an administrator in the chat
    /// and must explicitly specify “chat_member” in the list
    /// of allowed_updates to receive these updates.
    #[serde(rename = "chat_member")]
    UserStatus(ChatMemberUpdated),

    /// A request to join the chat has been sent
    ///
    /// The bot must have the can_invite_users administrator right in the chat to receive these updates.
    ChatJoinRequest(ChatJoinRequest),

    /// Used for unknown update types
    ///
    /// For example, Telegram introduced a new update type,
    /// but it is not supported in old versions of tgbot
    Unknown(JsonValue),
}

/// Type of update to receive
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    /// Message
    Message,
    /// Edited message
    EditedMessage,
    /// Channel post
    ChannelPost,
    /// Edited channel post
    EditedChannelPost,
    /// Inline query
    InlineQuery,
    /// Chosen inline result
    ChosenInlineResult,
    /// Callback query
    CallbackQuery,
    /// Shipping query
    ShippingQuery,
    /// Pre checkout query
    PreCheckoutQuery,
    /// Poll
    Poll,
    /// Poll answer
    PollAnswer,
    /// Chat member status
    ChatMember,
}

/// Receive incoming updates using long polling
///
/// An Array of Update objects is returned
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<HashSet<AllowedUpdate>>,
}

impl Method for GetUpdates {
    type Response = Vec<Update>;

    fn into_request(self) -> Request {
        Request::json("getUpdates", self)
    }
}

impl GetUpdates {
    /// Identifier of the first update to be returned
    ///
    /// Must be greater by one than the highest among the identifiers of previously received updates
    /// By default, updates starting with the earliest unconfirmed update are returned
    /// An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id
    /// The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue
    /// All previous updates will forgotten
    pub fn offset(mut self, offset: Integer) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Limits the number of updates to be retrieved
    ///
    /// Values between 1—100 are accepted
    /// Defaults to 100
    pub fn limit(mut self, limit: Integer) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Timeout for long polling
    ///
    /// Defaults to 0, i.e. usual short polling
    /// Should be positive, short polling should be used for testing purposes only
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout.as_secs() as i64);
        self
    }

    /// List the types of updates you want your bot to receive
    ///
    /// For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types
    /// Specify an empty list to receive all updates regardless of type (default)
    /// If not specified, the previous setting will be used
    /// Please note that this parameter doesn't affect updates created before the call to the getUpdates,
    /// so unwanted updates may be received for a short period of time
    pub fn allowed_updates(mut self, allowed_updates: HashSet<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Adds a type of updates you want your bot to receive
    pub fn add_allowed_update(mut self, allowed_update: AllowedUpdate) -> Self {
        match self.allowed_updates {
            Some(ref mut updates) => {
                updates.insert(allowed_update);
            }
            None => {
                let mut updates = HashSet::new();
                updates.insert(allowed_update);
                self.allowed_updates = Some(updates);
            }
        };
        self
    }
}

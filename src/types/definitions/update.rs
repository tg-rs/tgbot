use std::{collections::HashSet, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{
    api::{Method, Payload},
    types::{
        BusinessConnection,
        BusinessMessagesDeleted,
        CallbackQuery,
        Chat,
        ChatBoostRemoved,
        ChatBoostUpdated,
        ChatJoinRequest,
        ChatMemberUpdated,
        ChatPeerId,
        ChatUsername,
        ChosenInlineResult,
        InlineQuery,
        Integer,
        MaybeInaccessibleMessage,
        Message,
        MessageReactionCountUpdated,
        MessageReactionUpdated,
        PaidMediaPurchased,
        Poll,
        PollAnswer,
        PollAnswerVoter,
        PreCheckoutQuery,
        ShippingQuery,
        User,
        UserPeerId,
        UserUsername,
    },
};

/// Represents an incoming update.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Update {
    /// Unique identifier of the update.
    ///
    /// Update identifiers start from a certain positive number and increase sequentially.
    /// This ID becomes especially handy if you’re using Webhooks, since it allows you to
    /// ignore repeated updates or to restore the correct update sequence,
    /// should they get out of order.
    /// If there are no new updates for at least a week, then identifier
    /// of the next update will be chosen randomly instead of sequentially.
    #[serde(rename = "update_id")]
    pub id: Integer,
    /// Type of the update.
    #[serde(flatten)]
    pub update_type: UpdateType,
}

impl Update {
    /// Creates a new `Update`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the update.
    /// * `update_type` - Type of the update.
    pub fn new(id: Integer, update_type: UpdateType) -> Self {
        Self { id, update_type }
    }

    /// Returns the chat.
    pub fn get_chat(&self) -> Option<&Chat> {
        self.get_message().map(|msg| &msg.chat).or(match &self.update_type {
            UpdateType::BotStatus(x) | UpdateType::UserStatus(x) => Some(&x.chat),
            UpdateType::DeletedBusinessMessages(x) => Some(&x.chat),
            UpdateType::ChatBoostRemoved(x) => Some(&x.chat),
            UpdateType::ChatBoostUpdated(x) => Some(&x.chat),
            UpdateType::ChatJoinRequest(x) => Some(&x.chat),
            UpdateType::MessageReaction(x) => Some(&x.chat),
            UpdateType::MessageReactionCount(x) => Some(&x.chat),
            _ => None,
        })
    }

    /// Returns the ID of the chat.
    pub fn get_chat_id(&self) -> Option<ChatPeerId> {
        self.get_chat().map(|chat| chat.get_id())
    }

    /// Returns the username of the chat.
    pub fn get_chat_username(&self) -> Option<&ChatUsername> {
        self.get_chat().and_then(|chat| chat.get_username())
    }

    /// Returns the user.
    pub fn get_user(&self) -> Option<&User> {
        Some(match &self.update_type {
            UpdateType::BotStatus(x) | UpdateType::UserStatus(x) => &x.from,
            UpdateType::BusinessConnection(x) => &x.user,
            UpdateType::CallbackQuery(x) => &x.from,
            UpdateType::ChatBoostRemoved(_) => return None,
            UpdateType::ChatBoostUpdated(_) => return None,
            UpdateType::ChatJoinRequest(x) => &x.from,
            UpdateType::ChosenInlineResult(x) => &x.from,
            UpdateType::DeletedBusinessMessages(_) => return None,
            UpdateType::InlineQuery(x) => &x.from,
            UpdateType::Message(x)
            | UpdateType::BusinessMessage(x)
            | UpdateType::EditedBusinessMessage(x)
            | UpdateType::EditedMessage(x)
            | UpdateType::ChannelPost(x)
            | UpdateType::EditedChannelPost(x) => return x.sender.get_user(),
            UpdateType::MessageReaction(x) => return x.user.as_ref(),
            UpdateType::MessageReactionCount(_) => return None,
            UpdateType::Poll(_) => return None,
            UpdateType::PollAnswer(x) => match &x.voter {
                PollAnswerVoter::User(x) => x,
                PollAnswerVoter::Chat(_) => return None,
            },
            UpdateType::PreCheckoutQuery(x) => &x.from,
            UpdateType::PurchasedPaidMedia(x) => &x.from,
            UpdateType::ShippingQuery(x) => &x.from,
            UpdateType::Unknown(_) => return None,
        })
    }

    /// Returns the ID of the user.
    pub fn get_user_id(&self) -> Option<UserPeerId> {
        self.get_user().map(|user| user.id)
    }

    /// Returns the username of the user.
    pub fn get_user_username(&self) -> Option<&UserUsername> {
        self.get_user().and_then(|user| user.username.as_ref())
    }

    /// Returns the message.
    pub fn get_message(&self) -> Option<&Message> {
        match &self.update_type {
            UpdateType::Message(msg)
            | UpdateType::BusinessMessage(msg)
            | UpdateType::EditedBusinessMessage(msg)
            | UpdateType::EditedMessage(msg)
            | UpdateType::ChannelPost(msg)
            | UpdateType::EditedChannelPost(msg) => Some(msg),
            UpdateType::CallbackQuery(query) => match &query.message {
                Some(MaybeInaccessibleMessage::Message(msg)) => Some(msg),
                _ => None,
            },
            _ => None,
        }
    }
}

/// Represents a type of an update.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateType {
    /// The bot chat member status was updated in a chat.
    ///
    /// For private chats, this update is received only
    /// when the bot is blocked or unblocked by the user.
    #[serde(rename = "my_chat_member")]
    BotStatus(Box<ChatMemberUpdated>),
    /// The bot was connected to or disconnected from a business account,
    /// or a user edited an existing connection with the bot.
    BusinessConnection(Box<BusinessConnection>),
    /// New non-service message from a connected business account.
    BusinessMessage(Box<Message>),
    /// A new incoming callback query.
    CallbackQuery(Box<CallbackQuery>),
    /// A new incoming channel post.
    ChannelPost(Box<Message>),
    /// A boost was removed from a chat.
    ///
    /// The bot must be an administrator in the chat to receive these updates.
    #[serde(rename = "removed_chat_boost")]
    ChatBoostRemoved(Box<ChatBoostRemoved>),
    /// A chat boost was added or changed.
    ///
    /// The bot must be an administrator in the chat to receive these updates.
    #[serde(rename = "chat_boost")]
    ChatBoostUpdated(Box<ChatBoostUpdated>),
    /// A request to join the chat has been sent.
    ///
    /// The bot must have the `can_invite_users` administrator right
    /// in the chat to receive these updates.
    ChatJoinRequest(Box<ChatJoinRequest>),
    /// The result of an inline query that was chosen by a user and sent to their chat partner.
    ///
    /// Please see our documentation on the [feedback collecting][1]
    /// for details on how to enable these updates for your bot.
    ///
    /// [1]: https://core.telegram.org/bots/inline#collecting-feedback
    ChosenInlineResult(Box<ChosenInlineResult>),
    /// Messages were deleted from a connected business account.
    DeletedBusinessMessages(Box<BusinessMessagesDeleted>),
    /// New version of a message from a connected business account.
    EditedBusinessMessage(Box<Message>),
    /// A new version of a channel post that is known to the bot and was edited.
    EditedChannelPost(Box<Message>),
    /// A new version of a message that is known to the bot and was edited.
    EditedMessage(Box<Message>),
    /// A new incoming [inline][1] query.
    ///
    /// [1]: https://core.telegram.org/bots/api#inline-mode
    InlineQuery(Box<InlineQuery>),
    /// A new incoming message.
    Message(Box<Message>),
    /// A reaction to a message was changed by a user.
    ///
    /// The bot must be an administrator in the chat
    /// and must explicitly specify [`AllowedUpdate::MessageReaction`]
    /// in the list of allowed_updates to receive these updates.
    ///
    /// The update isn't received for reactions set by bots.
    MessageReaction(Box<MessageReactionUpdated>),
    /// Reactions to a message with anonymous reactions were changed.
    ///
    /// The bot must be an administrator in the chat
    /// and must explicitly specify [`AllowedUpdate::MessageReactionCount`]
    /// in the list of allowed_updates to receive these updates.
    MessageReactionCount(Box<MessageReactionCountUpdated>),
    /// A new poll state.
    ///
    /// Bots receive only updates about polls, which are sent or stopped by the bot.
    Poll(Box<Poll>),
    /// A user changed their answer in a non-anonymous poll
    ///
    /// Bots receive new votes only in polls that were sent by the bot itself.
    PollAnswer(Box<PollAnswer>),
    /// A new incoming pre-checkout query.
    ///
    /// Contains full information about checkout
    PreCheckoutQuery(Box<PreCheckoutQuery>),
    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat.
    PurchasedPaidMedia(Box<PaidMediaPurchased>),
    /// A new incoming shipping query.
    ///
    /// Only for invoices with flexible price.
    ShippingQuery(Box<ShippingQuery>),
    /// A chat member's status was updated in a chat.
    ///
    /// The bot must be an administrator in the chat
    /// and must explicitly specify [`AllowedUpdate::UserStatus`] in the list
    /// of `allowed_updates` to receive these updates.
    #[serde(rename = "chat_member")]
    UserStatus(Box<ChatMemberUpdated>),
    /// Used for unknown update types.
    ///
    /// For example, Telegram introduced a new update type,
    /// but tgbot does not support it.
    #[serde(untagged)]
    Unknown(JsonValue),
}

/// Conversion of an [`Update`] into `T` failed.
///
/// Use [`Update::from`] to get the original update.
pub struct UnexpectedUpdate(Update);

impl From<UnexpectedUpdate> for Update {
    fn from(value: UnexpectedUpdate) -> Self {
        value.0
    }
}

impl TryFrom<Update> for BusinessConnection {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            BusinessConnection(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for BusinessMessagesDeleted {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            DeletedBusinessMessages(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for ChatMemberUpdated {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            BotStatus(x) | UserStatus(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for CallbackQuery {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            CallbackQuery(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for ChatJoinRequest {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            ChatJoinRequest(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for ChosenInlineResult {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            ChosenInlineResult(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for InlineQuery {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            InlineQuery(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for Message {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            BusinessMessage(x)
            | EditedBusinessMessage(x)
            | EditedChannelPost(x)
            | EditedMessage(x)
            | ChannelPost(x)
            | Message(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for Poll {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            Poll(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for PollAnswer {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            PollAnswer(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for PreCheckoutQuery {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            PreCheckoutQuery(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for PaidMediaPurchased {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            PurchasedPaidMedia(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

impl TryFrom<Update> for ShippingQuery {
    type Error = UnexpectedUpdate;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        use self::UpdateType::*;
        match value.update_type {
            ShippingQuery(x) => Ok(*x),
            _ => Err(UnexpectedUpdate(value)),
        }
    }
}

/// Represents a type of update to receive.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    /// The bot chat member status.
    #[serde(rename = "my_chat_member")]
    BotStatus,
    /// Business account connection changes.
    BusinessConnection,
    /// New non-service message from a connected business account.
    BusinessMessage,
    /// A callback query.
    CallbackQuery,
    /// A channel post.
    ChannelPost,
    /// A boost was removed from a chat.
    #[serde(rename = "removed_chat_boost")]
    ChatBoostRemoved,
    /// A chat boost was added or changed.
    #[serde(rename = "chat_boost")]
    ChatBoostUpdated,
    /// A request to join a chat.
    ChatJoinRequest,
    /// A chosen inline result.
    ChosenInlineResult,
    /// Messages were deleted from a connected business account.
    DeletedBusinessMessages,
    /// New version of a message from a connected business account.
    EditedBusinessMessage,
    /// An edited channel post.
    EditedChannelPost,
    /// An edited message.
    EditedMessage,
    /// An inline query.
    InlineQuery,
    /// A message.
    Message,
    /// A reaction to a message.
    MessageReaction,
    /// An anonymous reaction to a message.
    MessageReactionCount,
    /// A poll.
    Poll,
    /// A poll answer.
    PollAnswer,
    /// A pre checkout query.
    PreCheckoutQuery,
    /// A user purchased paid media.
    PurchasedPaidMedia,
    /// A shipping query.
    ShippingQuery,
    /// A chat member status.
    #[serde(rename = "chat_member")]
    UserStatus,
}

#[serde_with::skip_serializing_none]
/// Returns incoming updates using long polling.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUpdates {
    allowed_updates: Option<HashSet<AllowedUpdate>>,
    limit: Option<Integer>,
    offset: Option<Integer>,
    timeout: Option<Integer>,
}

impl Method for GetUpdates {
    type Response = Vec<Update>;

    fn into_payload(self) -> Payload {
        Payload::json("getUpdates", self)
    }
}

impl GetUpdates {
    /// Adds a type of an update you want your bot to receive.
    ///
    /// # Arguments
    ///
    /// * `value` - The type to add.
    pub fn add_allowed_update(mut self, value: AllowedUpdate) -> Self {
        match self.allowed_updates {
            Some(ref mut updates) => {
                updates.insert(value);
            }
            None => {
                let mut updates = HashSet::new();
                updates.insert(value);
                self.allowed_updates = Some(updates);
            }
        };
        self
    }

    /// Sets a new list of allowed updates.
    ///
    /// # Arguments
    ///
    /// * `value` - List of the types of updates you want your bot to receive.
    ///
    /// For example, specify `[AllowedUpdate::Message, AllowedUpdate::EditedChannelPost]`
    /// to only receive updates of these types.
    /// Specify an empty list to receive all updates regardless of type (default).
    /// If not specified, the previous setting will be used.
    /// Please note that this parameter doesn't affect updates
    /// created before the call to the method,
    /// so unwanted updates may be received for a short period of time.
    pub fn with_allowed_updates<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = AllowedUpdate>,
    {
        self.allowed_updates = Some(value.into_iter().collect());
        self
    }

    /// Sets a new limit.
    ///
    /// # Arguments
    ///
    /// * `value` - Limit of the number of updates to be retrieved; 1—100; default - 100.
    pub fn with_limit(mut self, value: Integer) -> Self {
        self.limit = Some(value);
        self
    }

    /// Sets a new offset.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the first update to be returned.
    ///
    /// Must be greater by one than the highest
    /// among the identifiers of previously received updates.
    /// By default, updates starting with the earliest unconfirmed update are returned.
    /// An update is considered confirmed as soon as the method is called with
    /// an offset higher than its `update_id`.
    /// The negative offset can be specified to retrieve updates starting
    /// from `-offset` update from the end of the updates queue.
    /// All previous updates will forgotten.
    pub fn with_offset(mut self, value: Integer) -> Self {
        self.offset = Some(value);
        self
    }

    /// Sets a new timeout.
    ///
    /// # Arguments
    ///
    /// * `value` - Timeout for long polling; default - 0, i.e. usual short polling; should be positive;
    ///   short polling should be used for testing purposes only.
    pub fn with_timeout(mut self, value: Duration) -> Self {
        self.timeout = Some(value.as_secs() as i64);
        self
    }
}

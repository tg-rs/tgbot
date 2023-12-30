use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    api::{Method, Payload},
    types::{Chat, ChatId, Integer, User},
};

#[cfg(test)]
mod tests;

/// Contains information about a chat boost.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatBoost {
    /// Point in time (Unix timestamp) when the chat was boosted.
    pub add_date: Integer,
    /// Unique identifier of the boost.
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the boost will automatically expire,
    /// unless the booster's Telegram Premium subscription is prolonged.
    pub expiration_date: Integer,
    /// Source of the added boost.
    pub source: ChatBoostSource,
}

impl ChatBoost {
    /// Creates a new `ChatBoost`.
    ///
    /// # Arguments
    ///
    /// * `add_date` - Point in time (Unix timestamp) when the chat was boosted.
    /// * `boost_id` - Unique identifier of the boost.
    /// * `expiration_date` - Point in time (Unix timestamp) when the boost will automatically expire,
    ///                       unless the booster's Telegram Premium subscription is prolonged.
    /// * `source` - Source of the added boost.
    pub fn new<T>(add_date: Integer, boost_id: T, expiration_date: Integer, source: ChatBoostSource) -> Self
    where
        T: Into<String>,
    {
        Self {
            add_date,
            boost_id: boost_id.into(),
            expiration_date,
            source,
        }
    }
}

/// Represents a boost removed from a chat.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatBoostRemoved {
    /// Unique identifier of the boost.
    pub boost_id: String,
    /// Chat which was boosted.
    pub chat: Chat,
    /// Point in time (Unix timestamp) when the boost was removed.
    pub remove_date: Integer,
    /// Source of the removed boost.
    pub source: ChatBoostSource,
}

impl ChatBoostRemoved {
    /// Creates a new `ChatBoostRemoved`.
    ///
    /// # Arguments
    ///
    /// * `boost_id` - Unique identifier of the boost.
    /// * `chat` - Chat which was boosted.
    /// * `remove_date` - Point in time (Unix timestamp) when the boost was removed.
    /// * `source` - Source of the removed boost.
    pub fn new<A, B>(boost_id: A, chat: B, remove_date: Integer, source: ChatBoostSource) -> Self
    where
        A: Into<String>,
        B: Into<Chat>,
    {
        Self {
            boost_id: boost_id.into(),
            chat: chat.into(),
            remove_date,
            source,
        }
    }
}

/// Describes the source of a chat boost.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "source")]
pub enum ChatBoostSource {
    /// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat.
    ///
    /// Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
    /// Contains a user for which the gift code was created.
    #[serde(
        deserialize_with = "ChatBoostSourceUser::deserialize_value",
        serialize_with = "ChatBoostSourceUser::serialize_value"
    )]
    GiftCode(User),
    /// The boost was obtained by the creation of a Telegram Premium giveaway.
    Giveaway(ChatBoostSourceGiveaway),
    /// The boost was obtained by subscribing to Telegram Premium
    /// or by gifting a Telegram Premium subscription to another user.
    ///
    /// Contains a user that boosted the chat
    #[serde(
        deserialize_with = "ChatBoostSourceUser::deserialize_value",
        serialize_with = "ChatBoostSourceUser::serialize_value"
    )]
    Premium(User),
}

#[derive(Deserialize, Serialize)]
struct ChatBoostSourceUser {
    user: User,
}

impl ChatBoostSourceUser {
    pub(super) fn deserialize_value<'de, D>(deserializer: D) -> Result<User, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::deserialize(deserializer).map(|x| x.user)
    }

    pub(super) fn serialize_value<S>(value: &User, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self { user: value.clone() }.serialize(serializer)
    }
}

/// The boost was obtained by the creation of a Telegram Premium giveaway.
///
/// This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a message in the chat with the giveaway;
    /// the message could have been deleted already.
    /// May be 0 if the message isn't sent yet.
    pub giveaway_message_id: Integer,
    /// Whether the giveaway was completed, but there was no user to win the prize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<bool>,
    /// User that won the prize in the giveaway if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl ChatBoostSourceGiveaway {
    /// Creates a new `ChatBoostSourceGiveaway`.
    ///
    /// # Arguments
    ///
    /// * `giveaway_message_id` - Identifier of a message in the chat with the giveaway.
    pub fn new(giveaway_message_id: Integer) -> Self {
        Self {
            giveaway_message_id,
            is_unclaimed: None,
            user: None,
        }
    }

    /// Sets a new value for an `is_unclaimed` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the giveaway was completed, but there was no user to win the prize.
    pub fn with_is_unclaimed(mut self, value: bool) -> Self {
        self.is_unclaimed = Some(value);
        self
    }

    /// Sets a new user
    ///
    /// # Arguments
    ///
    /// * `value` - User that won the prize in the giveaway.
    pub fn with_user(mut self, value: User) -> Self {
        self.user = Some(value);
        self
    }
}

/// Represents a boost added to a chat or changed.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatBoostUpdated {
    /// Infomation about the chat boost.
    pub boost: ChatBoost,
    /// Chat which was boosted.
    pub chat: Chat,
}

impl ChatBoostUpdated {
    /// Creates a new `ChatBoostUpdated`.
    ///
    /// # Arguments
    ///
    /// * `boost` - Infomation about the chat boost.
    /// * `chat` - Chat which was boosted.
    pub fn new<T>(boost: ChatBoost, chat: T) -> Self
    where
        T: Into<Chat>,
    {
        Self {
            boost,
            chat: chat.into(),
        }
    }
}

/// Represents a list of boosts added to a chat by a user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user.
    pub boosts: Vec<ChatBoost>,
}

impl<T> From<T> for UserChatBoosts
where
    T: IntoIterator<Item = ChatBoost>,
{
    fn from(value: T) -> Self {
        Self {
            boosts: value.into_iter().collect(),
        }
    }
}

/// Returns the list of boosts added to a chat by a user.
///
/// Requires administrator rights in the chat.
#[derive(Clone, Debug, Serialize)]
pub struct GetUserChatBoosts {
    chat_id: ChatId,
    user_id: Integer,
}

impl GetUserChatBoosts {
    /// Creates a new `GetUserChatBoosts`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the chat.
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

impl Method for GetUserChatBoosts {
    type Response = UserChatBoosts;

    fn into_payload(self) -> Payload {
        Payload::json("getUserChatBoosts", self)
    }
}

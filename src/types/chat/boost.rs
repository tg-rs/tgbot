use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    api::{Method, Payload, PayloadError},
    types::{Chat, ChatId, Integer, User},
};

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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a message in the chat with the giveaway;
    /// the message could have been deleted already.
    /// May be 0 if the message isn't sent yet.
    pub giveaway_message_id: Integer,
    /// Whether the giveaway was completed, but there was no user to win the prize.
    pub is_unclaimed: Option<bool>,
    /// The number of Telegram Stars to be split between giveaway winners;
    /// for Telegram Star giveaways only.
    pub prize_star_count: Option<Integer>,
    /// User that won the prize in the giveaway if any.
    pub user: Option<User>,
}

/// Represents a boost added to a chat or changed.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatBoostUpdated {
    /// Infomation about the chat boost.
    pub boost: ChatBoost,
    /// Chat which was boosted.
    pub chat: Chat,
}

/// Represents a list of boosts added to a chat by a user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user.
    pub boosts: Vec<ChatBoost>,
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

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("getUserChatBoosts", self)
    }
}

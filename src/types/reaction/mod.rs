use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Chat, ChatId, Integer, User},
};

#[cfg(test)]
mod tests;

/// Represents a reaction added to a message along with the number of times it was added.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReactionCount {
    /// Type of the reaction.
    #[serde(rename = "type")]
    pub reaction_type: ReactionType,
    /// Number of times the reaction was added.
    pub total_count: Integer,
}

impl ReactionCount {
    /// Creates a new `ReactionCount`.
    ///
    /// # Arguments
    ///
    /// * `reaction_type` - Type of the reaction.
    /// * `total_count` - Number of times the reaction was added.
    pub fn new(reaction_type: ReactionType, total_count: Integer) -> Self {
        Self {
            reaction_type,
            total_count,
        }
    }
}

/// Represents reaction changes on a message with anonymous reactions.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message.
    pub chat: Chat,
    /// Date of the change in Unix time.
    pub date: Integer,
    /// Unique message identifier inside the chat.
    pub message_id: Integer,
    /// List of reactions that are present on the message.
    pub reactions: Vec<ReactionCount>,
}

impl MessageReactionCountUpdated {
    /// Creates a new `MessageReactionCountUpdated`.
    ///
    /// # Arguments
    ///
    /// * `chat` - The chat containing the message.
    /// * `date` - Date of the change in Unix time.
    /// * `message_id` - Unique message identifier inside the chat.
    /// * `reactions` - List of reactions that are present on the message.
    pub fn new<A, B>(chat: A, date: Integer, message_id: Integer, reactions: B) -> Self
    where
        A: Into<Chat>,
        B: IntoIterator<Item = ReactionCount>,
    {
        Self {
            chat: chat.into(),
            date,
            message_id,
            reactions: reactions.into_iter().collect(),
        }
    }
}

/// Represents a reaction type.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(from = "RawReactionType", into = "RawReactionType")]
pub enum ReactionType {
    /// A reaction based on a custom emoji.
    CustomEmoji(String),
    /// A reaction based on a predefined emoji.
    Emoji(String),
    /// The reaction is paid.
    Paid,
}

impl ReactionType {
    /// Creates a new `ReactionType` based on a custom emoji.
    ///
    /// # Arguments
    ///
    /// `value` - A custom emoji.
    pub fn custom_emoji<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::CustomEmoji(value.into())
    }

    /// Creates a new `ReactionType` based on a predefined emoji.
    ///
    /// # Arguments
    ///
    /// `value` - A predefined emoji.
    pub fn emoji<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Emoji(value.into())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum RawReactionType {
    CustomEmoji { custom_emoji: String },
    Emoji { emoji: String },
    Paid,
}

impl From<ReactionType> for RawReactionType {
    fn from(value: ReactionType) -> Self {
        match value {
            ReactionType::CustomEmoji(custom_emoji) => Self::CustomEmoji { custom_emoji },
            ReactionType::Emoji(emoji) => Self::Emoji { emoji },
            ReactionType::Paid => Self::Paid,
        }
    }
}

impl From<RawReactionType> for ReactionType {
    fn from(value: RawReactionType) -> Self {
        match value {
            RawReactionType::CustomEmoji { custom_emoji } => Self::CustomEmoji(custom_emoji),
            RawReactionType::Emoji { emoji } => Self::Emoji(emoji),
            RawReactionType::Paid => Self::Paid,
        }
    }
}

/// Represents a change of a reaction on a message performed by a user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to.
    pub chat: Chat,
    /// Date of the change in Unix time.
    pub date: Integer,
    /// Unique identifier of the message inside the chat.
    pub message_id: Integer,
    /// New list of reaction types that have been set by the user.
    pub new_reaction: Vec<ReactionType>,
    /// Previous list of reaction types that were set by the user.
    pub old_reaction: Vec<ReactionType>,
    /// The chat on behalf of which the reaction was changed, if the user is anonymous.
    pub actor_chat: Option<Chat>,
    /// The user that changed the reaction, if the user isn't anonymous.
    pub user: Option<User>,
}

impl MessageReactionUpdated {
    /// Creates a new `MessageReactionUpdated`.
    ///
    ///  # Arguments
    ///
    /// * `chat` - The chat containing the message the user reacted to.
    /// * `date` - Date of the change in Unix time.
    /// * `message_id` - Unique identifier of the message inside the chat.
    /// * `new_reaction` - New list of reaction types that have been set by the user.
    /// * `old_reaction` - Previous list of reaction types that were set by the user.
    pub fn new<A, B, C>(chat: A, date: Integer, message_id: Integer, new_reaction: B, old_reaction: C) -> Self
    where
        A: Into<Chat>,
        B: IntoIterator<Item = ReactionType>,
        C: IntoIterator<Item = ReactionType>,
    {
        Self {
            chat: chat.into(),
            date,
            message_id,
            new_reaction: new_reaction.into_iter().collect(),
            old_reaction: old_reaction.into_iter().collect(),
            actor_chat: None,
            user: None,
        }
    }

    /// Sets a new actor chat.
    ///
    /// # Arguments
    ///
    /// * `value` - The chat on behalf of which the reaction was changed, if the user is anonymous.
    pub fn with_actor_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.actor_chat = Some(value.into());
        self
    }

    /// Sets a new user.
    ///
    /// # Arguments
    ///
    /// * `value` - The user that changed the reaction, if the user isn't anonymous.
    pub fn with_user(mut self, value: User) -> Self {
        self.user = Some(value);
        self
    }
}

/// Changes the chosen reactions on a message.
///
/// Service messages of some types can't be reacted to.
/// Automatically forwarded messages from a channel to its discussion group have
/// the same available reactions as messages in the channel.
/// Bots can't use paid reactions.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetMessageReaction {
    chat_id: ChatId,
    is_big: bool,
    message_id: Integer,
    reaction: Option<Vec<ReactionType>>,
}

impl SetMessageReaction {
    /// Creates a new `SetMessageReaction`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_id` - Identifier of the target message.
    pub fn new<T>(chat_id: T, message_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reaction: None,
            is_big: false,
        }
    }

    /// Sets a new value for the `is_big` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to set the reaction with a big animation.
    pub fn with_is_big(mut self, value: bool) -> Self {
        self.is_big = value;
        self
    }

    /// Sets a new list of reaction types.
    ///
    /// # Arguments
    ///
    /// * `value` - New list of reaction types to set on the message.
    ///
    /// Currently, as non-premium users, bots can set up to one reaction per message.
    /// A custom emoji reaction can be used if it is either already present on
    /// the message or explicitly allowed by chat administrators.
    pub fn with_reaction<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = ReactionType>,
    {
        self.reaction = Some(value.into_iter().collect());
        self
    }
}

impl Method for SetMessageReaction {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setMessageReaction", self)
    }
}

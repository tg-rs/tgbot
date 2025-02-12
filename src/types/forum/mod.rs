use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, ForumTopicIconColor, Integer, Sticker},
};

#[cfg(test)]
mod tests;

/// Represents a forum topic.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ForumTopic {
    /// Color of the icon.
    pub icon_color: ForumTopicIconColor,
    /// Unique identifier.
    pub message_thread_id: Integer,
    /// Name.
    pub name: String,
    /// Unique identifier of the custom emoji shown as the topic icon.
    pub icon_custom_emoji_id: Option<String>,
}

impl ForumTopic {
    /// Creates a new `ForumTopic`.
    ///
    /// # Arguments
    ///
    /// * `icon_color` - Color of the icon.
    /// * `message_thread_id` - Unique identifier of the topic.
    /// * `name` - Name of the topic.
    pub fn new<A, B>(icon_color: A, message_thread_id: Integer, name: B) -> Self
    where
        A: Into<ForumTopicIconColor>,
        B: Into<String>,
    {
        Self {
            icon_color: icon_color.into(),
            message_thread_id,
            name: name.into(),
            icon_custom_emoji_id: None,
        }
    }

    /// Sets a new icon custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Emoji ID.
    pub fn with_icon_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.icon_custom_emoji_id = Some(value.into());
        self
    }
}

/// Closes an open topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_manage_topics` administrator rights,
/// unless it is the creator of the topic.
#[derive(Clone, Debug, Serialize)]
pub struct CloseForumTopic {
    chat_id: ChatId,
    message_thread_id: Integer,
}

impl CloseForumTopic {
    /// Creates a new `CloseForumTopic`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_thread_id` - Unique identifier of the target message thread of the forum topic.
    pub fn new<T>(chat_id: T, message_thread_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }
}

impl Method for CloseForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("closeForumTopic", self)
    }
}

/// Creates a topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CreateForumTopic {
    chat_id: ChatId,
    name: String,
    icon_color: Option<ForumTopicIconColor>,
    icon_custom_emoji_id: Option<String>,
}

impl CreateForumTopic {
    /// Creates a new `CreateForumTopic`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `name` - Topic name; 1 - 128 characters.
    pub fn new<A, B>(chat_id: A, name: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            name: name.into(),
            icon_color: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Sets a new color of the topic icon.
    ///
    /// # Arguments
    ///
    /// * `value` - Color of the topic icon.
    pub fn with_icon_color(mut self, value: ForumTopicIconColor) -> Self {
        self.icon_color = Some(value);
        self
    }

    /// Sets a new icon custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the custom emoji shown as the topic icon.
    ///
    /// Use [`GetForumTopicIconStickers`] to get all allowed custom emoji identifiers.
    pub fn with_icon_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.icon_custom_emoji_id = Some(value.into());
        self
    }
}

impl Method for CreateForumTopic {
    type Response = ForumTopic;

    fn into_payload(self) -> Payload {
        Payload::json("createForumTopic", self)
    }
}

/// Closes an opened 'General' topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct CloseGeneralForumTopic {
    chat_id: ChatId,
}

impl CloseGeneralForumTopic {
    /// Creates a new `CloseGeneralForumTopic`.
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

impl Method for CloseGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("closeGeneralForumTopic", self)
    }
}

/// Deletes a forum topic along with all its messages in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_delete_messages administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteForumTopic {
    chat_id: ChatId,
    message_thread_id: Integer,
}

impl DeleteForumTopic {
    /// Creates a new `DeleteForumTopic`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_thread_id` - Unique identifier of the target message thread of the forum topic.
    pub fn new<T>(chat_id: T, message_thread_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }
}

impl Method for DeleteForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteForumTopic", self)
    }
}

/// Changes name and icon of a topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have can_manage_topics administrator rights,
/// unless it is the creator of the topic.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct EditForumTopic {
    chat_id: ChatId,
    message_thread_id: Integer,
    icon_custom_emoji_id: Option<String>,
    name: Option<String>,
}

impl EditForumTopic {
    /// Creates a new `EditForumTopic`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target.
    /// * `message_thread_id` - Unique identifier of the target message thread of the forum topic.
    pub fn new<C>(chat_id: C, message_thread_id: Integer) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
            icon_custom_emoji_id: None,
            name: None,
        }
    }

    /// Sets a new icon custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - New unique identifier of the custom emoji shown as the topic icon.
    ///
    /// Use [`GetForumTopicIconStickers`] to get all allowed custom emoji identifiers.
    /// Pass an empty string to remove the icon.
    /// If not specified, the current icon will be kept.
    pub fn with_icon_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.icon_custom_emoji_id = Some(value.into());
        self
    }

    /// Sets a new name.
    ///
    /// # Arguments
    ///
    /// * `value` - New topic name; 0-128 characters.
    ///
    /// If not specified or empty, the current name of the topic will be kept.
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }
}

impl Method for EditForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("editForumTopic", self)
    }
}

/// Changes the name of the 'General' topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have `can_manage_topics` administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct EditGeneralForumTopic {
    chat_id: ChatId,
    name: String,
}

impl EditGeneralForumTopic {
    /// Creates a new `EditGeneralForumTopic`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `name` - New topic name, 1-128 characters.
    pub fn new<A, B>(chat_id: A, name: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            name: name.into(),
        }
    }
}

impl Method for EditGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("editGeneralForumTopic", self)
    }
}

/// Returns custom emoji stickers, which can be used as a forum topic icon by any user.
#[derive(Clone, Copy, Debug)]
pub struct GetForumTopicIconStickers;

impl Method for GetForumTopicIconStickers {
    type Response = Vec<Sticker>;

    fn into_payload(self) -> Payload {
        Payload::empty("getForumTopicIconStickers")
    }
}

/// Hides the 'General' topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_manage_topics` administrator rights.
/// The topic will be automatically closed if it was open.
#[derive(Clone, Debug, Serialize)]
pub struct HideGeneralForumTopic {
    chat_id: ChatId,
}

impl HideGeneralForumTopic {
    /// Creates a new `HideGeneralForumTopic`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for HideGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("hideGeneralForumTopic", self)
    }
}

/// Reopens a closed topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_manage_topics` administrator rights,
/// unless it is the creator of the topic.
#[derive(Clone, Debug, Serialize)]
pub struct ReopenForumTopic {
    chat_id: ChatId,
    message_thread_id: Integer,
}

impl ReopenForumTopic {
    /// Creates a new `ReopenForumTopic`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_thread_id` - Unique identifier of the target message thread of the forum topic.
    pub fn new<T>(chat_id: T, message_thread_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }
}

impl Method for ReopenForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("reopenForumTopic", self)
    }
}

/// Reopens a closed 'General' topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_manage_topics` administrator rights.
/// The topic will be automatically unhidden if it was hidden.
#[derive(Clone, Debug, Serialize)]
pub struct ReopenGeneralForumTopic {
    chat_id: ChatId,
}

impl ReopenGeneralForumTopic {
    /// Creates a new `ReopenGeneralForumTopic`.
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

impl Method for ReopenGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("reopenGeneralForumTopic", self)
    }
}

/// Reveals the 'General' topic in a forum supergroup chat.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_manage_topics` administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct UnhideGeneralForumTopic {
    chat_id: ChatId,
}

impl UnhideGeneralForumTopic {
    /// Creates a new `UnhideGeneralForumTopic`.
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

impl Method for UnhideGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("unhideGeneralForumTopic", self)
    }
}

/// Clears the list of pinned messages in a forum topic.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_pin_messages` administrator right in the supergroup.
#[derive(Clone, Debug, Serialize)]
pub struct UnpinAllForumTopicMessages {
    chat_id: ChatId,
    message_thread_id: Integer,
}

impl UnpinAllForumTopicMessages {
    /// Creates a new `UnpinAllForumTopicMessages`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `message_thread_id` - Unique identifier of the target message thread of the forum topic.
    pub fn new<T>(chat_id: T, message_thread_id: Integer) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }
}

impl Method for UnpinAllForumTopicMessages {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("unpinAllForumTopicMessages", self)
    }
}

/// Clears the list of pinned messages in a General forum topic.
///
/// The bot must be an administrator in the chat for this to work
/// and must have the `can_pin_messages` administrator right in the supergroup.
#[derive(Clone, Debug, Serialize)]
pub struct UnpinAllGeneralForumTopicMessages {
    chat_id: ChatId,
}

impl UnpinAllGeneralForumTopicMessages {
    /// Creates a new `UnpinAllGeneralForumTopicMessages`.
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

impl Method for UnpinAllGeneralForumTopicMessages {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("unpinAllGeneralForumTopicMessages", self)
    }
}

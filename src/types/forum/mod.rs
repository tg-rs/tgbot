use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, Sticker},
};

#[cfg(test)]
mod tests;

/// Represents a forum topic
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: Integer,
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: Integer,
    /// Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Represents a service message about a forum topic closed in the chat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ForumTopicClosed {}

/// Represents a service message about a new forum topic created in the chat.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: Integer,
    /// Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Represents a service message about an edited forum topic.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ForumTopicEdited {
    /// New name of the topic, if it was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New identifier of the custom emoji shown as the topic icon,
    /// if it was edited; an empty string if the icon was removed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Represents a service message about a forum topic closed in the chat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ForumTopicReopened {}

/// Represents a service message about General forum topic hidden in the chat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct GeneralForumTopicHidden {}

/// Represents a service message about General forum topic unhidden in the chat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct GeneralForumTopicUnhidden {}

/// Close an open topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights, unless it is the creator of the topic.
#[derive(Clone, Debug, Serialize)]
pub struct CloseForumTopic {
    chat_id: ChatId,
    message_thread_id: Integer,
}

impl CloseForumTopic {
    /// Creates a new CloseForumTopic
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_thread_id - Unique identifier for the target message thread of the forum topic
    pub fn new<C>(chat_id: C, message_thread_id: Integer) -> Self
    where
        C: Into<ChatId>,
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

/// Create a topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct CreateForumTopic {
    chat_id: ChatId,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_color: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<String>,
}

impl CreateForumTopic {
    /// Creates a new CreateForumTopic
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * name - Topic name, 1 - 128 characters
    pub fn new<C, N>(chat_id: C, name: N) -> Self
    where
        C: Into<ChatId>,
        N: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            name: name.into(),
            icon_color: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Color of the topic icon in RGB format
    ///
    /// Currently, must be one of
    /// 7322096 (0x6FB9F0),
    /// 16766590 (0xFFD67E),
    /// 13338331 (0xCB86DB),
    /// 9367192 (0x8EEE98),
    /// 16749490 (0xFF93B2),
    /// 16478047 (0xFB6F5F)
    pub fn icon_color(mut self, value: Integer) -> Self {
        self.icon_color = Some(value);
        self
    }

    /// Unique identifier of the custom emoji shown as the topic icon
    ///
    /// Use getForumTopicIconStickers to get all allowed custom emoji identifiers
    pub fn icon_custom_emoji_id<T>(mut self, value: T) -> Self
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

/// Close an open 'General' topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct CloseGeneralForumTopic {
    chat_id: ChatId,
}

impl CloseGeneralForumTopic {
    /// Creates a new CloseGeneralForumTopic
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
        }
    }
}

impl Method for CloseGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("closeGeneralForumTopic", self)
    }
}

/// Delete a forum topic along with all its messages in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_delete_messages administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteForumTopic {
    chat_id: ChatId,
    message_thread_id: Integer,
}

impl DeleteForumTopic {
    /// Creates a new DeleteForumTopic
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_thread_id - Unique identifier for the target message thread of the forum topic
    pub fn new<C>(chat_id: C, message_thread_id: Integer) -> Self
    where
        C: Into<ChatId>,
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

/// Edit name and icon of a topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have can_manage_topics administrator rights,
/// unless it is the creator of the topic.
#[derive(Clone, Debug, Serialize)]
pub struct EditForumTopic {
    chat_id: ChatId,
    message_thread_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<String>,
}

impl EditForumTopic {
    /// Creates a new EditForumTopic
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target
    /// * message_thread_id - Unique identifier for the target message thread of the forum topic
    pub fn new<C>(chat_id: C, message_thread_id: Integer) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
            name: None,
            icon_custom_emoji_id: None,
        }
    }

    /// New topic name, 0-128 characters
    ///
    /// If not specified or empty, the current name of the topic will be kept
    pub fn name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }

    /// New unique identifier of the custom emoji shown as the topic icon
    ///
    /// Use getForumTopicIconStickers to get all allowed custom emoji identifiers.
    /// Pass an empty string to remove the icon.
    /// If not specified, the current icon will be kept.
    pub fn icon_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.icon_custom_emoji_id = Some(value.into());
        self
    }
}

impl Method for EditForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("editForumTopic", self)
    }
}

/// Edit the name of the 'General' topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have can_manage_topics administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct EditGeneralForumTopic {
    chat_id: ChatId,
    name: String,
}

impl EditGeneralForumTopic {
    /// Creates a new EditGeneralForumTopic
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * name - New topic name, 1-128 characters
    pub fn new<C, N>(chat_id: C, name: N) -> Self
    where
        C: Into<ChatId>,
        N: Into<String>,
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

/// Get custom emoji stickers, which can be used as a forum topic icon by any user
#[derive(Clone, Copy, Debug)]
pub struct GetForumTopicIconStickers;

impl Method for GetForumTopicIconStickers {
    type Response = Vec<Sticker>;

    fn into_payload(self) -> Payload {
        Payload::empty("getForumTopicIconStickers")
    }
}

/// Hide the 'General' topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights.
/// The topic will be automatically closed if it was open.
#[derive(Clone, Debug, Serialize)]
pub struct HideGeneralForumTopic {
    chat_id: ChatId,
}

impl HideGeneralForumTopic {
    /// Creates a new HideGeneralForumTopic
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
        }
    }
}

impl Method for HideGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("hideGeneralForumTopic", self)
    }
}

/// Reopen a closed topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights, unless it is the creator of the topic.
#[derive(Clone, Debug, Serialize)]
pub struct ReopenForumTopic {
    chat_id: ChatId,
    message_thread_id: Integer,
}

impl ReopenForumTopic {
    /// Creates a new ReopenForumTopic
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_thread_id - Unique identifier for the target message thread of the forum topic
    pub fn new<C>(chat_id: C, message_thread_id: Integer) -> Self
    where
        C: Into<ChatId>,
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

/// Reopen a closed 'General' topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights.
/// The topic will be automatically unhidden if it was hidden.
#[derive(Clone, Debug, Serialize)]
pub struct ReopenGeneralForumTopic {
    chat_id: ChatId,
}

impl ReopenGeneralForumTopic {
    /// Creates a new ReopenGeneralForumTopic
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
        }
    }
}

impl Method for ReopenGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("reopenGeneralForumTopic", self)
    }
}

/// Unhide the 'General' topic in a forum supergroup chat
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_manage_topics administrator rights.
#[derive(Clone, Debug, Serialize)]
pub struct UnhideGeneralForumTopic {
    chat_id: ChatId,
}

impl UnhideGeneralForumTopic {
    /// Creates a new UnhideGeneralForumTopic
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
        }
    }
}

impl Method for UnhideGeneralForumTopic {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("unhideGeneralForumTopic", self)
    }
}

/// Clear the list of pinned messages in a forum topic
///
/// The bot must be an administrator in the chat for this to work
/// and must have the can_pin_messages administrator right in the supergroup
#[derive(Clone, Debug, Serialize)]
pub struct UnpinAllForumTopicMessages {
    chat_id: ChatId,
    message_thread_id: Integer,
}

impl UnpinAllForumTopicMessages {
    /// Creates a new UnpinAllForumTopicMessages
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_thread_id - Unique identifier for the target message thread of the forum topic
    pub fn new<C>(chat_id: C, message_thread_id: Integer) -> Self
    where
        C: Into<ChatId>,
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

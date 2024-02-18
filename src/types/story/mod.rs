use serde::{Deserialize, Serialize};

use crate::types::{Chat, Integer};

#[cfg(test)]
mod tests;

/// Represents a story.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Story {
    /// Chat that posted the story.
    pub chat: Chat,
    /// Unique identifier of the story in the chat.
    pub id: Integer,
}

impl Story {
    /// Creates a new `Story`.
    ///
    /// # Arguments
    ///
    /// * `chat` - Chat that posted the story.
    /// * `id` - Unique identifier of the story in the chat.
    pub fn new<T>(chat: T, id: Integer) -> Self
    where
        T: Into<Chat>,
    {
        Self { chat: chat.into(), id }
    }

    /// Sets a new chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat that posted the story.
    pub fn with_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.chat = value.into();
        self
    }

    /// Sets a new ID.
    ///
    /// # Arguments
    ///
    /// `value` - Unique identifier of the story in the chat.
    pub fn with_id(mut self, value: Integer) -> Self {
        self.id = value;
        self
    }
}

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Represents a reply interface trigger.
///
/// Upon receiving a message with this object, Telegram clients
/// will display a reply interface to the user.
/// This can be extremely useful if you want to create user-friendly step-by-step interfaces
/// without having to sacrifice [privacy mode][1].
///
/// [1]: https://core.telegram.org/bots/features#privacy-mode
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ForceReply {
    force_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

impl ForceReply {
    /// Creates a new `ForceReply`.
    ///
    /// # Arguments
    ///
    /// * `force_reply` - Indicates whether to show reply interface to the user,
    ///                   as if they manually selected the bot‘s message
    ///                   and tapped ’Reply'.
    pub fn new(force_reply: bool) -> Self {
        ForceReply {
            force_reply,
            input_field_placeholder: None,
            selective: None,
        }
    }

    /// Sets a new input field placeholder.
    ///
    /// # Arguments
    ///
    /// * `value` - The placeholder to be shown
    ///             in the input field when
    ///             the keyboard is active;
    ///             1-64 characters.
    pub fn with_input_field_placeholder<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.input_field_placeholder = Some(value.into());
        self
    }

    /// Sets a new value for a `selective` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Value of the flag.
    ///
    /// Use this parameter if you want to force reply from specific users only.
    ///
    /// Targets:
    ///
    /// 1. users that are `@mentioned` in the text of the Message object.
    /// 2. if the bot message is a reply (has `reply_to_message_id`),
    ///    sender of the original message.
    pub fn with_selective(mut self, selective: bool) -> Self {
        self.selective = Some(selective);
        self
    }
}

impl From<bool> for ForceReply {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

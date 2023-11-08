use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, Message, ReplyMarkup},
};

#[cfg(test)]
mod tests;

/// Represents a dice with random value
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Dice {
    #[serde(rename = "emoji")]
    dice_type: DiceType,
    value: Integer,
}

impl Dice {
    /// Returns a type of a dice
    pub fn dice_type(&self) -> DiceType {
        self.dice_type
    }

    /// Returns a value of a dice
    pub fn value(&self) -> Integer {
        self.value
    }
}

/// Represents a type of a dice
#[derive(Debug, Copy, Clone, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
pub enum DiceType {
    /// Basketball; range: 1-5
    #[serde(rename = "🏀")]
    Basketball,
    /// Bones; range: 1-6
    #[serde(rename = "🎲")]
    Bones,
    /// Bowling; range: 1-6
    #[serde(rename = "🎳")]
    Bowling,
    /// Darts; range: 1-6
    #[serde(rename = "🎯")]
    Darts,
    /// Football; range: 1-5
    #[serde(rename = "⚽")]
    Football,
    /// Slot machine; range: 1-64
    #[serde(rename = "🎰")]
    SlotMachine,
}

impl DiceType {
    fn as_char(self) -> char {
        use super::DiceType::*;
        match self {
            Basketball => '🏀',
            Bones => '🎲',
            Bowling => '🎳',
            Darts => '🎯',
            Football => '⚽',
            SlotMachine => '🎰',
        }
    }
}

impl From<DiceType> for char {
    fn from(value: DiceType) -> Self {
        value.as_char()
    }
}

impl fmt::Display for DiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.as_char(), f)
    }
}

/// Send a dice
#[derive(Clone, Debug, Serialize)]
pub struct SendDice {
    chat_id: ChatId,
    emoji: DiceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
}

impl SendDice {
    /// Creates a new SendDice
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * dice_type - Type of a dice
    pub fn new<C>(chat_id: C, dice_type: DiceType) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            emoji: dice_type,
            allow_sending_without_reply: None,
            disable_notification: None,
            message_thread_id: None,
            protect_content: None,
            reply_markup: None,
            reply_to_message_id: None,
        }
    }

    /// Sets a new value for the `allow_sending_without_reply` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the message should be sent even
    ///           if the specified replied-to message is not found
    pub fn with_allow_sending_without_reply(mut self, value: bool) -> Self {
        self.allow_sending_without_reply = Some(value);
        self
    }

    /// Sets a new value for the `disable_notification` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether to sends the message silently;
    ///           a user will receive a notification without sound
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message thread ID
    ///
    /// # Arguments
    ///
    /// * value - Unique identifier of the target message thread (topic) of the forum;
    ///           for forum supergroups only
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for the `protect_content` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether to protects the contents of the sent message from forwarding
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets a new reply markup
    ///
    /// # Arguments
    ///
    /// * value - Markup
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets a new message ID for a reply
    ///
    /// # Arguments
    ///
    /// * value - ID of the original message
    pub fn with_reply_to_message_id(mut self, value: Integer) -> Self {
        self.reply_to_message_id = Some(value);
        self
    }
}

impl Method for SendDice {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendDice", self)
    }
}

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
    kind: DiceKind,
    value: Integer,
}

impl Dice {
    /// Kind of the dice
    pub fn kind(&self) -> DiceKind {
        self.kind
    }

    /// Value of the dice
    pub fn value(&self) -> Integer {
        self.value
    }
}

/// Kind of the dice
#[derive(Debug, Copy, Clone, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
pub enum DiceKind {
    /// Basketball
    ///
    /// Value of the dice: 1-5
    #[serde(rename = "🏀")]
    Basketball,
    /// Bones
    ///
    /// Value of the dice: 1-6
    #[serde(rename = "🎲")]
    Bones,
    /// Bowling
    ///
    /// Value of the dice: 1-6
    #[serde(rename = "🎳")]
    Bowling,
    /// Darts
    ///
    /// Value of the dice: 1-6
    #[serde(rename = "🎯")]
    Darts,
    /// Football
    ///
    /// Value of the dice: 1-5
    #[serde(rename = "⚽")]
    Football,
    /// Slot machine
    ///
    /// Value of the dice: 1-64
    #[serde(rename = "🎰")]
    SlotMachine,
}

impl DiceKind {
    fn as_char(self) -> char {
        use super::DiceKind::*;
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

impl From<DiceKind> for char {
    fn from(kind: DiceKind) -> Self {
        kind.as_char()
    }
}

impl fmt::Display for DiceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.as_char(), f)
    }
}

/// Use this method to send a dice
///
/// Dice will have a random value from 1 to 6
#[derive(Clone, Debug, Serialize)]
pub struct SendDice {
    chat_id: ChatId,
    emoji: DiceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendDice {
    /// Creates a new SendDice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * kind - Kind of dice
    pub fn new<C>(chat_id: C, kind: DiceKind) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            emoji: kind,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendDice {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendDice", self)
    }
}

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, Message, ReplyMarkup, ReplyParameters},
};

#[cfg(test)]
mod tests;

/// Represents a dice with a random value.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Dice {
    #[serde(rename = "emoji")]
    dice_type: DiceType,
    value: Integer,
}

impl Dice {
    /// Creates a new `Dice`.
    ///
    /// # Arguments
    ///
    /// * `dice_type` - Type of the dice.
    /// * `value` - Value rolled on the dice.
    pub fn new(dice_type: DiceType, value: Integer) -> Self {
        Self { dice_type, value }
    }

    /// Returns the type of the dice.
    pub fn dice_type(&self) -> DiceType {
        self.dice_type
    }

    /// Returns the value rolled on the dice.
    pub fn value(&self) -> Integer {
        self.value
    }
}

/// Represents a type of a dice.
#[derive(Debug, Copy, Clone, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
pub enum DiceType {
    /// Basketball; range: 1-5.
    #[serde(rename = "🏀")]
    Basketball,
    /// Bones; range: 1-6.
    #[serde(rename = "🎲")]
    Bones,
    /// Bowling; range: 1-6.
    #[serde(rename = "🎳")]
    Bowling,
    /// Darts; range: 1-6.
    #[serde(rename = "🎯")]
    Darts,
    /// Football; range: 1-5.
    #[serde(rename = "⚽")]
    Football,
    /// Slot machine; range: 1-64.
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

/// Sends a dice.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendDice {
    chat_id: ChatId,
    emoji: DiceType,
    allow_paid_broadcast: Option<bool>,
    business_connection_id: Option<String>,
    disable_notification: Option<bool>,
    message_effect_id: Option<String>,
    message_thread_id: Option<Integer>,
    protect_content: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
    reply_parameters: Option<ReplyParameters>,
}

impl SendDice {
    /// Creates a new `SendDice`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `dice_type` - Type of a dice.
    pub fn new<T>(chat_id: T, dice_type: DiceType) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            emoji: dice_type,
            allow_paid_broadcast: None,
            business_connection_id: None,
            disable_notification: None,
            message_effect_id: None,
            message_thread_id: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
        }
    }

    /// Sets a new value for an `allow_paid_broadcast` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to allow up to 1000 messages per second, ignoring broadcasting limits
    ///             for a fee of 0.1 Telegram Stars per message.
    ///             The relevant Stars will be withdrawn from the bot's balance.
    pub fn with_allow_paid_broadcast(mut self, value: bool) -> Self {
        self.allow_paid_broadcast = Some(value);
        self
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect to be added to the message; for private chats only.
    pub fn with_message_effect_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.message_effect_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.reply_parameters = Some(value);
        self
    }
}

impl Method for SendDice {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendDice", self)
    }
}

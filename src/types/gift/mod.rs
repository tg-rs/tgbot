use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, ParseMode, Sticker, TextEntities, TextEntity},
};

#[cfg(test)]
mod tests;

/// Represents a gift that can be sent by the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Gift {
    /// Unique identifier of the gift.
    pub id: String,
    /// The sticker that represents the gift.
    pub sticker: Sticker,
    /// The number of Telegram Stars that must be paid to send the sticker.
    pub star_count: Integer,
    /// The number of remaining gifts of this type that can be sent;
    /// for limited gifts only.
    pub remaining_count: Option<Integer>,
    /// The total number of the gifts of this type that can be sent;
    /// for limited gifts only.
    pub total_count: Option<Integer>,
    /// The number of Telegram Stars that must be paid to upgrade the gift to a unique one.
    pub upgrade_star_count: Option<Integer>,
}

impl Gift {
    /// Creates a new `Gift`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the gift.
    /// * `sticker` - The sticker that represents the gift.
    /// * `star_count` - The number of Telegram Stars that must be paid to send the sticker.
    pub fn new<T>(id: T, sticker: Sticker, star_count: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            id: id.into(),
            sticker,
            star_count,
            total_count: None,
            remaining_count: None,
            upgrade_star_count: None,
        }
    }

    /// Sets a new remaining count.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of remaining gifts of this type that can be sent.
    pub fn with_remaining_count(mut self, value: Integer) -> Self {
        self.remaining_count = Some(value);
        self
    }

    /// Sets a new total count.
    ///
    /// # Arguments
    ///
    /// * `value` - The total number of the gifts of this type that can be sent.
    pub fn with_total_count(mut self, value: Integer) -> Self {
        self.total_count = Some(value);
        self
    }

    /// Sets a new upgrade star count.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of Telegram Stars that must be paid to upgrade the gift to a unique one.
    pub fn with_upgrade_star_count(mut self, value: Integer) -> Self {
        self.upgrade_star_count = Some(value);
        self
    }
}

/// Represent a list of gifts.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Gifts {
    gifts: Vec<Gift>,
}

impl<T> From<T> for Gifts
where
    T: IntoIterator<Item = Gift>,
{
    fn from(value: T) -> Self {
        Self {
            gifts: value.into_iter().collect(),
        }
    }
}

/// Returns the list of gifts that can be sent by the bot to users.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GetAvailableGifts;

impl Method for GetAvailableGifts {
    type Response = Gifts;

    fn into_payload(self) -> Payload {
        Payload::empty("getAvailableGifts")
    }
}

/// Sends a gift to the given user.
///
/// The gift can't be converted to Telegram Stars by the user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SendGift {
    gift_id: String,
    chat_id: Option<ChatId>,
    pay_for_upgrade: Option<bool>,
    text: Option<String>,
    text_parse_mode: Option<ParseMode>,
    text_entities: Option<TextEntities>,
    user_id: Option<Integer>,
}

impl SendGift {
    /// Creates a new `SendGift` with a `user_id`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat that will receive the gift.
    /// * `gift_id` - Identifier of the gift
    pub fn for_chat_id<A, B>(chat_id: A, gift_id: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
    {
        Self {
            gift_id: gift_id.into(),
            chat_id: Some(chat_id.into()),
            pay_for_upgrade: None,
            text: None,
            text_parse_mode: None,
            text_entities: None,
            user_id: None,
        }
    }

    /// Creates a new `SendGift` with a `user_id`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user that will receive the gift.
    /// * `gift_id` - Identifier of the gift
    pub fn for_user_id<T>(user_id: Integer, gift_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            gift_id: gift_id.into(),
            chat_id: None,
            pay_for_upgrade: None,
            text: None,
            text_parse_mode: None,
            text_entities: None,
            user_id: Some(user_id),
        }
    }

    /// Sets a new value for a `pay_for_upgrade` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to pay for the gift upgrade from the bot's balance,
    ///             thereby making the upgrade free for the receiver.
    pub fn with_pay_for_upgrade(mut self, value: bool) -> Self {
        self.pay_for_upgrade = Some(value);
        self
    }

    /// Sets a new text.
    ///
    /// # Arguments
    ///
    /// * `value` - Text that will be shown along with the gift; 0-255 characters.
    pub fn with_text<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.text = Some(value.into());
        self
    }

    /// Sets a new text parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the text.
    ///
    /// Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
    /// Text entities will be set to [`None`] when this method is called.
    pub fn with_text_parse_mode(mut self, value: ParseMode) -> Self {
        self.text_parse_mode = Some(value);
        self.text_entities = None;
        self
    }

    /// Sets a new text entities.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of special entities that appear in the gift text.
    ///
    /// Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
    /// Text parse mode will be set to [`None`] when this method is called.
    pub fn with_text_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.text_entities = Some(value.into_iter().collect());
        self.text_parse_mode = None;
        self
    }
}

impl Method for SendGift {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("sendGift", self)
    }
}

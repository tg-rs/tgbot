use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, ParseMode, Sticker, TextEntities, TextEntity, User},
};

#[cfg(test)]
mod tests;

/// Describes the types of gifts that can be gifted to a user or a chat.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct AcceptedGiftTypes {
    /// Whether limited regular gifts are accepted.
    pub limited_gifts: bool,
    /// Whether a Telegram Premium subscription is accepted.
    pub premium_subscription: bool,
    /// Whether unique gifts or gifts that can be upgraded to unique for free are accepted.
    pub unique_gifts: bool,
    /// Whether unlimited regular gifts are accepted.
    pub unlimited_gifts: bool,
}

impl AcceptedGiftTypes {
    /// Sets a new value for the `limited_gifts` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether limited regular gifts are accepted.
    pub fn with_limited_gifts(mut self, value: bool) -> Self {
        self.limited_gifts = value;
        self
    }

    /// Sets a new value for the `premium_subscription` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether a Telegram Premium subscription is accepted.
    pub fn with_premium_subscription(mut self, value: bool) -> Self {
        self.premium_subscription = value;
        self
    }

    /// Sets a new value for the `unique_gifts` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether unique gifts or gifts that can be upgraded to unique for free are accepted.
    pub fn with_unique_gifts(mut self, value: bool) -> Self {
        self.unique_gifts = value;
        self
    }

    /// Sets a new value for the `unlimited_gifts` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether unlimited regular gifts are accepted.
    pub fn with_unlimited_gifts(mut self, value: bool) -> Self {
        self.unlimited_gifts = value;
        self
    }
}

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

/// Describes a service message about a regular gift that was sent or received.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct GiftInfo {
    /// Information about the gift.
    pub gift: Gift,
    /// Whether the gift can be upgraded to a unique gift.
    pub can_be_upgraded: Option<bool>,
    /// Number of Telegram Stars that can be claimed by the receiver by converting the gift;
    /// omitted if conversion to Telegram Stars is impossible.
    pub convert_star_count: Option<Integer>,
    /// Special entities that appear in the text.
    pub entities: Option<TextEntities>,
    /// Whether the sender and gift text are shown only to the gift receiver;
    /// otherwise, everyone will be able to see them.
    pub is_private: Option<bool>,
    /// Unique identifier of the received gift for the bot;
    /// only present for gifts received on behalf of business accounts.
    pub owned_gift_id: Option<String>,
    /// Number of Telegram Stars that were prepaid by the sender for the ability to upgrade the gift.
    pub prepaid_upgrade_star_count: Option<Integer>,
    /// Text of the message that was added to the gift.
    pub text: Option<String>,
}

impl GiftInfo {
    /// Creates a new `GiftInfo`.
    ///
    /// # Arguments
    ///
    /// * `gift` - Information about the gift.
    pub fn new(gift: Gift) -> Self {
        Self {
            gift,
            can_be_upgraded: None,
            convert_star_count: None,
            entities: None,
            is_private: None,
            owned_gift_id: None,
            prepaid_upgrade_star_count: None,
            text: None,
        }
    }

    /// Sets a new value for the `can_be_upgraded` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the gift can be upgraded to a unique gift.
    pub fn with_can_be_upgraded(mut self, value: bool) -> Self {
        self.can_be_upgraded = Some(value);
        self
    }

    /// Sets a new convert star count.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of Telegram Stars that can be claimed by the receiver by converting the gift;
    ///   omitted if conversion to Telegram Stars is impossible.
    pub fn with_convert_star_count(mut self, value: Integer) -> Self {
        self.convert_star_count = Some(value);
        self
    }

    /// Sets a new list of entities.
    ///
    /// # Arguments
    ///
    /// * `value` - Special entities that appear in the text.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(TextEntities::from_iter(value));
        self
    }

    /// Sets a new value for the `is_private` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the sender and gift text are shown only to the gift receiver;
    ///   otherwise, everyone will be able to see them.
    pub fn with_is_private(mut self, value: bool) -> Self {
        self.is_private = Some(value);
        self
    }

    /// Sets a new owned gift ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the received gift for the bot;
    ///   only present for gifts received on behalf of business accounts.
    pub fn with_owned_gift_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.owned_gift_id = Some(value.into());
        self
    }

    /// Sets a new prepaid upgrade star count.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of Telegram Stars that were prepaid by the sender for the ability to upgrade the gift.
    pub fn with_prepaid_upgrade_star_count(mut self, value: Integer) -> Self {
        self.prepaid_upgrade_star_count = Some(value);
        self
    }

    /// Sets a new text.
    ///
    /// # Arguments
    ///
    /// * `value` - Text of the message that was added to the gift.
    pub fn with_text<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.text = Some(value.into());
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

/// Describes a gift received and owned by a user or a chat.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum OwnedGift {
    /// A regular gift owned by a user or a chat.
    Regular(OwnedGiftRegular),
    /// A unique gift received and owned by a user or a chat.
    Unique(OwnedGiftUnique),
}

/// Describes a regular gift owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OwnedGiftRegular {
    /// Information about the regular gift.
    pub gift: Gift,
    /// Date the gift was sent in Unix time.
    pub send_date: Integer,
    /// Whether the gift can be upgraded to a unique gift;
    /// for gifts received on behalf of business accounts only.
    pub can_be_upgraded: Option<bool>,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the gift;
    /// omitted if the gift cannot be converted to Telegram Stars.
    pub convert_star_count: Option<Integer>,
    /// Special entities that appear in the text.
    pub entities: Option<TextEntities>,
    /// Whether the sender and gift text are shown only to the gift receiver;
    /// otherwise, everyone will be able to see them.
    pub is_private: Option<bool>,
    /// Whether the gift is displayed on the account's profile page;
    /// for gifts received on behalf of business accounts only.
    pub is_saved: Option<bool>,
    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only.
    pub owned_gift_id: Option<String>,
    /// Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift.
    pub prepaid_upgrade_star_count: Option<Integer>,
    /// Sender of the gift if it is a known user.
    pub sender_user: Option<User>,
    /// Text of the message that was added to the gift.
    pub text: Option<String>,
    /// Whether the gift was refunded and isn't available anymore.
    pub was_refunded: Option<bool>,
}

impl OwnedGiftRegular {
    /// Creates a new `OwnedGiftRegular`.
    ///
    /// # Arguments
    ///
    /// * `gift` - Information about the regular gift.
    /// * `send_date` - Date the gift was sent in Unix time.
    pub fn new(gift: Gift, send_date: Integer) -> Self {
        Self {
            gift,
            send_date,
            can_be_upgraded: None,
            convert_star_count: None,
            entities: None,
            is_private: None,
            is_saved: None,
            owned_gift_id: None,
            prepaid_upgrade_star_count: None,
            sender_user: None,
            text: None,
            was_refunded: None,
        }
    }

    /// Sets a new value for the `can_be_upgraded` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the gift can be upgraded to a unique gift;
    ///   for gifts received on behalf of business accounts only.
    pub fn with_can_be_upgraded(mut self, value: bool) -> Self {
        self.can_be_upgraded = Some(value);
        self
    }

    /// Sets a new convert star count.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of Telegram Stars that can be claimed by the receiver instead of the gift;
    ///   omitted if the gift cannot be converted to Telegram Stars.
    pub fn with_convert_star_count(mut self, value: Integer) -> Self {
        self.convert_star_count = Some(value);
        self
    }

    /// Sets a new list of text entities.
    ///
    /// # Arguments
    ///
    /// * `value` - Special entities that appear in the text.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(value.into_iter().collect());
        self
    }

    /// Sets a new value for the `is_private` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the sender and gift text are shown only to the gift receiver;
    ///   otherwise, everyone will be able to see them.
    pub fn with_is_private(mut self, value: bool) -> Self {
        self.is_private = Some(value);
        self
    }

    /// Sets a new value for the `is_saved` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the gift is displayed on the account's profile page;
    ///   for gifts received on behalf of business accounts only.
    pub fn with_is_saved(mut self, value: bool) -> Self {
        self.is_saved = Some(value);
        self
    }

    /// Sets a new owned gift ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the gift for the bot;
    ///   for gifts received on behalf of business accounts only.
    pub fn with_owned_gift_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.owned_gift_id = Some(value.into());
        self
    }

    /// Sets a new prepaid upgrade star count.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift.
    pub fn with_prepaid_upgrade_star_count(mut self, value: Integer) -> Self {
        self.prepaid_upgrade_star_count = Some(value);
        self
    }

    /// Sets a new sender user.
    ///
    /// # Arguments
    ///
    /// * `value` - Sender of the gift if it is a known user.
    pub fn with_sender_user(mut self, value: User) -> Self {
        self.sender_user = Some(value);
        self
    }

    /// Sets a new text.
    ///
    /// # Arguments
    ///
    /// * `value` - Text of the message that was added to the gift.
    pub fn with_text<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.text = Some(value.into());
        self
    }

    /// Sets a new value for the `was_refunded` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the gift was refunded and isn't available anymore.
    pub fn with_was_refunded(mut self, value: bool) -> Self {
        self.was_refunded = Some(value);
        self
    }
}

/// Describes a unique gift received and owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OwnedGiftUnique {
    /// Information about the unique gift.
    pub gift: UniqueGift,
    /// Date the gift was sent in Unix time.
    pub send_date: Integer,
    /// Number of Telegram Stars that must be paid to transfer the gift;
    /// omitted if the bot cannot transfer the gift.
    pub transfer_star_count: Option<Integer>,
    /// Whether the gift can be transferred to another owner;
    /// for gifts received on behalf of business accounts only.
    pub can_be_transferred: Option<bool>,
    /// Whether the gift is displayed on the account's profile page;
    /// for gifts received on behalf of business accounts only.
    pub is_saved: Option<bool>,
    /// Unique identifier of the received gift for the bot;
    /// for gifts received on behalf of business accounts only.
    pub owned_gift_id: Option<String>,
    /// Sender of the gift if it is a known user.
    pub sender_user: Option<User>,
}

impl OwnedGiftUnique {
    /// Creates a new `OwnedGiftUnique`.
    ///
    /// # Arguments
    ///
    /// * `gift` - Information about the unique gift.
    /// * `send_date` - Date the gift was sent in Unix time.
    pub fn new(gift: UniqueGift, send_date: Integer) -> Self {
        Self {
            gift,
            send_date,
            transfer_star_count: None,
            can_be_transferred: None,
            is_saved: None,
            owned_gift_id: None,
            sender_user: None,
        }
    }

    /// Sets a new transfer star count.
    ///
    /// # Arguments
    ///
    /// * `value` Number of Telegram Stars that must be paid to transfer the gift;
    ///   omitted if the bot cannot transfer the gift.
    pub fn with_transfer_star_count(mut self, value: Integer) -> Self {
        self.transfer_star_count = Some(value);
        self
    }

    /// Sets a new value for the `can_be_transferred` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the gift can be transferred to another owner;
    ///   for gifts received on behalf of business accounts only.
    pub fn with_can_be_transferred(mut self, value: bool) -> Self {
        self.can_be_transferred = Some(value);
        self
    }

    /// Sets a new value for the `is_saved` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the gift is displayed on the account's profile page;
    ///   for gifts received on behalf of business accounts only.
    pub fn with_is_saved(mut self, value: bool) -> Self {
        self.is_saved = Some(value);
        self
    }

    /// Sets a new owned gift ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the received gift for the bot;
    ///   for gifts received on behalf of business accounts only.
    pub fn with_owned_gift_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.owned_gift_id = Some(value.into());
        self
    }

    /// Sets a new sender user.
    ///
    /// # Arguments
    ///
    /// * `value` - Sender of the gift if it is a known user.
    pub fn with_sender_user(mut self, value: User) -> Self {
        self.sender_user = Some(value);
        self
    }
}

/// Contains the list of gifts received and owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OwnedGifts {
    /// The list of gifts.
    pub gifts: Vec<OwnedGift>,
    /// The total number of gifts owned by the user or the chat.
    pub total_count: Integer,
    /// Offset for the next request.
    ///
    /// If empty, then there are no more results.
    pub next_offset: Option<String>,
}

impl OwnedGifts {
    /// Creates a new `OwnedGifts`.
    ///
    /// # Arguments
    ///
    /// * `gifts` - The list of gifts.
    /// * `total_count` - The total number of gifts owned by the user or the chat.
    pub fn new<T>(gifts: T, total_count: Integer) -> Self
    where
        T: IntoIterator<Item = OwnedGift>,
    {
        Self {
            gifts: gifts.into_iter().collect(),
            total_count,
            next_offset: None,
        }
    }

    /// Sets a new next offset.
    ///
    /// # Arguments
    ///
    /// * `value` - Offset for the next request.
    pub fn with_next_offset<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.next_offset = Some(value.into());
        self
    }
}

/// Describes a unique gift that was upgraded from a regular gift.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UniqueGift {
    /// Backdrop of the gift.
    pub backdrop: UniqueGiftBackdrop,
    /// Human-readable name of the regular gift from which this unique gift was upgraded.
    pub base_name: String,
    /// Model of the gift.
    pub model: UniqueGiftModel,
    /// Unique name of the gift. This name can be used in https://t.me/nft/... links and story areas.
    pub name: String,
    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift.
    pub number: Integer,
    ///  Symbol of the gift.
    pub symbol: UniqueGiftSymbol,
}

/// Describes the backdrop of a unique gift.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UniqueGiftBackdrop {
    /// Colors of the backdrop.
    pub colors: UniqueGiftBackdropColors,
    /// Name of the backdrop.
    pub name: String,
    /// The number of unique gifts that receive this backdrop for every 1000 gifts upgraded.
    pub rarity_per_mille: Integer,
}

/// Describes the colors of the backdrop of a unique gift.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UniqueGiftBackdropColors {
    /// The color in the center of the backdrop in RGB format
    pub center_color: Integer,
    /// The color on the edges of the backdrop in RGB format
    pub edge_color: Integer,
    /// The color to be applied to the symbol in RGB format
    pub symbol_color: Integer,
    /// The color for the text on the backdrop in RGB format
    pub text_color: Integer,
}

/// Describes the model of a unique gift.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UniqueGiftModel {
    /// Name of the model.
    pub name: String,
    /// The number of unique gifts that receive this model for every 1000 gifts upgraded.
    pub rarity_per_mille: Integer,
    /// The sticker that represents the unique gift
    pub sticker: Sticker,
}

/// Describes the symbol shown on the pattern of a unique gift.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UniqueGiftSymbol {
    /// Name of the symbol.
    pub name: String,
    /// The number of unique gifts that receive this model for every 1000 gifts upgraded.
    pub rarity_per_mille: Integer,
    /// The sticker that represents the unique gift.
    pub sticker: Sticker,
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

    /// Sets a new value for the `pay_for_upgrade` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to pay for the gift upgrade from the bot's balance,
    ///   thereby making the upgrade free for the receiver.
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

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload, PayloadError},
    types::{Chat, ChatId, InputText, Integer, ParseMode, Sticker, TextEntities, User},
};

/// Describes the types of gifts that can be gifted to a user or a chat.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct AcceptedGiftTypes {
    /// Whether transfers of unique gifts from channels are accepted.
    pub gifts_from_channels: bool,
    /// Whether limited regular gifts are accepted.
    pub limited_gifts: bool,
    /// Whether a Telegram Premium subscription is accepted.
    pub premium_subscription: bool,
    /// Whether unique gifts or gifts that can be upgraded to unique for free are accepted.
    pub unique_gifts: bool,
    /// Whether unlimited regular gifts are accepted.
    pub unlimited_gifts: bool,
}

/// Represents a gift that can be sent by the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Gift {
    /// Unique identifier of the gift.
    pub id: String,
    /// The number of Telegram Stars that must be paid to send the sticker.
    pub star_count: Integer,
    /// The sticker that represents the gift.
    pub sticker: Sticker,
    /// Background of the gift.
    pub background: Option<GiftBackground>,
    /// Whether the gift can be used (after being upgraded) to customize a user's appearance.
    pub has_colors: Option<bool>,
    /// Whether the gift can only be purchased by Telegram Premium subscribers.
    pub is_premium: Option<bool>,
    /// The number of remaining gifts of this type that can be sent by the bot; for limited gifts only.
    pub personal_remaining_count: Option<Integer>,
    /// The total number of gifts of this type that can be sent by the bot; for limited gifts only.
    pub personal_total_count: Option<Integer>,
    /// Information about the chat that published the gift.
    pub publisher_chat: Option<Chat>,
    /// The number of remaining gifts of this type that can be sent;
    /// for limited gifts only.
    pub remaining_count: Option<Integer>,
    /// The total number of the gifts of this type that can be sent;
    /// for limited gifts only.
    pub total_count: Option<Integer>,
    /// The total number of different unique gifts that can be obtained by upgrading the gift.
    pub unique_gift_variant_count: Option<Integer>,
    /// The number of Telegram Stars that must be paid to upgrade the gift to a unique one.
    pub upgrade_star_count: Option<Integer>,
}

/// Describes the background of a gift.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: Integer,
    /// Edge color of the background in RGB format
    pub edge_color: Integer,
    /// Text color of the background in RGB format
    pub text_color: Integer,
}

/// Describes a service message about a regular gift that was sent or received.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    /// Whether the gift's upgrade was purchased after the gift was sent.
    pub is_upgrade_separate: Option<bool>,
    /// Unique identifier of the received gift for the bot;
    /// only present for gifts received on behalf of business accounts.
    pub owned_gift_id: Option<String>,
    /// Number of Telegram Stars that were prepaid by the sender for the ability to upgrade the gift.
    pub prepaid_upgrade_star_count: Option<Integer>,
    /// Text of the message that was added to the gift.
    pub text: Option<String>,
    /// Unique number reserved for this gift when upgraded.
    ///
    /// See the number field in [`crate::types::UniqueGift`].
    pub unique_gift_number: Option<Integer>,
}

/// Represent a list of gifts.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Gifts {
    gifts: Vec<Gift>,
}

impl From<Gifts> for Vec<Gift> {
    fn from(value: Gifts) -> Self {
        value.gifts
    }
}

/// Returns the list of gifts that can be sent by the bot to users.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GetAvailableGifts;

impl Method for GetAvailableGifts {
    type Response = Gifts;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::empty("getAvailableGifts")
    }
}

/// Returns the gifts received and owned by a managed business account.
///
/// Requires the `can_view_gifts_and_stars` business bot right.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct GetBusinessAccountGifts {
    business_connection_id: String,
    exclude_from_blockchain: Option<bool>,
    exclude_limited_non_upgradable: Option<bool>,
    exclude_limited_upgradable: Option<bool>,
    exclude_saved: Option<bool>,
    exclude_unique: Option<bool>,
    exclude_unlimited: Option<bool>,
    exclude_unsaved: Option<bool>,
    limit: Option<Integer>,
    offset: Option<String>,
    sort_by_price: Option<bool>,
}

impl GetBusinessAccountGifts {
    /// Creates a new `GetBusinessAccountGifts`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    pub fn new<T>(business_connection_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            exclude_from_blockchain: None,
            exclude_limited_non_upgradable: None,
            exclude_limited_upgradable: None,
            exclude_saved: None,
            exclude_unique: None,
            exclude_unlimited: None,
            exclude_unsaved: None,
            limit: None,
            offset: None,
            sort_by_price: None,
        }
    }

    /// Sets a new value for the `exclude_from_blockchain` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that were assigned from the TON blockchain
    ///   and can't be resold or transferred in Telegram.
    pub fn with_exclude_from_blockchain(mut self, value: bool) -> Self {
        self.exclude_from_blockchain = Some(value);
        self
    }

    /// Sets a new value for the `exclude_limited_non_upgradable` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased
    ///   a limited number of times and can't be upgraded to unique.
    pub fn with_exclude_limited_non_upgradable(mut self, value: bool) -> Self {
        self.exclude_limited_non_upgradable = Some(value);
        self
    }

    /// Sets a new value for the `exclude_limited_upgradable` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased
    ///   a limited number of times and can be upgraded to unique
    pub fn with_exclude_limited_upgradable(mut self, value: bool) -> Self {
        self.exclude_limited_upgradable = Some(value);
        self
    }

    /// Sets a new value for the `exclude_saved` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that are saved to the account's profile page.
    pub fn with_exclude_saved(mut self, value: bool) -> Self {
        self.exclude_saved = Some(value);
        self
    }

    /// Sets a new value for the `exclude_unique` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude unique gifts.
    pub fn with_exclude_unique(mut self, value: bool) -> Self {
        self.exclude_unique = Some(value);
        self
    }

    /// Sets a new value for the `exclude_unlimited` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased an unlimited number of times.
    pub fn with_exclude_unlimited(mut self, value: bool) -> Self {
        self.exclude_unlimited = Some(value);
        self
    }

    /// Sets a new value for the `exclude_unsaved` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that aren't saved to the account's profile page.
    pub fn with_exclude_unsaved(mut self, value: bool) -> Self {
        self.exclude_unsaved = Some(value);
        self
    }

    /// Sets a new limit.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum number of gifts to be returned; 1-100; defaults to 100.
    pub fn with_limit(mut self, value: Integer) -> Self {
        self.limit = Some(value);
        self
    }

    /// Sets a new offset.
    ///
    /// # Arguments
    ///
    /// * `value` - Offset of the first entry to return as received from the previous request;
    ///   use empty string to get the first chunk of results.
    pub fn with_offset<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.offset = Some(value.into());
        self
    }

    /// Sets a new value for the `sort_by_price` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to sort results by gift price instead of send date;
    ///   sorting is applied before pagination.
    pub fn with_sort_by_price(mut self, value: bool) -> Self {
        self.sort_by_price = Some(value);
        self
    }
}

impl Method for GetBusinessAccountGifts {
    type Response = OwnedGifts;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("getBusinessAccountGifts", self)
    }
}

/// Returns the gifts owned by a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct GetChatGifts {
    chat_id: ChatId,
    exclude_from_blockchain: Option<bool>,
    exclude_limited_non_upgradable: Option<bool>,
    exclude_limited_upgradable: Option<bool>,
    exclude_saved: Option<bool>,
    exclude_unique: Option<bool>,
    exclude_unlimited: Option<bool>,
    exclude_unsaved: Option<bool>,
    limit: Option<Integer>,
    offset: Option<String>,
    sort_by_price: Option<bool>,
}

impl GetChatGifts {
    /// Creates a new `GetChatGifts`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel.
    pub fn new<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            exclude_from_blockchain: None,
            exclude_limited_non_upgradable: None,
            exclude_limited_upgradable: None,
            exclude_saved: None,
            exclude_unique: None,
            exclude_unlimited: None,
            exclude_unsaved: None,
            limit: None,
            offset: None,
            sort_by_price: None,
        }
    }

    /// Sets a new value for the `exclude_from_blockchain` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that were assigned from the TON blockchain
    ///   and can't be resold or transferred in Telegram.
    pub fn with_exclude_from_blockchain(mut self, value: bool) -> Self {
        self.exclude_from_blockchain = Some(value);
        self
    }

    /// Sets a new value for the `exclude_limited_non_upgradable` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased
    ///   a limited number of times and can't be upgraded to unique.
    pub fn with_exclude_limited_non_upgradable(mut self, value: bool) -> Self {
        self.exclude_limited_non_upgradable = Some(value);
        self
    }

    /// Sets a new value for the `exclude_limited_upgradable` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased
    ///   a limited number of times and can be upgraded to unique.
    pub fn with_exclude_limited_upgradable(mut self, value: bool) -> Self {
        self.exclude_limited_upgradable = Some(value);
        self
    }

    /// Sets a new value for the `exclude_saved` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that are saved to the chat's profile page.
    ///   Always `false`, unless the bot has the `can_post_messages` administrator right in the channel.
    pub fn with_exclude_saved(mut self, value: bool) -> Self {
        self.exclude_saved = Some(value);
        self
    }

    /// Sets a new value for the `exclude_unique` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude unique gifts.
    pub fn with_exclude_unique(mut self, value: bool) -> Self {
        self.exclude_unique = Some(value);
        self
    }

    /// Sets a new value for the `exclude_unlimited` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased an unlimited number of times.
    pub fn with_exclude_unlimited(mut self, value: bool) -> Self {
        self.exclude_unlimited = Some(value);
        self
    }

    /// Sets a new value for the `exclude_unsaved` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that aren't saved to the chat's profile page.
    ///   Always `true`, unless the bot has the `can_post_messages` administrator right in the channel.
    pub fn with_exclude_unsaved(mut self, value: bool) -> Self {
        self.exclude_unsaved = Some(value);
        self
    }

    /// Sets a new limit.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum number of gifts to be returned; 1-100. Defaults to 100.
    pub fn with_limit(mut self, value: Integer) -> Self {
        self.limit = Some(value);
        self
    }

    /// Sets a new offset.
    ///
    /// # Arguments
    ///
    /// * `value` - Offset of the first entry to return as received from the previous request;
    ///   use an empty string to get the first chunk of results.
    pub fn with_offset<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.offset = Some(value.into());
        self
    }

    /// Sets a new value for the `sort_by_price` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to sort results by gift price instead of send date.
    ///   Sorting is applied before pagination.
    pub fn with_sort_by_price(mut self, value: bool) -> Self {
        self.sort_by_price = Some(value);
        self
    }
}

impl Method for GetChatGifts {
    type Response = OwnedGifts;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("getChatGifts", self)
    }
}

/// Returns the gifts owned and hosted by a user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct GetUserGifts {
    user_id: Integer,
    exclude_from_blockchain: Option<bool>,
    exclude_limited_non_upgradable: Option<bool>,
    exclude_limited_upgradable: Option<bool>,
    exclude_unique: Option<bool>,
    exclude_unlimited: Option<bool>,
    limit: Option<Integer>,
    offset: Option<String>,
    sort_by_price: Option<bool>,
}

impl GetUserGifts {
    /// Creates a new `GetUserGifts`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the user.
    pub fn new(user_id: Integer) -> Self {
        Self {
            user_id,
            exclude_from_blockchain: None,
            exclude_limited_non_upgradable: None,
            exclude_limited_upgradable: None,
            exclude_unique: None,
            exclude_unlimited: None,
            limit: None,
            offset: None,
            sort_by_price: None,
        }
    }

    /// Sets a new value for the `exclude_from_blockchain` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that were assigned from the TON blockchain
    ///   and can't be resold or transferred in Telegram.
    pub fn with_exclude_from_blockchain(mut self, value: bool) -> Self {
        self.exclude_from_blockchain = Some(value);
        self
    }

    /// Sets a new value for the `exclude_limited_non_upgradable` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased
    ///   a limited number of times and can't be upgraded to unique.
    pub fn with_exclude_limited_non_upgradable(mut self, value: bool) -> Self {
        self.exclude_limited_non_upgradable = Some(value);
        self
    }

    /// Sets a new value for the `exclude_limited_upgradable` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased
    ///   a limited number of times and can be upgraded to unique.
    pub fn with_exclude_limited_upgradable(mut self, value: bool) -> Self {
        self.exclude_limited_upgradable = Some(value);
        self
    }

    /// Sets a new value for the `exclude_unique` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude unique gifts.
    pub fn with_exclude_unique(mut self, value: bool) -> Self {
        self.exclude_unique = Some(value);
        self
    }

    /// Sets a new value for the `exclude_unlimited` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to exclude gifts that can be purchased an unlimited number of times.
    pub fn with_exclude_unlimited(mut self, value: bool) -> Self {
        self.exclude_unlimited = Some(value);
        self
    }

    /// Sets a new limit.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum number of gifts to be returned; 1-100. Defaults to 100.
    pub fn with_limit(mut self, value: Integer) -> Self {
        self.limit = Some(value);
        self
    }

    /// Sets a new offset.
    ///
    /// # Arguments
    ///
    /// * `value` - Offset of the first entry to return as received from the previous request;
    ///   use an empty string to get the first chunk of results.
    pub fn with_offset<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.offset = Some(value.into());
        self
    }

    /// Sets a new value for the `sort_by_price` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to sort results by gift price instead of send date.
    ///   Sorting is applied before pagination.
    pub fn with_sort_by_price(mut self, value: bool) -> Self {
        self.sort_by_price = Some(value);
        self
    }
}

impl Method for GetUserGifts {
    type Response = OwnedGifts;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("getUserGifts", self)
    }
}

/// Describes a gift received and owned by a user or a chat.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OwnedGift {
    /// A regular gift owned by a user or a chat.
    #[from(OwnedGiftRegular)]
    Regular(Box<OwnedGiftRegular>),
    /// A unique gift received and owned by a user or a chat.
    #[from(OwnedGiftUnique)]
    Unique(Box<OwnedGiftUnique>),
}

/// Describes a regular gift owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    /// Whether the gift's upgrade was purchased after the gift was sent;
    /// for gifts received on behalf of business accounts only.
    pub is_upgrade_separate: Option<bool>,
    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only.
    pub owned_gift_id: Option<String>,
    /// Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift.
    pub prepaid_upgrade_star_count: Option<Integer>,
    /// Sender of the gift if it is a known user.
    pub sender_user: Option<User>,
    /// Text of the message that was added to the gift.
    pub text: Option<String>,
    /// Unique number reserved for this gift when upgraded.
    ///
    /// See the number field in [`crate::types::UniqueGift`].
    pub unique_gift_number: Option<Integer>,
    /// Whether the gift was refunded and isn't available anymore.
    pub was_refunded: Option<bool>,
}

/// Describes a unique gift received and owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    /// Point in time (Unix timestamp) when the gift can be transferred.
    ///
    /// If it is in the past, then the gift can be transferred now.
    pub next_transfer_date: Option<Integer>,
    /// Unique identifier of the received gift for the bot;
    /// for gifts received on behalf of business accounts only.
    pub owned_gift_id: Option<String>,
    /// Sender of the gift if it is a known user.
    pub sender_user: Option<User>,
}

/// Contains the list of gifts received and owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

/// Describes a unique gift that was upgraded from a regular gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UniqueGift {
    /// Backdrop of the gift.
    pub backdrop: UniqueGiftBackdrop,
    /// Human-readable name of the regular gift from which this unique gift was upgraded.
    pub base_name: String,
    /// Identifier of the regular gift from which the gift was upgraded.
    pub gift_id: String,
    /// Model of the gift.
    pub model: UniqueGiftModel,
    /// Unique name of the gift. This name can be used in https://t.me/nft/... links and story areas.
    pub name: String,
    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift.
    pub number: Integer,
    /// Symbol of the gift.
    pub symbol: UniqueGiftSymbol,
    /// The color scheme that can be used by the gift's owner for the chat's name,
    /// replies to messages and link previews;
    /// for business account gifts and gifts that are currently on sale only.
    pub colors: Option<UniqueGiftColors>,
    /// Whether the gift was used to craft another gift and isn't available anymore.
    pub is_burned: Option<bool>,
    /// Whether the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram.
    pub is_from_blockchain: Option<bool>,
    /// Whether the original regular gift was exclusively purchaseable by Telegram Premium subscribers.
    pub is_premium: Option<bool>,
    /// Information about the chat that published the gift.
    pub publisher_chat: Option<Chat>,
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

/// Contains information about the color scheme for a user's name,
/// message replies and link previews based on a unique gift.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UniqueGiftColors {
    /// Main color used in dark themes; RGB format.
    pub dark_theme_main_color: Integer,
    /// List of 1-3 additional colors used in dark themes; RGB format.
    pub dark_theme_other_colors: Vec<Integer>,
    /// Main color used in light themes; RGB format.
    pub light_theme_main_color: Integer,
    /// List of 1-3 additional colors used in light themes; RGB format.
    pub light_theme_other_colors: Vec<Integer>,
    /// Custom emoji identifier of the unique gift's model.
    pub model_custom_emoji_id: String,
    /// Custom emoji identifier of the unique gift's symbol.
    pub symbol_custom_emoji_id: String,
}

/// Describes the model of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UniqueGiftModel {
    /// Name of the model.
    pub name: String,
    /// The number of unique gifts that receive this model for every 1000 gifts upgraded.
    pub rarity_per_mille: Integer,
    /// The sticker that represents the unique gift
    pub sticker: Sticker,
    /// Rarity of the model if it is a crafted model.
    pub rarity: Option<UniqueGiftModelRarity>,
}

/// Rarity of a unique gift model.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UniqueGiftModelRarity {
    /// Epic.
    Epic,
    /// Legendary.
    Legendary,
    /// Rare.
    Rare,
    /// Uncommon.
    Uncommon,
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

/// Origin of the unique gift.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UniqueGiftOrigin {
    /// For upgrades purchased after the gift was sent.
    GiftedUpgrade,
    /// Gifts bought or sold through gift purchase offers.
    Offer,
    /// Resale for gifts bought from other users.
    Resale,
    /// Transfer for gifts transferred from other users or channels.
    Transfer,
    /// Upgrade for gifts upgraded from regular gifts.
    Upgrade,
}

/// Describes a service message about a unique gift that was sent or received.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UniqueGiftInfo {
    /// Information about the gift.
    pub gift: UniqueGift,
    /// Origin of the gift.
    pub origin: UniqueGiftOrigin,
    /// For gifts bought from other users, the price paid for the gift.
    pub last_resale_star_count: Option<Integer>,
    /// Point in time (Unix timestamp) when the gift can be transferred.
    ///
    /// If it is in the past, then the gift can be transferred now.
    pub next_transfer_date: Option<Integer>,
    /// Unique identifier of the received gift for the bot;
    /// only present for gifts received on behalf of business accounts.
    pub owned_gift_id: Option<String>,
    /// Number of Telegram Stars that must be paid to transfer the gift;
    /// omitted if the bot cannot transfer the gift.
    pub transfer_star_count: Option<Integer>,
}

/// Gifts a Telegram Premium subscription to the given user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct GiftPremiumSubscription {
    month_count: Integer,
    star_count: Integer,
    user_id: Integer,
    #[serde(flatten)]
    text: Option<RawGiftText>,
}

impl GiftPremiumSubscription {
    /// Creates a new `GiftPremiumSubscription`.
    ///
    /// # Arguments
    ///
    /// * `month_count` - Number of months the Telegram Premium subscription will be active for the user;
    ///   must be one of 3, 6, or 12.
    /// * `star_count` - Number of Telegram Stars to pay for the Telegram Premium subscription;
    ///   must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months.
    /// * `user_id` - Unique identifier of the target user who will receive a Telegram Premium subscription.
    pub fn new(month_count: Integer, star_count: Integer, user_id: Integer) -> Self {
        Self {
            month_count,
            star_count,
            user_id,
            text: None,
        }
    }

    /// Sets a new text.
    ///
    /// # Arguments
    ///
    /// * `value` - Text that will be shown along with the service message about the subscription; 0-128 characters
    pub fn with_text<T>(mut self, value: T) -> Self
    where
        T: Into<InputText>,
    {
        self.text = Some(RawGiftText::from(value));
        self
    }
}

impl Method for GiftPremiumSubscription {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("giftPremiumSubscription", self)
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
    #[serde(flatten)]
    text: Option<RawGiftText>,
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
        T: Into<InputText>,
    {
        self.text = Some(RawGiftText::from(value));
        self
    }
}

impl Method for SendGift {
    type Response = bool;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("sendGift", self)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
struct RawGiftText {
    text: Option<String>,
    text_entities: Option<TextEntities>,
    text_parse_mode: Option<ParseMode>,
}

impl<T> From<T> for RawGiftText
where
    T: Into<InputText>,
{
    fn from(value: T) -> Self {
        let value = value.into();
        Self {
            text: Some(value.data),
            text_entities: value.entities,
            text_parse_mode: value.parse_mode,
        }
    }
}

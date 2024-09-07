use serde::{Deserialize, Serialize};

use crate::types::{Chat, Integer, Message, User};

#[cfg(test)]
mod tests;

/// Represents a message about a scheduled giveaway.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Giveaway {
    /// The list of chats which the user must join to participate in the giveaway.
    pub chats: Vec<Chat>,
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected.
    pub winners_selection_date: Integer,
    /// The number of users which are supposed to be selected as winners of the giveaway.
    pub winner_count: Integer,
    /// A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries
    /// from which eligible users for the giveaway must come.
    ///
    /// If empty, then all users can participate in the giveaway.
    /// Users with a phone number that was bought on Fragment can always participate in giveaways.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    /// Whether the list of giveaway winners will be visible to everyone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,
    /// Whether only users who join the chats after the giveaway started should be eligible to win.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<Integer>,
    /// Description of additional giveaway prize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}

impl Giveaway {
    /// Creates a new `Giveaway`.
    ///
    /// # Arguments
    ///
    /// * `chats` - The list of chats which the user must join to participate in the giveaway.
    /// * `winners_selection_date` - Point in time (Unix timestamp) when winners of the giveaway will be selected.
    /// * `winner_count` - The number of users which are supposed to be selected as winners of the giveaway.
    pub fn new<A, B>(chats: A, winners_selection_date: Integer, winner_count: Integer) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<Chat>,
    {
        Self {
            chats: chats.into_iter().map(Into::into).collect(),
            winners_selection_date,
            winner_count,
            country_codes: None,
            has_public_winners: None,
            only_new_members: None,
            premium_subscription_month_count: None,
            prize_description: None,
        }
    }

    /// Sets a new list of country codes.
    ///
    /// # Arguments
    ///
    /// * `value` - A list of two-letter ISO 3166-1 alpha-2 country codes.
    pub fn with_country_codes<A, B>(mut self, value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        self.country_codes = Some(value.into_iter().map(Into::into).collect());
        self
    }

    /// Sets a new value for a `has_public_winners` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the list of giveaway winners will be visible to everyone.
    pub fn with_has_public_winners(mut self, value: bool) -> Self {
        self.has_public_winners = Some(value);
        self
    }

    /// Sets a new value for an `only_new_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether only users who join the chats after the giveaway started should be eligible to win.
    pub fn with_only_new_members(mut self, value: bool) -> Self {
        self.only_new_members = Some(value);
        self
    }

    /// Sets a new number of premium subscription months.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of months the Telegram Premium subscription won from the giveaway will be active for.
    pub fn with_premium_subscription_month_count(mut self, value: Integer) -> Self {
        self.premium_subscription_month_count = Some(value);
        self
    }

    /// Sets a new description of additional prizes.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of additional giveaway prize.
    pub fn with_prize_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.prize_description = Some(value.into());
        self
    }
}

/// Represents a service message about the creation of a scheduled giveaway.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GiveawayCreated {
    /// The number of Telegram Stars to be split between giveaway winners;
    /// for Telegram Star giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<Integer>,
}

impl GiveawayCreated {
    /// Sets a new prize star count.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of Telegram Stars to be split between giveaway winners;
    ///             for Telegram Star giveaways only.
    pub fn with_prize_star_count(mut self, value: Integer) -> Self {
        self.prize_star_count = Some(value);
        self
    }
}

/// Represents a service message about the completion of a giveaway without public winners.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway.
    pub winner_count: Integer,
    /// Message with the giveaway that was completed, if it wasn't deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<Message>>,
    /// Number of undistributed prizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<Integer>,
}

impl GiveawayCompleted {
    /// Creates a new `GiveawayCompleted`.
    ///
    /// # Arguments
    ///
    /// * `winner_count` - Number of winners in the giveaway.
    pub fn new(winner_count: Integer) -> Self {
        Self {
            winner_count,
            unclaimed_prize_count: None,
            giveaway_message: None,
        }
    }

    /// Sets a new giveaway message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message with the giveaway that was completed.
    pub fn with_giveaway_message(mut self, value: Message) -> Self {
        self.giveaway_message = Some(Box::new(value));
        self
    }

    /// Sets a new unclaimed prize count.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of undistributed prizes.
    pub fn with_unclaimed_prize_count(mut self, value: Integer) -> Self {
        self.unclaimed_prize_count = Some(value);
        self
    }
}

/// Represents a message about the completion of a giveaway with public winners.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GiveawayWinners {
    /// The chat that created the giveaway.
    pub chat: Chat,
    /// Identifier of the messsage with the giveaway in the chat.
    pub giveaway_message_id: Integer,
    /// Total number of winners in the giveaway.
    pub winner_count: Integer,
    /// List of up to 100 winners of the giveaway.
    pub winners: Vec<User>,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected.
    pub winners_selection_date: Integer,
    /// The number of other chats the user had to join in order to be eligible for the giveaway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<Integer>,
    /// Whether only users who had joined the chats after the giveaway started were eligible to win.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<Integer>,
    /// Description of additional giveaway prize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
    /// Number of undistributed prizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<Integer>,
    /// Whether the giveaway was canceled because the payment for it was refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
}

impl GiveawayWinners {
    /// Creates a new `GiveawayWinners`.
    ///
    ///
    /// # Arguments
    ///
    /// * `chat` - The chat that created the giveaway.
    /// * `giveaway_message_id` - Identifier of the messsage with the giveaway in the chat.
    /// * `winner_count` - Total number of winners in the giveaway.
    /// * `winners` - List of up to 100 winners of the giveaway.
    /// * `winners_selection_date` - Point in time (Unix timestamp) when winners of the giveaway were selected.
    pub fn new<A, B>(
        chat: A,
        giveaway_message_id: Integer,
        winner_count: Integer,
        winners: B,
        winners_selection_date: Integer,
    ) -> Self
    where
        A: Into<Chat>,
        B: IntoIterator<Item = User>,
    {
        Self {
            chat: chat.into(),
            giveaway_message_id,
            winner_count,
            winners: winners.into_iter().collect(),
            winners_selection_date,
            additional_chat_count: None,
            only_new_members: None,
            premium_subscription_month_count: None,
            prize_description: None,
            unclaimed_prize_count: None,
            was_refunded: None,
        }
    }

    /// Sets a new number of additional chats.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of other chats the user had to join in order to be eligible for the giveaway.
    pub fn with_additional_chat_count(mut self, value: Integer) -> Self {
        self.additional_chat_count = Some(value);
        self
    }

    /// Sets a new value for an `only_new_members` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether only users who had joined the chats after the giveaway started were eligible to win.
    pub fn with_only_new_members(mut self, value: bool) -> Self {
        self.only_new_members = Some(value);
        self
    }

    /// Sets a new number of premium subscription months.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of months the Telegram Premium subscription won from the giveaway will be active for.
    pub fn with_premium_subscription_month_count(mut self, value: Integer) -> Self {
        self.premium_subscription_month_count = Some(value);
        self
    }

    /// Sets a new prize description.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of additional giveaway prize.
    pub fn with_prize_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.prize_description = Some(value.into());
        self
    }

    /// Sets a new number of unclaimed prizes.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of undistributed prizes.
    pub fn with_unclaimed_prize_count(mut self, value: Integer) -> Self {
        self.unclaimed_prize_count = Some(value);
        self
    }

    /// Sets a new value for a `was_refunded` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the giveaway was canceled because the payment for it was refunded.
    pub fn with_was_refunded(mut self, value: bool) -> Self {
        self.was_refunded = Some(value);
        self
    }
}

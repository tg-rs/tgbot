use serde::{Deserialize, Serialize};

use crate::types::{Chat, Integer, Message, User};

/// Represents a message about a scheduled giveaway.
#[serde_with::skip_serializing_none]
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
    pub country_codes: Option<Vec<String>>,
    /// Whether the list of giveaway winners will be visible to everyone.
    pub has_public_winners: Option<bool>,
    /// Whether only users who join the chats after the giveaway started should be eligible to win.
    pub only_new_members: Option<bool>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for.
    pub premium_subscription_month_count: Option<Integer>,
    /// Description of additional giveaway prize.
    pub prize_description: Option<String>,
    /// The number of Telegram Stars to be split between giveaway winners;
    /// for Telegram Star giveaways only.
    pub prize_star_count: Option<Integer>,
}

/// Represents a service message about the creation of a scheduled giveaway.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GiveawayCreated {
    /// The number of Telegram Stars to be split between giveaway winners;
    /// for Telegram Star giveaways only.
    pub prize_star_count: Option<Integer>,
}

/// Represents a service message about the completion of a giveaway without public winners.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway.
    pub winner_count: Integer,
    /// Message with the giveaway that was completed, if it wasn't deleted.
    pub giveaway_message: Option<Box<Message>>,
    /// Whether the giveaway is a Telegram Star giveaway.
    ///
    /// Otherwise, currently, the giveaway is a Telegram Premium giveaway.
    pub is_star_giveaway: Option<bool>,
    /// Number of undistributed prizes.
    pub unclaimed_prize_count: Option<Integer>,
}

/// Represents a message about the completion of a giveaway with public winners.
#[serde_with::skip_serializing_none]
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
    pub additional_chat_count: Option<Integer>,
    /// Whether only users who had joined the chats after the giveaway started were eligible to win.
    pub only_new_members: Option<bool>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for.
    pub premium_subscription_month_count: Option<Integer>,
    /// Description of additional giveaway prize.
    pub prize_description: Option<String>,
    /// The number of Telegram Stars that were split between giveaway winners;
    /// for Telegram Star giveaways only.
    pub prize_star_count: Option<Integer>,
    /// Number of undistributed prizes.
    pub unclaimed_prize_count: Option<Integer>,
    /// Whether the giveaway was canceled because the payment for it was refunded.
    pub was_refunded: Option<bool>,
}

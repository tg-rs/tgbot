use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload, PayloadError},
    types::{Chat, Gift, Integer, PaidMedia, User},
};

/// Contains a list of Telegram Star transactions.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StarTransactions {
    /// The list of transactions.
    pub transactions: Vec<StarTransaction>,
}

/// Describes a Telegram Star transaction.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StarTransaction {
    /// Integer amount of Telegram Stars transferred by the transaction.
    pub amount: Integer,
    /// Date the transaction was created in Unix time.
    pub date: Integer,
    /// Unique identifier of the transaction.
    ///
    /// Coincides with the identifier of the original transaction
    /// for refund transactions.
    ///
    /// Coincides with SuccessfulPayment.telegram_payment_charge_id
    /// for successful incoming payments from users.
    pub id: String,
    /// The number of 1/1000000000 shares of
    /// Telegram Stars transferred by the transaction;
    /// from 0 to 999999999.
    pub nanostar_amount: Option<Integer>,
    /// Source of an incoming transaction.
    pub source: Option<TransactionPartner>,
    /// Receiver of an outgoing transaction.
    pub receiver: Option<TransactionPartner>,
}

/// Describes the affiliate program that issued the affiliate commission received via this transaction.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct TransactionPartnerAffiliateProgramParameters {
    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars
    /// received by the affiliate program sponsor from referred users.
    pub commission_per_mille: Integer,
    /// Information about the bot that sponsored the affiliate program
    pub sponsor_user: Option<User>,
}

/// Contains information about the affiliate that received a commission via this transaction.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AffiliateInfo {
    /// Integer amount of Telegram Stars received by the affiliate from the transaction,
    /// rounded to 0; can be negative for refunds.
    pub amount: Integer,
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars
    /// received by the bot from referred users.
    pub commission_per_mille: Integer,
    /// The chat that received an affiliate commission if it was received by a chat.
    pub affiliate_chat: Option<Chat>,
    /// The bot or the user that received an affiliate commission if it was received by a bot or a user.
    pub affiliate_user: Option<User>,
    /// The number of 1/1000000000 shares of Telegram Stars received by the affiliate;
    /// from -999999999 to 999999999; can be negative for refunds.
    pub nanostar_amount: Option<Integer>,
}

/// Describes a transaction with a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransactionPartnerChatParameters {
    /// Information about the chat.
    pub chat: Chat,
    /// The gift sent to the chat by the bot.
    pub gift: Option<Gift>,
}

/// Type of the partner user transaction.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionPartnerUserType {
    /// For direct transfers from managed business accounts.
    BusinessAccountTransfer,
    /// For gifts sent by the bot.
    GiftPurchase,
    /// For payments via invoices.
    InvoicePayment,
    /// For payments for paid media.
    PaidMediaPayment,
    /// For Telegram Premium subscriptions gifted by the bot.
    PremiumPurchase,
}

/// Describes a transaction with a user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransactionPartnerUserParameters {
    /// Type of the transaction.
    pub transaction_type: TransactionPartnerUserType,
    /// Information about the user.
    pub user: User,
    /// Information about the affiliate that received a commission via this transaction.
    pub affiliate: Option<AffiliateInfo>,
    /// The gift sent to the user by the bot.
    pub gift: Option<String>,
    /// Bot-specified invoice payload.
    pub invoice_payload: Option<String>,
    /// Information about the paid media bought by the user.
    pub paid_media: Option<Vec<PaidMedia>>,
    /// Bot-specified paid media payload.
    pub paid_media_payload: Option<String>,
    /// Number of months the gifted Telegram Premium subscription will be active for;
    /// for “premium_purchase” transactions only.
    pub premium_subscription_duration: Option<Integer>,
    /// The duration of the paid subscription.
    pub subscription_period: Option<Integer>,
}

/// Describes the source of a transaction, or its recipient for outgoing transactions.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(from = "RawTransactionPartner", into = "RawTransactionPartner")]
pub enum TransactionPartner {
    /// Describes the affiliate program that issued the affiliate commission received via this transaction.
    AffiliateProgram(TransactionPartnerAffiliateProgramParameters),
    /// Describes a transaction with a chat.
    Chat(TransactionPartnerChatParameters),
    /// Describes a withdrawal transaction with Fragment.
    Fragment(Option<RevenueWithdrawalState>),
    /// Describes a transaction with an unknown source or recipient.
    Other,
    /// Describes a withdrawal transaction to the Telegram Ads platform.
    TelegramAds,
    /// Describes a transaction with payment for paid broadcasting.
    TelegramApi {
        /// The number of successful requests that exceeded regular limits and were therefore billed.
        request_count: Integer,
    },
    /// Describes a transaction with a user.
    User(TransactionPartnerUserParameters),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawTransactionPartner {
    AffiliateProgram(TransactionPartnerAffiliateProgramParameters),
    Chat(TransactionPartnerChatParameters),
    Fragment {
        withdrawal_state: Option<RevenueWithdrawalState>,
    },
    Other {},
    TelegramAds {},
    TelegramApi {
        request_count: Integer,
    },
    User(TransactionPartnerUserParameters),
}

impl From<RawTransactionPartner> for TransactionPartner {
    fn from(value: RawTransactionPartner) -> Self {
        match value {
            RawTransactionPartner::AffiliateProgram(parameters) => Self::AffiliateProgram(parameters),
            RawTransactionPartner::Chat(parameters) => Self::Chat(parameters),
            RawTransactionPartner::Fragment { withdrawal_state } => Self::Fragment(withdrawal_state),
            RawTransactionPartner::Other {} => Self::Other,
            RawTransactionPartner::TelegramAds {} => Self::TelegramAds,
            RawTransactionPartner::TelegramApi { request_count } => Self::TelegramApi { request_count },
            RawTransactionPartner::User(parameters) => Self::User(parameters),
        }
    }
}

impl From<TransactionPartner> for RawTransactionPartner {
    fn from(value: TransactionPartner) -> Self {
        match value {
            TransactionPartner::AffiliateProgram(parameters) => Self::AffiliateProgram(parameters),
            TransactionPartner::Chat(parameters) => Self::Chat(parameters),
            TransactionPartner::Fragment(withdrawal_state) => Self::Fragment { withdrawal_state },
            TransactionPartner::Other => Self::Other {},
            TransactionPartner::TelegramAds => Self::TelegramAds {},
            TransactionPartner::TelegramApi { request_count } => Self::TelegramApi { request_count },
            TransactionPartner::User(parameters) => Self::User(parameters),
        }
    }
}

/// Describes the state of a revenue withdrawal operation.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(from = "RawRevenueWithdrawalState", into = "RawRevenueWithdrawalState")]
pub enum RevenueWithdrawalState {
    /// The withdrawal failed and the transaction was refunded.
    Failed,
    /// The withdrawal is in progress.
    Pending,
    /// The withdrawal succeeded.
    Succeeded {
        /// Date the withdrawal was completed in Unix time.
        date: Integer,
        /// An HTTPS URL that can be used to see transaction details.
        url: String,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawRevenueWithdrawalState {
    Failed {},
    Pending {},
    Succeeded { date: Integer, url: String },
}

impl From<RawRevenueWithdrawalState> for RevenueWithdrawalState {
    fn from(value: RawRevenueWithdrawalState) -> Self {
        use self::RawRevenueWithdrawalState::*;
        match value {
            Failed {} => Self::Failed,
            Pending {} => Self::Pending,
            Succeeded { date, url } => Self::Succeeded { date, url },
        }
    }
}

impl From<RevenueWithdrawalState> for RawRevenueWithdrawalState {
    fn from(value: RevenueWithdrawalState) -> Self {
        use self::RevenueWithdrawalState::*;
        match value {
            Failed => Self::Failed {},
            Pending => Self::Pending {},
            Succeeded { date, url } => Self::Succeeded { date, url },
        }
    }
}

/// Returns the bot's Telegram Star transactions in chronological order.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct GetStarTransactions {
    offset: Option<Integer>,
    limit: Option<Integer>,
}

impl GetStarTransactions {
    /// Sets a new offset.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of transactions to skip in the response.
    pub fn with_offset(mut self, value: Integer) -> Self {
        self.offset = Some(value);
        self
    }

    /// Sets a new limit.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum number of transactions to be retrieved.
    ///
    /// Values between 1-100 are accepted.
    /// Defaults to 100.
    pub fn with_limit(mut self, value: Integer) -> Self {
        self.limit = Some(value);
        self
    }
}

impl Method for GetStarTransactions {
    type Response = StarTransactions;

    fn into_payload(self) -> Result<Payload, PayloadError> {
        Payload::json("getStarTransactions", self)
    }
}

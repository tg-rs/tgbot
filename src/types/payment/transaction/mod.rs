use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Integer, PaidMedia, User},
};

#[cfg(test)]
mod tests;

/// Contains a list of Telegram Star transactions.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StarTransactions {
    /// The list of transactions.
    pub transactions: Vec<StarTransaction>,
}

impl<T> From<T> for StarTransactions
where
    T: IntoIterator<Item = StarTransaction>,
{
    fn from(value: T) -> Self {
        Self {
            transactions: value.into_iter().collect(),
        }
    }
}

/// Describes a Telegram Star transaction.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StarTransaction {
    amount: Integer,
    date: Integer,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanostar_amount: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<TransactionPartner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver: Option<TransactionPartner>,
}

impl StarTransaction {
    /// Creates a new `StarTransaction`.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of Telegram Stars transferred by the transaction.
    /// * `date` - Date the transaction was created in Unix time.
    /// * `id` -  Unique identifier of the transaction.
    ///           Coincides with the identifer of the original transaction for refund transactions.
    ///           Coincides with `telegram_payment_charge_id` of [`crate::types::SuccessfulPayment`]
    ///           for successful incoming payments from users.
    pub fn new<T>(amount: Integer, date: Integer, id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            amount,
            date,
            id: id.into(),
            nanostar_amount: None,
            source: None,
            receiver: None,
        }
    }

    /// Sets a new nanostar amount.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of 1/1000000000 shares of Telegram Stars transferred by the transaction;
    ///             from 0 to 999999999.
    pub fn with_nanostar_amount(mut self, value: Integer) -> Self {
        self.nanostar_amount = Some(value);
        self
    }

    /// Sets a new source.
    ///
    /// # Arguments
    ///
    /// * `value` - Source of an incoming transaction.
    ///             E.g., a user purchasing goods or services, Fragment refunding a failed withdrawal.
    ///             Only for incoming transactions.
    pub fn with_source(mut self, value: TransactionPartner) -> Self {
        self.source = Some(value);
        self
    }

    /// Sets a new receiver.
    ///
    /// # Arguments
    ///
    /// * `value` - Receiver of an outgoing transaction.
    ///             E.g., a user for a purchase refund, Fragment for a withdrawal.
    ///             Only for outgoing transactions.
    pub fn with_receiver(mut self, value: TransactionPartner) -> Self {
        self.receiver = Some(value);
        self
    }
}

/// Describes the affiliate program that issued the affiliate commission received via this transaction.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct TransactionPartnerAffiliateProgramParameters {
    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars
    /// received by the affiliate program sponsor from referred users.
    pub commission_per_mille: Integer,
    /// Information about the bot that sponsored the affiliate program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor_user: Option<User>,
}

impl TransactionPartnerAffiliateProgramParameters {
    /// Creates a new `TransactionPartnerAffiliateProgramParameters`.
    ///
    /// # Arguments
    ///
    /// * `commission_per_mille` - The number of Telegram Stars received by the bot.
    pub fn new(commission_per_mille: Integer) -> Self {
        Self {
            commission_per_mille,
            sponsor_user: None,
        }
    }

    /// Sets a new sponsor user.
    ///
    /// # Arguments
    ///
    /// * `value` - Information about the bot that sponsored the affiliate program.
    pub fn with_sponsor_user(mut self, value: User) -> Self {
        self.sponsor_user = Some(value);
        self
    }
}

/// Describes a transaction with a user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct TransactionPartnerUserParameters {
    /// Information about the user.
    pub user: User,
    /// The gift sent to the user by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<String>,
    /// Bot-specified invoice payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,
    /// Information about the paid media bought by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Vec<PaidMedia>>,
    /// Bot-specified paid media payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<String>,
    /// The duration of the paid subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<Integer>,
}

impl TransactionPartnerUserParameters {
    /// Creates a new `TransactionPartnerUserParameters`.
    ///
    /// # Arguments
    ///
    /// * `user` - Information about the user.
    pub fn new(user: User) -> Self {
        Self {
            user,
            gift: None,
            invoice_payload: None,
            paid_media: None,
            paid_media_payload: None,
            subscription_period: None,
        }
    }

    /// Sets a new gift.
    ///
    /// # Arguments
    ///
    /// * `value` - The gift sent to the user by the bot.
    pub fn with_gift<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.gift = Some(value.into());
        self
    }

    /// Sets a new invoice payload.
    ///
    /// # Arguments
    ///
    /// * `value` - Bot-specified invoice payload.
    pub fn with_invoice_payload<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.invoice_payload = Some(value.into());
        self
    }

    /// Sets a new paid media.
    ///
    /// # Arguments
    ///
    /// * `value` - Information about the paid media bought by the user.
    pub fn with_paid_media<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PaidMedia>,
    {
        self.paid_media = Some(value.into_iter().collect());
        self
    }

    /// Sets a new paid media payload.
    ///
    /// # Arguments
    ///
    /// * `value` - Bot-specified paid media payload.
    pub fn with_paid_media_payload<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.paid_media_payload = Some(value.into());
        self
    }

    /// Sets a new subscription period.
    ///
    /// # Arguments
    ///
    /// * `value` - The duration of the paid subscription.
    pub fn with_subscription_period(mut self, value: Integer) -> Self {
        self.subscription_period = Some(value);
        self
    }
}

/// Describes the source of a transaction, or its recipient for outgoing transactions.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(from = "RawTransactionPartner", into = "RawTransactionPartner")]
pub enum TransactionPartner {
    /// Describes the affiliate program that issued the affiliate commission received via this transaction.
    AffiliateProgram(TransactionPartnerAffiliateProgramParameters),
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

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawTransactionPartner {
    AffiliateProgram(TransactionPartnerAffiliateProgramParameters),
    Fragment {
        #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct GetStarTransactions {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    fn into_payload(self) -> Payload {
        Payload::json("getStarTransactions", self)
    }
}

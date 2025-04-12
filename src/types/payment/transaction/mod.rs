use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Chat, Gift, Integer, PaidMedia, User},
};

#[cfg(test)]
mod tests;

/// Contains a list of Telegram Star transactions.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StarTransaction {
    amount: Integer,
    date: Integer,
    id: String,
    nanostar_amount: Option<Integer>,
    source: Option<TransactionPartner>,
    receiver: Option<TransactionPartner>,
}

impl StarTransaction {
    /// Creates a new `StarTransaction`.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of Telegram Stars transferred by the transaction.
    /// * `date` - Date the transaction was created in Unix time.
    /// * `id` -  Unique identifier of the transaction;
    ///   coincides with the identifer of the original transaction for refund transactions;
    ///   coincides with `telegram_payment_charge_id` of [`crate::types::SuccessfulPayment`]
    ///   for successful incoming payments from users.
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
    ///   from 0 to 999999999.
    pub fn with_nanostar_amount(mut self, value: Integer) -> Self {
        self.nanostar_amount = Some(value);
        self
    }

    /// Sets a new source.
    ///
    /// # Arguments
    ///
    /// * `value` - Source of an incoming transaction.
    ///   E.g., a user purchasing goods or services, Fragment refunding a failed withdrawal.
    ///   Only for incoming transactions.
    pub fn with_source(mut self, value: TransactionPartner) -> Self {
        self.source = Some(value);
        self
    }

    /// Sets a new receiver.
    ///
    /// # Arguments
    ///
    /// * `value` - Receiver of an outgoing transaction.
    ///   E.g., a user for a purchase refund, Fragment for a withdrawal.
    ///   Only for outgoing transactions.
    pub fn with_receiver(mut self, value: TransactionPartner) -> Self {
        self.receiver = Some(value);
        self
    }
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

impl AffiliateInfo {
    /// Creates a new `AffiliateInfo`.
    ///
    /// # Arguments
    ///
    /// * `amount` - Integer amount of Telegram Stars received by the affiliate from the transaction
    /// * `comission_per_mille` - The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars
    pub fn new(amount: Integer, commission_per_mille: Integer) -> Self {
        Self {
            amount,
            commission_per_mille,
            affiliate_chat: None,
            affiliate_user: None,
            nanostar_amount: None,
        }
    }

    /// Sets a new affiliate chat.
    ///
    /// # Arguments
    ///
    /// * `value` - The chat that received an affiliate commission if it was received by a chat.
    pub fn with_affiliate_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.affiliate_chat = Some(value.into());
        self
    }

    /// Sets a new affiliate user.
    ///
    /// # Arguments
    ///
    /// * `value` - The bot or the user that received an affiliate commission if it was received by a bot or a user.
    pub fn with_affiliate_user(mut self, value: User) -> Self {
        self.affiliate_user = Some(value);
        self
    }

    /// Sets a new nanostar amount.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of 1/1000000000 shares of Telegram Stars received by the affiliate.
    pub fn with_nanostar_amount(mut self, value: Integer) -> Self {
        self.nanostar_amount = Some(value);
        self
    }
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

impl TransactionPartnerChatParameters {
    /// Creates a new `TransactionPartnerChatParameters`.
    ///
    /// # Arguments
    ///
    /// * `chat` - Information about the chat.
    pub fn new<T>(chat: T) -> Self
    where
        T: Into<Chat>,
    {
        Self {
            chat: chat.into(),
            gift: None,
        }
    }

    /// Sets a new gift
    ///
    /// # Arguments
    ///
    /// * `value` - The gift sent to the chat by the bot.
    pub fn with_gift(mut self, value: Gift) -> Self {
        self.gift = Some(value);
        self
    }
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

impl TransactionPartnerUserParameters {
    /// Creates a new `TransactionPartnerUserParameters`.
    ///
    /// # Arguments
    ///
    /// * `transaction_type` - Type of the transaction.
    /// * `user` - Information about the user.
    pub fn new(transaction_type: TransactionPartnerUserType, user: User) -> Self {
        Self {
            transaction_type,
            user,
            affiliate: None,
            gift: None,
            invoice_payload: None,
            paid_media: None,
            paid_media_payload: None,
            premium_subscription_duration: None,
            subscription_period: None,
        }
    }

    /// Sets a new affiliate.
    ///
    /// # Arguments
    ///
    /// * `value` - Information about the affiliate that received a commission via this transaction.
    pub fn with_affiliate(mut self, value: AffiliateInfo) -> Self {
        self.affiliate = Some(value);
        self
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

    /// Sets a new premium subscription duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Number of months the gifted Telegram Premium subscription will be active for;
    ///   for “premium_purchase” transactions only.
    pub fn with_premium_subscription_duration(mut self, value: Integer) -> Self {
        self.premium_subscription_duration = Some(value);
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(from = "RawTransactionPartner", into = "RawTransactionPartner")]
#[allow(clippy::large_enum_variant)]
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
#[allow(clippy::large_enum_variant)]
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

    fn into_payload(self) -> Payload {
        Payload::json("getStarTransactions", self)
    }
}

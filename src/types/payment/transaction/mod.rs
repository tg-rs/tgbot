use serde::{Deserialize, Serialize};

use crate::types::{Integer, User};

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
    /// Number of Telegram Stars transferred by the transaction.
    amount: Integer,
    /// Date the transaction was created in Unix time.
    date: Integer,
    /// Unique identifier of the transaction.
    ///
    /// Coincides with the identifer of the original transaction for refund transactions.
    /// Coincides with `telegram_payment_charge_id` of [`crate::types::SuccessfulPayment`] for successful incoming payments from users.
    id: String,
    /// Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal).
    ///
    /// Only for incoming transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<TransactionPartner>,
    ///  Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal).
    ///
    /// Only for outgoing transactions.
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
    pub fn new<T>(amount: Integer, date: Integer, id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            amount,
            date,
            id: id.into(),
            source: None,
            receiver: None,
        }
    }

    /// Sets a new source.
    ///
    /// # Arguments
    ///
    /// * `value` - Source of an incoming transaction.
    pub fn with_source(mut self, value: TransactionPartner) -> Self {
        self.source = Some(value);
        self
    }

    /// Sets a new receiver.
    ///
    /// # Arguments
    ///
    /// * `value` - Receiver of an outgoing transaction.
    pub fn with_receiver(mut self, value: TransactionPartner) -> Self {
        self.receiver = Some(value);
        self
    }
}

/// Describes the source of a transaction, or its recipient for outgoing transactions.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(from = "RawTransactionPartner", into = "RawTransactionPartner")]
pub enum TransactionPartner {
    /// Describes a withdrawal transaction with Fragment.
    Fragment(Option<RevenueWithdrawalState>),
    /// Describes a transaction with an unknown source or recipient.
    Other,
    /// Describes a transaction with a user.
    User(User),
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawTransactionPartner {
    Fragment {
        #[serde(skip_serializing_if = "Option::is_none")]
        withdrawal_state: Option<RevenueWithdrawalState>,
    },
    Other {},
    User {
        user: User,
    },
}

impl From<RawTransactionPartner> for TransactionPartner {
    fn from(value: RawTransactionPartner) -> Self {
        match value {
            RawTransactionPartner::Fragment { withdrawal_state } => Self::Fragment(withdrawal_state),
            RawTransactionPartner::Other {} => Self::Other,
            RawTransactionPartner::User { user } => Self::User(user),
        }
    }
}

impl From<TransactionPartner> for RawTransactionPartner {
    fn from(value: TransactionPartner) -> Self {
        match value {
            TransactionPartner::Fragment(withdrawal_state) => Self::Fragment { withdrawal_state },
            TransactionPartner::Other => Self::Other {},
            TransactionPartner::User(user) => Self::User { user },
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

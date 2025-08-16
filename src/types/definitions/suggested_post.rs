use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Integer, Message, StarAmount},
};

/// Approves a suggested post in a direct messages chat.
///
/// The bot must have the 'can_post_messages' administrator right in the corresponding channel chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct ApproveSuggestedPost {
    chat_id: Integer,
    message_id: Integer,
    send_date: Option<Integer>,
}

impl ApproveSuggestedPost {
    /// Creates a new `ApproveSuggestedPost`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target direct messages chat
    /// * `message_id` - Identifier of a suggested post message to approve
    pub fn new(chat_id: Integer, message_id: Integer) -> Self {
        Self {
            chat_id,
            message_id,
            send_date: None,
        }
    }

    /// Sets a new send date.
    ///
    /// # Arguments
    ///
    /// * `value` - Point in time (Unix timestamp) when the post is expected to be published.
    ///
    /// Omit if the date has already been specified when the suggested post was created.
    ///
    /// If specified, then the date must be not more than 2678400 seconds (30 days) in the future.
    pub fn with_send_date(mut self, value: Integer) -> Self {
        self.send_date = Some(value);
        self
    }
}

impl Method for ApproveSuggestedPost {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("approveSuggestedPost", self)
    }
}

/// Declines a suggested post in a direct messages chat.
///
/// The bot must have the 'can_manage_direct_messages' administrator right in the corresponding channel chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct DeclineSuggestedPost {
    chat_id: Integer,
    message_id: Integer,
    comment: Option<String>,
}

impl DeclineSuggestedPost {
    /// Creates a new `DeclineSuggestedPost`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier for the target direct messages chat
    /// * `message_id` - Identifier of a suggested post message to decline
    pub fn new(chat_id: Integer, message_id: Integer) -> Self {
        Self {
            chat_id,
            message_id,
            comment: None,
        }
    }

    /// Sets a new comment.
    ///
    /// # Arguments
    ///
    /// * `value` - Comment for the creator of the suggested post; 0-128 characters.
    pub fn with_comment<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.comment = Some(value.into());
        self
    }
}

impl Method for DeclineSuggestedPost {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("declineSuggestedPost", self)
    }
}

/// Describes a service message about the approval of a suggested post.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SuggestedPostApproved {
    /// Date when the post will be published.
    pub send_date: Integer,
    /// Amount paid for the post.
    pub price: Option<SuggestedPostPrice>,
    /// Message containing the suggested post.
    ///
    /// Note that the Message object in this field will not contain the reply_to_message field
    /// even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
}

impl SuggestedPostApproved {
    /// Creates a new `SuggestedPostApproved`.
    ///
    /// # Arguments
    ///
    /// * `send_date` - Date when the post will be published.
    pub fn new(send_date: Integer) -> Self {
        Self {
            send_date,
            price: None,
            suggested_post_message: None,
        }
    }

    /// Sets a new price.
    ///
    /// # Arguments
    ///
    /// * `value` - Amount paid for the post.
    pub fn with_price(mut self, value: SuggestedPostPrice) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets a new suggested post message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message containing the suggested post.
    pub fn with_suggested_post_message(mut self, value: Message) -> Self {
        self.suggested_post_message = Some(value);
        self
    }
}

/// Describes a service message about the failed approval of a suggested post.
///
/// Currently, only caused by insufficient user funds at the time of approval.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SuggestedPostApprovalFailed {
    /// Expected price of the post.
    pub price: SuggestedPostPrice,
    /// Message containing the suggested post whose approval has failed.
    ///
    /// Note that the Message object in this field will not contain the reply_to_message field
    /// even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
}

impl SuggestedPostApprovalFailed {
    /// Creates a new `SuggestedPostApprovalFailed`.
    ///
    /// # Arguments
    ///
    /// * `price` - Expected price of the post.
    pub fn new(price: SuggestedPostPrice) -> Self {
        Self {
            price,
            suggested_post_message: None,
        }
    }

    /// Sets a new suggested post message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message containing the suggested post.
    pub fn with_suggested_post_message(mut self, value: Message) -> Self {
        self.suggested_post_message = Some(value);
        self
    }
}

/// Describes a service message about the rejection of a suggested post.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SuggestedPostDeclined {
    /// Comment with which the post was declined.
    pub comment: Option<String>,
    /// Message containing the suggested post.
    ///
    /// Note that the Message object in this field will not contain the reply_to_message field
    /// even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
}

impl SuggestedPostDeclined {
    /// Sets a new comment.
    ///
    /// # Arguments
    ///
    /// * `value` - Comment with which the post was declined.
    pub fn with_comment<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.comment = Some(value.into());
        self
    }

    /// Sets a new suggested post message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message containing the suggested post.
    pub fn with_suggested_post_message(mut self, value: Message) -> Self {
        self.suggested_post_message = Some(value);
        self
    }
}

/// Contains information about a suggested post.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SuggestedPostInfo {
    /// State of the suggested post.
    pub state: SuggestedPostState,
    /// Proposed price of the post.
    ///
    /// If the field is omitted, then the post is unpaid.
    pub price: Option<SuggestedPostPrice>,
    /// Proposed send date of the post.
    ///
    /// If the field is omitted, then the post can be published at any time within 30 days
    /// at the sole discretion of the user or administrator who approves it.
    pub send_date: Option<Integer>,
}

impl SuggestedPostInfo {
    /// Creates a new `SuggestedPostInfo`.
    ///
    /// # Arguments
    ///
    /// * `state` - State of the suggested post.
    pub fn new(state: SuggestedPostState) -> Self {
        Self {
            state,
            price: None,
            send_date: None,
        }
    }

    /// Sets a new price.
    ///
    /// # Arguments
    ///
    /// * `value` - Proposed price of the post.
    pub fn with_price(mut self, value: SuggestedPostPrice) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets a new send date.
    ///
    /// # Arguments
    ///
    /// * `value` - Proposed send date of the post.
    pub fn with_send_date(mut self, value: Integer) -> Self {
        self.send_date = Some(value);
        self
    }
}

/// Describes a service message about a successful payment for a suggested post.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SuggestedPostPaid {
    /// Currency in which the payment was made.
    ///
    /// Currently, one of “XTR” for Telegram Stars or “TON” for toncoins
    pub currency: String,
    /// The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only.
    pub amount: Option<Integer>,
    /// The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only.
    pub star_amount: Option<StarAmount>,
    /// Message containing the suggested post.
    ///
    /// Note that the Message object in this field will not contain the reply_to_message field even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
}

impl SuggestedPostPaid {
    /// Creates a new `SuggestedPostPaid`.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency in which the payment was made.
    pub fn new<T>(currency: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            currency: currency.into(),
            amount: None,
            star_amount: None,
            suggested_post_message: None,
        }
    }

    /// Sets a new amount.
    ///
    /// # Arguments
    ///
    /// * `value` - The amount of the currency that was received by the channel in nanotoncoins.
    pub fn with_amount(mut self, value: Integer) -> Self {
        self.amount = Some(value);
        self
    }

    /// Sets a new star amount.
    ///
    /// # Arguments
    ///
    /// * `value` - The amount of Telegram Stars that was received by the channel.
    pub fn with_star_amount<T>(mut self, value: T) -> Self
    where
        T: Into<StarAmount>,
    {
        self.star_amount = Some(value.into());
        self
    }

    /// Sets a new suggested post message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message containing the suggested post.
    pub fn with_suggested_post_message(mut self, value: Message) -> Self {
        self.suggested_post_message = Some(value);
        self
    }
}

/// Contains parameters of a post that is being suggested by the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SuggestedPostParameters {
    /// Proposed price for the post.
    ///
    /// If the field is omitted, then the post is unpaid.
    pub price: Option<SuggestedPostPrice>,
    /// Proposed send date of the post.
    ///
    /// If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future.
    ///
    /// If the field is omitted, then the post can be published at any time within 30 days
    /// at the sole discretion of the user who approves it.
    pub send_date: Option<Integer>,
}

impl SuggestedPostParameters {
    /// Sets a new price.
    ///
    /// # Arguments
    ///
    /// * `value` - Proposed price for the post.
    pub fn with_price(mut self, value: SuggestedPostPrice) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets a new send date.
    ///
    /// # Arguments
    ///
    /// * `value` - Proposed send date of the post.
    pub fn with_send_date(mut self, value: Integer) -> Self {
        self.send_date = Some(value);
        self
    }
}

/// Desribes price of a suggested post.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SuggestedPostPrice {
    /// The amount of the currency that will be paid for the post in the smallest units of the currency
    /// i.e. Telegram Stars or nanotoncoins.
    ///
    /// Currently, price in Telegram Stars must be between 5 and 100000,
    /// and price in nanotoncoins must be between 10000000 and 10000000000000.
    pub amount: Integer,
    /// Currency in which the post will be paid.
    ///
    /// Currently, must be one of “XTR” for Telegram Stars or “TON” for toncoins.
    pub currency: String,
}

impl SuggestedPostPrice {
    /// Creates a new `SuggestedPostPrice`.
    ///
    /// # Arguments
    ///
    /// * `amount` -  The amount of the currency that will be paid for the post in the smallest units of the currency.
    /// * `currency` - Currency in which the post will be paid.
    pub fn new<T>(amount: Integer, currency: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            amount,
            currency: currency.into(),
        }
    }
}

/// Describes a service message about a payment refund for a suggested post.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SuggestedPostRefunded {
    /// Reason for the refund.
    pub reason: SuggestedPostRefundReason,
    /// Message containing the suggested post.
    ///
    /// Note that the Message object in this field will not contain the reply_to_message field
    /// even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
}

impl SuggestedPostRefunded {
    /// Creates a new `SuggestedPostRefunded`.
    ///
    /// # Arguments
    ///
    /// * `reason` - Reason for the refund.
    pub fn new(reason: SuggestedPostRefundReason) -> Self {
        Self {
            reason,
            suggested_post_message: None,
        }
    }

    /// Sets a new suggested post message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message containing the suggested post.
    pub fn with_suggested_post_message(mut self, value: Message) -> Self {
        self.suggested_post_message = Some(value);
        self
    }
}

/// Reason for the refund.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestedPostRefundReason {
    /// The post was deleted within 24 hours of being posted
    /// or removed from scheduled messages without being posted.
    PostDeleted,
    /// The payer refunded their payment.
    PaymentRefunded,
}

/// State of a suggested post.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestedPostState {
    /// The post is approved.
    Approved,
    /// The post is declined.
    Declined,
    /// The post is waiting for approval.
    Pending,
}

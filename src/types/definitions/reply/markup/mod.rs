use serde::{Deserialize, Serialize};

pub use self::{force_reply::*, inline_keyboard::*, prepared::*, reply_keyboard::*};

mod force_reply;
mod inline_keyboard;
mod prepared;
mod reply_keyboard;

/// Represents a reply markup.
#[derive(Clone, Debug, derive_more::From, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    /// A force reply
    ForceReply(ForceReply),
    /// An inline keyboard
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    /// A custom keyboard with reply options
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    /// A remove keyboard
    ReplyKeyboardRemove(ReplyKeyboardRemove),
}

impl<const A: usize, const B: usize> From<[[InlineKeyboardButton; B]; A]> for ReplyMarkup {
    fn from(value: [[InlineKeyboardButton; B]; A]) -> Self {
        ReplyMarkup::InlineKeyboardMarkup(value.into())
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for ReplyMarkup {
    fn from(markup: Vec<Vec<InlineKeyboardButton>>) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboardMarkup(markup.into())
    }
}

impl<const A: usize, const B: usize> From<[[KeyboardButton; B]; A]> for ReplyMarkup {
    fn from(value: [[KeyboardButton; B]; A]) -> Self {
        ReplyMarkup::ReplyKeyboardMarkup(value.into())
    }
}

impl From<Vec<Vec<KeyboardButton>>> for ReplyMarkup {
    fn from(markup: Vec<Vec<KeyboardButton>>) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardMarkup(markup.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let obj = ReplyMarkup::from([[InlineKeyboardButton::for_pay("test")]]);
        assert!(matches!(obj, ReplyMarkup::InlineKeyboardMarkup(_)));
        let obj = ReplyMarkup::from(vec![vec![InlineKeyboardButton::for_pay("test")]]);
        assert!(matches!(obj, ReplyMarkup::InlineKeyboardMarkup(_)));
        let obj = ReplyMarkup::from([[KeyboardButton::new("test")]]);
        assert!(matches!(obj, ReplyMarkup::ReplyKeyboardMarkup(_)));
        let obj = ReplyMarkup::from(vec![vec![KeyboardButton::new("test")]]);
        assert!(matches!(obj, ReplyMarkup::ReplyKeyboardMarkup(_)));
    }
}

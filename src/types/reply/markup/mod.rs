use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

pub use self::{force_reply::*, inline_keyboard::*, reply_keyboard::*};

mod force_reply;
mod inline_keyboard;
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

impl ReplyMarkup {
    pub(crate) fn serialize(&self) -> Result<String, ReplyMarkupError> {
        serde_json::to_string(self).map_err(ReplyMarkupError::Serialize)
    }
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

/// Represents an error occurred with reply markup.
#[derive(Debug)]
pub enum ReplyMarkupError {
    /// Can not serialize markup
    Serialize(JsonError),
}

impl Error for ReplyMarkupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReplyMarkupError::Serialize(err) => Some(err),
        }
    }
}

impl fmt::Display for ReplyMarkupError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplyMarkupError::Serialize(err) => write!(out, "can not serialize reply markup: {err}"),
        }
    }
}

use serde::{Deserialize, Serialize};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultType,
};
use crate::types::InlineKeyboardMarkup;

#[cfg(test)]
mod tests;

/// Represents a game.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultGame {
    game_short_name: String,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultGame {
    /// Creates a new `InlineQueryResultGame`.
    ///
    /// # Arguments
    ///
    /// * `game_short_name` - Short name of the game.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    pub fn new<A, B>(game_short_name: A, id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            game_short_name: game_short_name.into(),
            id: id.into(),
            reply_markup: None,
        }
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultGame {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            game_short_name: value.data.game_short_name.ok_or(MissingField("game_short_name"))?,
            id: value.id,
            reply_markup: value.data.reply_markup,
        })
    }
}

impl From<InlineQueryResultGame> for RawInlineQueryResult {
    fn from(value: InlineQueryResultGame) -> Self {
        Self {
            data: RawInlineQueryResultData {
                game_short_name: Some(value.game_short_name),
                reply_markup: value.reply_markup,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Game,
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::types::InlineKeyboardMarkup;

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultKind,
};

#[cfg(test)]
mod tests;

/// Game
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultGame {
    id: String,
    game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultGame {
    /// Creates a new InlineQueryResultGame with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * game_short_name - Short name of the game
    pub fn new<I, N>(id: I, game_short_name: N) -> Self
    where
        I: Into<String>,
        N: Into<String>,
    {
        InlineQueryResultGame {
            id: id.into(),
            game_short_name: game_short_name.into(),
            reply_markup: None,
        }
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultGame {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            game_short_name: value.data.game_short_name.ok_or(MissingField("game_short_name"))?,
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
            kind: RawInlineQueryResultKind::Game,
        }
    }
}

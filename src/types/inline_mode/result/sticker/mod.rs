use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultKind,
};

#[cfg(test)]
mod tests;

/// Link to a sticker stored on the Telegram servers
///
/// By default, this sticker will be sent by the user
/// Alternatively, you can use input_message_content to
/// send a message with the specified content instead of the sticker
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedSticker {
    id: String,
    sticker_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedSticker {
    /// Creates a new InlineQueryResultCachedSticker with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * sticker_file_id - A valid file identifier of the sticker
    pub fn new<I, F>(id: I, sticker_file_id: F) -> Self
    where
        I: Into<String>,
        F: Into<String>,
    {
        InlineQueryResultCachedSticker {
            id: id.into(),
            sticker_file_id: sticker_file_id.into(),
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the photo
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedSticker {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            sticker_file_id: value.data.sticker_file_id.ok_or(MissingField("sticker_file_id"))?,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultCachedSticker> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedSticker) -> Self {
        Self {
            data: RawInlineQueryResultData {
                sticker_file_id: Some(value.sticker_file_id),
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::CachedSticker,
        }
    }
}

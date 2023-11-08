use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultType,
};

#[cfg(test)]
mod tests;

/// Represents a link to a sticker stored on the Telegram servers
///
/// By default, this sticker will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`] to
/// send a message with the specified content instead of the sticker.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedSticker {
    id: String,
    sticker_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultCachedSticker {
    /// Creates a new InlineQueryResultCachedSticker
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier of the result; 1-64 bytes
    /// * sticker_file_id - A valid file identifier of the sticker
    pub fn new<A, B>(id: A, sticker_file_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            sticker_file_id: sticker_file_id.into(),
            input_message_content: None,
            reply_markup: None,
        }
    }

    /// Sets a new input message content
    ///
    /// # Arguments
    ///
    /// * value - Content of the message to be sent instead of the photo
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new reply markup
    ///
    /// # Arguments
    ///
    /// * value - Inline keyboard attached to the message
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedSticker {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            input_message_content: value.data.input_message_content,
            reply_markup: value.data.reply_markup,
            sticker_file_id: value.data.sticker_file_id.ok_or(MissingField("sticker_file_id"))?,
        })
    }
}

impl From<InlineQueryResultCachedSticker> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedSticker) -> Self {
        Self {
            data: RawInlineQueryResultData {
                input_message_content: value.input_message_content,
                reply_markup: value.reply_markup,
                sticker_file_id: Some(value.sticker_file_id),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::CachedSticker,
        }
    }
}

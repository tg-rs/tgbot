use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, Integer};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultType,
};

#[cfg(test)]
mod tests;

/// Represents a link to an article or a web page.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultArticle {
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl InlineQueryResultArticle {
    /// Creates a new `InlineQueryResultArticle`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier; 1-64 Bytes.
    /// * `input_message_content` - Content of the message.
    /// * `title` - Title of the result.
    pub fn new<A, B, C>(id: A, input_message_content: B, title: C) -> Self
    where
        A: Into<String>,
        B: Into<InputMessageContent>,
        C: Into<String>,
    {
        Self {
            id: id.into(),
            input_message_content: input_message_content.into(),
            title: title.into(),
            description: None,
            hide_url: None,
            reply_markup: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
            url: None,
        }
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(value.into());
        self
    }

    /// Sets a new value for a `hide_url` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the URL must to be shown in the message.
    pub fn with_hide_url(mut self, value: bool) -> Self {
        self.hide_url = Some(value);
        self
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

    /// Sets a new thumbnail height.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail height.
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail width.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail width.
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.thumbnail_width = Some(value);
        self
    }

    /// Sets a new thumbnail URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the thumbnail for the result.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.thumbnail_url = Some(value.into());
        self
    }

    /// Sets a new URL.
    ///
    /// * `value` - URL of the result.
    pub fn with_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.url = Some(value.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultArticle {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            description: value.data.description,
            hide_url: value.data.hide_url,
            id: value.id,
            input_message_content: value.data.input_message_content.ok_or(MissingField("content"))?,
            reply_markup: value.data.reply_markup,
            thumbnail_url: value.data.thumbnail_url,
            thumbnail_width: value.data.thumbnail_width,
            thumbnail_height: value.data.thumbnail_height,
            title: value.data.title.ok_or(MissingField("title"))?,
            url: value.data.url,
        })
    }
}

impl From<InlineQueryResultArticle> for RawInlineQueryResult {
    fn from(value: InlineQueryResultArticle) -> Self {
        Self {
            data: RawInlineQueryResultData {
                description: value.description,
                hide_url: value.hide_url,
                input_message_content: Some(value.input_message_content),
                reply_markup: value.reply_markup,
                thumbnail_url: value.thumbnail_url,
                thumbnail_width: value.thumbnail_width,
                thumbnail_height: value.thumbnail_height,
                title: Some(value.title),
                url: value.url,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Article,
        }
    }
}

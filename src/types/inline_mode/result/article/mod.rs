use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, Integer};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultKind,
};

#[cfg(test)]
mod tests;

/// Link to an article or web page
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultArticle {
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_height: Option<Integer>,
}

impl InlineQueryResultArticle {
    /// Creates a new InlineQueryResultArticle with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 Bytes
    /// * title - Title of the result
    /// * input_message_content - Content of the message to be sent
    pub fn new<I, T, C>(id: I, title: T, input_message_content: C) -> Self
    where
        I: Into<String>,
        T: Into<String>,
        C: Into<InputMessageContent>,
    {
        InlineQueryResultArticle {
            id: id.into(),
            title: title.into(),
            input_message_content: input_message_content.into(),
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// URL of the result
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Pass True, if you don't want the URL to be shown in the message
    pub fn hide_url(mut self, hide_url: bool) -> Self {
        self.hide_url = Some(hide_url);
        self
    }

    /// Short description of the result
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Url of the thumbnail for the result
    pub fn thumbnail_url<S: Into<String>>(mut self, thumbnail_url: S) -> Self {
        self.thumbnail_url = Some(thumbnail_url.into());
        self
    }

    /// Thumbnail width
    pub fn thumbnail_width(mut self, thumbnail_width: Integer) -> Self {
        self.thumbnail_width = Some(thumbnail_width);
        self
    }

    /// Thumbnail height
    pub fn thumbnail_height(mut self, thumbnail_height: Integer) -> Self {
        self.thumbnail_height = Some(thumbnail_height);
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultArticle {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            title: value.data.title.ok_or(MissingField("title"))?,
            input_message_content: value.data.input_message_content.ok_or(MissingField("content"))?,
            reply_markup: value.data.reply_markup,
            url: value.data.url,
            hide_url: value.data.hide_url,
            description: value.data.description,
            thumbnail_url: value.data.thumbnail_url,
            thumbnail_width: value.data.thumbnail_width,
            thumbnail_height: value.data.thumbnail_height,
        })
    }
}

impl From<InlineQueryResultArticle> for RawInlineQueryResult {
    fn from(value: InlineQueryResultArticle) -> Self {
        Self {
            data: RawInlineQueryResultData {
                title: Some(value.title),
                input_message_content: Some(value.input_message_content),
                reply_markup: value.reply_markup,
                url: value.url,
                hide_url: value.hide_url,
                description: value.description,
                thumbnail_url: value.thumbnail_url,
                thumbnail_width: value.thumbnail_width,
                thumbnail_height: value.thumbnail_height,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Article,
        }
    }
}
